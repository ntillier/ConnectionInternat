#![allow(warnings)]
use chrono::{Date, DateTime, Local, Utc};
use core::panic;
use std::borrow::Borrow;
use std::fmt::format;
use std::io::{self, BufReader, Read};
use std::io::{BufRead, Write};
use std::process::{Command, Stdio};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use wait_timeout::ChildExt;

use config::Config;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, HighlightSpacing, List, ListDirection, ListItem, ListState, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
    DefaultTerminal, Frame,
};

use tui_big_text::{BigText, PixelSize};

mod components;
mod config;

use components::{Input, InputMode, Menu, Offset};
//use reqwest::Result;

enum Step {
    Username,
    Password,
}

#[derive(PartialEq)]
enum Screen {
    Home,
    Credentials,
    Status,
    Disconnect,

    Exit,
}

#[derive(PartialEq)]
enum ConnectionStatus {
    Uninitialized,
    Connected,
    Disconnected,
    Connecting,
}

struct App {
    // Paramètres généraux
    config: Config,

    // Paramètres de l'application
    screen: Screen,
    username: Option<String>,
    password: Option<String>,
    passwordDigest: Option<String>,
    connectionStatus: ConnectionStatus,
    lastLogin: Option<String>,
    lastPingAttempt: Option<DateTime<Local>>,
    lastPingTimestamp: Option<DateTime<Local>>,
    backendPath: String,
    lastError: Option<String>,

    // Paramètre de l'entrée des identifiants
    step: Step,
    username_component: Input,
    password_component: Input,

    // Paramètre de l'écran d'accueil
    menu: Menu,
    // first element is the last connection status where this was updated: if it's different from current status, it probably needs to be changed
    status_menu: Menu,

    value: String,
}

const TICK_RATE: u64 = 1000;
const PING_INTERVAL: i64 = 50;
const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
const BACKEND_FILE_NAME: &str = "ConnectionInternat-backend.exe";

fn getBackendPath() -> String {
    // if env variable BACKEND_PATH is set, use it
    if let Ok(path) = std::env::var("BACKEND_PATH") {
        return path;
    }
    let mut path = std::env::current_exe().unwrap();
    if !path.pop() {
        panic!("Failed to pop path to backend");
    }
    path.push(BACKEND_FILE_NAME);
    path.to_str().unwrap().to_string()
}

impl App {
    fn new() -> Self {
        let config = Config::init();
        config.save();

        let home_menu = Menu::new(
            "Actions",
            if config.password != "" && config.username != "" {
                vec![
                    format!("Se connecter (en tant que {})", config.username).to_string(),
                    "Rentrer ses identifiants".to_string(),
                    "Oublier les identifiants sauvegardés".to_string(),
                    "Quitter".to_string(),
                ]
            } else {
                vec![
                    "Rentrer ses identifiants".to_string(),
                    "Quitter".to_string(),
                ]
            },
        );

        Self {
            config,

            screen: Screen::Home,
            username: None,
            password: None,
            passwordDigest: None,
            connectionStatus: ConnectionStatus::Uninitialized,
            lastLogin: None,
            lastPingAttempt: None,
            lastPingTimestamp: None,
            backendPath: getBackendPath(),
            lastError: None,

            step: Step::Username,
            username_component: Input::new("Identifiant", true),
            password_component: Input::new("Mot de passe", false),

            menu: home_menu,
            status_menu: Menu::new("Actions", vec!["Se déconnecter".to_string()]),

            value: String::new(),
        }
    }

    fn on_tick(&mut self) {
        if !matches!(self.connectionStatus, ConnectionStatus::Uninitialized)
            && !self.passwordDigest.is_none()
        {
            let seconds = seconds_since(self.lastPingAttempt).unwrap_or(0);
            if seconds >= PING_INTERVAL {
                self.ping()
            }
        }
    }

    fn draw_credentials(&mut self, frame: &mut Frame, area: Rect) {
        let [username_area, password_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(3)]).areas(area);

        self.username_component.draw(frame, username_area);
        self.password_component.draw(frame, password_area);
    }

    fn draw_home(&mut self, frame: &mut Frame, area: Rect) {
        let [help_area, list_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(3)]).areas(area);

        let text = Text::from(Line::from("Que souhaitez vous faire ?"))
            .patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));
        let help_message = Paragraph::new(text);

        frame.render_widget(&mut self.menu, list_area);
    }

    fn draw_status(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            // .margin(1)
            .constraints([Constraint::Min(0), Constraint::Min(0), Constraint::Min(0)])
            .split(area);
        let (status_area, menu_area) = (chunks[0], chunks[1]);

        let last_ping = match &self.lastPingTimestamp {
            Some(date) => {
                if !matches!(self.connectionStatus, ConnectionStatus::Connected)
                    && !self.lastPingAttempt.is_none()
                {
                    format!(
                        "{} (il y a {}s) (essai il y a {}s)",
                        date.format(DATE_FORMAT).to_string(),
                        seconds_since(self.lastPingTimestamp).unwrap_or(0),
                        seconds_since(self.lastPingAttempt).unwrap()
                    )
                } else {
                    format!(
                        "{} (il y a {} secondes)",
                        date.format(DATE_FORMAT).to_string(),
                        seconds_since(self.lastPingTimestamp).unwrap_or(0)
                    )
                }
            }
            None => "N/A".to_string(),
        };

        let status = match self.connectionStatus {
            ConnectionStatus::Uninitialized => "Non initialise".green(),
            ConnectionStatus::Connected => "Connecté".green(),
            ConnectionStatus::Disconnected => "Déconnecté".red(),
            ConnectionStatus::Connecting => "Connexion en cours...".yellow(),
        };

        let error: String = match &self.lastError {
            Some(error) => format!("Error: {}", error),
            None => "".to_string(),
        };

        let mut lines = vec![
            Line::from(format!("Statut: {}", status)),
            Line::from(format!(
                "Dernier login: {}",
                self.lastLogin.as_ref().unwrap_or(&"N/A".to_string())
            )),
            Line::from(format!("Dernier ping: {}", last_ping)),
        ];

        if !error.is_empty() {
            for line in error.lines() {
                lines.push(Line::styled(line, Style::default().fg(Color::Red)));
            }
        }

        let status_text = Text::from(lines);
        let status_paragraph = Paragraph::new(status_text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(status_paragraph, status_area);

        // Render menu
        let text = Text::from(Line::from("Que souhaitez vous faire ?"))
            .patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));
        let help_message = Paragraph::new(text);

        frame.render_widget(&mut self.status_menu, menu_area);
    }

    fn refresh_status_menu(&mut self) {
        match self.connectionStatus {
            ConnectionStatus::Connected => {
                self.status_menu = Menu::new("Actions", vec!["Se déconnecter".to_string()]);
            }
            ConnectionStatus::Disconnected => {
                self.status_menu = Menu::new(
                    "Actions",
                    vec![
                        "Essayer de se reconnecter".to_string(),
                        "Se déconnecter".to_string(),
                    ],
                );
            }
            ConnectionStatus::Connecting => {
                self.status_menu = Menu::new("Actions", vec![]);
            }
            ConnectionStatus::Uninitialized => {}
        }
    }

    fn draw_disconnect(&mut self, frame: &mut Frame, area: Rect) {
        let text = Text::from(Line::from("Déconnexion en cours...").bold());
        let widget = Paragraph::new(text).alignment(Alignment::Center);

        frame.render_widget(widget, area);
    }

    fn handle_key_events(&mut self, key: KeyEvent) {
        if (key.kind == KeyEventKind::Press && key.code == KeyCode::Esc) {
            self.screen = Screen::Exit;
            return;
        }

        match self.screen {
            Screen::Home => {
                if (key.kind == KeyEventKind::Press) {
                    match key.code {
                        KeyCode::Char('q') => self.screen = Screen::Exit,
                        KeyCode::Enter => {
                            if let Some(index) = self.menu.state.selected() {
                                if self.config.username != "" && self.config.password != "" {
                                    if index == 0 {
                                        self.username = Some(self.config.username.clone());
                                        self.password = Some(self.config.password.clone());
                                        self.screen = Screen::Status;

                                        self.login();
                                    } else if index == 1 {
                                        self.screen = Screen::Credentials;
                                    } else if index == 2 {
                                        // clear old credentials
                                        self.config.username = "".to_string();
                                        self.config.password = "".to_string();
                                        self.config.save();

                                        self.menu = Menu::new(
                                            "Actions",
                                            vec![
                                                "Rentrer ses identifiants".to_string(),
                                                "Quitter".to_string(),
                                            ],
                                        );
                                    } else {
                                        self.screen = Screen::Exit;
                                    }
                                } else {
                                    if index == 0 {
                                        self.screen = Screen::Credentials;
                                    } else {
                                        self.screen = Screen::Exit;
                                    }
                                }
                            }
                        }
                        _ => {
                            self.menu.handle_key(key);
                        }
                    }
                }
            }
            Screen::Credentials => {
                if (key.kind == KeyEventKind::Press) {
                    let input = match self.step {
                        Step::Username => &mut self.username_component,
                        Step::Password => &mut self.password_component,
                    };

                    let next = input.handle_key(key.code);

                    if (next) {
                        match self.step {
                            Step::Username => {
                                self.password_component.mode = InputMode::Editing;
                                self.step = Step::Password;
                            }
                            Step::Password => {
                                self.username = Some(self.username_component.value.clone());
                                self.password = Some(self.password_component.value.clone());

                                self.screen = Screen::Status;
                                self.login();
                            }
                        };
                    }
                }
            }
            Screen::Status => {
                if (key.kind == KeyEventKind::Press) {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            self.screen = match self.connectionStatus {
                                ConnectionStatus::Connected => Screen::Disconnect,
                                _ => Screen::Exit,
                            };
                            self.disconnect();
                        }
                        KeyCode::Enter => {
                            if let Some(index) = self.status_menu.state.selected() {
                                if matches!(self.connectionStatus, ConnectionStatus::Connected) {
                                    self.screen = Screen::Disconnect;
                                    self.disconnect();
                                } else {
                                    if index == 0 {
                                        // reconnect
                                        self.reconnect();
                                    } else {
                                        // disconnect
                                        self.screen = Screen::Disconnect;
                                        self.disconnect();
                                    }
                                }
                            }
                        }
                        _ => {
                            self.status_menu.handle_key(key);
                        }
                    }
                }
            }
            Screen::Disconnect => {
                if (key.kind == KeyEventKind::Press) {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.screen = Screen::Exit,
                        _ => {}
                    }
                }
            }
            Screen::Exit => {}
        }
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let widget = BigText::builder()
            .centered()
            .pixel_size(PixelSize::Quadrant)
            .style(Style::new().bg(Color::Red))
            .lines(vec!["Internet - Intranet".white().into()])
            .build();

        frame.render_widget(widget, area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        match self.screen {
            Screen::Home => {
                let widget = Paragraph::new(
                    " Utilisez les flèches ↓↑ pour naviguer dans le menu, Entrée pour valider. ",
                )
                .centered();

                frame.render_widget(widget, area);
            }
            Screen::Credentials => {
                let (msg, style) = match self.step {
                    Step::Username => (
                        vec![
                            " Indiquez votre identifiant. Pressez ".into(),
                            "Entrée".bold(),
                            " pour passer à l'étape suivante ".into(),
                        ],
                        Style::default().add_modifier(Modifier::RAPID_BLINK),
                    ),
                    Step::Password => (
                        vec![
                            " Entrez votre mot de passe. Pressez ".into(),
                            "Entrée".bold(),
                            " pour passer à l'étape suivante ".into(),
                        ],
                        Style::default(),
                    ),
                };

                let text = Text::from(Line::from(msg)).patch_style(style);

                let widget = Paragraph::new(text).centered();

                frame.render_widget(widget, area);
            }
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Max(80),
            Constraint::Fill(1),
        ])
        .split(frame.area());

        let [_, header_area, screen_area, footer_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(4),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(chunks[1]);

        let [_, inner_screen_area, _] = Layout::horizontal([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
        ])
        .areas(screen_area);

        self.render_header(frame, header_area);
        self.render_footer(frame, footer_area);

        match self.screen {
            Screen::Home => {
                self.draw_home(frame, inner_screen_area);
            }
            Screen::Credentials => {
                self.draw_credentials(frame, inner_screen_area);
            }
            Screen::Status => {
                self.draw_status(frame, inner_screen_area);
            }
            Screen::Disconnect => {
                self.draw_disconnect(frame, inner_screen_area);
            }
            Screen::Exit => {}
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        let mut last_tick = Instant::now();
        let tick_rate = std::time::Duration::from_millis(TICK_RATE);

        loop {
            terminal.draw(|frame| {
                let outer_block = Block::default()
                    .title(" HXi² © | Gloire au pingouin ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White));

                // Create the layout to hold content inside the block
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Fill(1),
                        Constraint::Max(80),
                        Constraint::Fill(1),
                    ])
                    .split(frame.area());

                frame.render_widget(outer_block, chunks[1]);

                self.render(frame)
            })?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());

            //terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if event::poll(timeout)? {
                match event::read()? {
                    Event::Key(key) => {
                        self.handle_key_events(key);
                    }
                    _ => {}
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }

            if (self.screen == Screen::Exit) {
                if self.connectionStatus == ConnectionStatus::Connected {
                    self.disconnect();
                }
                return Ok(());
            }
        }
    }

    fn call_backend(&mut self, args: Vec<String>) -> Result<String, String> {
        let count = args.len();
        let input_data = format!("{}\n{}", count, args.join("\n"));

        let mut child = Command::new(self.backendPath.clone())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute child process");

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(input_data.as_bytes())
                .expect("Failed to write to stdin");
        } else {
            return Err("Failed to obtain stdin".to_string());
        }

        let timeout = std::time::Duration::from_secs(20);
        let output_code = match child.wait_timeout(timeout).unwrap() {
            Some(status) => status.code(),
            None => {
                child.kill().unwrap();
                child.wait().unwrap().code()
            }
        };

        let mut stdout = String::new();
        if child.stdout.is_none() {
            return Err("Failed to obtain stdout".to_string());
        }
        child.stdout.unwrap().read_to_string(&mut stdout).unwrap();

        if output_code == Some(0) {
            Ok(stdout)
        } else {
            Err(stdout)
        }
    }

    fn login(&mut self) {
        let username = self.username.as_ref().unwrap();
        let password = self.password.as_ref().unwrap();

        let args = vec!["login".to_string(), username.clone(), password.clone()];

        self.connectionStatus = ConnectionStatus::Connecting;

        self.lastLogin = Some(Local::now().format(DATE_FORMAT).to_string());

        match self.call_backend(args) {
            Ok(output) => {
                self.connectionStatus = ConnectionStatus::Connected;
                self.refresh_status_menu();

                self.lastPingTimestamp = Some(Local::now());
                self.lastPingAttempt = Some(Local::now());
                self.lastError = None;

                // second line is the digest
                self.passwordDigest = output.lines().nth(1).map(|s| s.to_string());

                self.config.username = self.username.clone().unwrap();
                self.config.password = self.password.clone().unwrap();
                self.config.save();

                self.screen = Screen::Status;
            }
            Err(output) => {
                self.connectionStatus = ConnectionStatus::Disconnected;
                self.refresh_status_menu();
                self.lastError = Some(clarify_connection_error(output));
            }
        }
    }

    fn ping(&mut self) {
        self.lastPingAttempt = Some(Local::now());

        let args = vec![
            "ping".to_string(),
            self.username.clone().unwrap_or(String::new()),
            self.passwordDigest.clone().unwrap_or(String::new()),
        ];
        match self.call_backend(args) {
            Ok(_) => {
                self.lastPingTimestamp = Some(Local::now());
                self.lastError = None;
                self.connectionStatus = ConnectionStatus::Connected;
                self.refresh_status_menu();
            }
            Err(output) => {
                self.connectionStatus = ConnectionStatus::Disconnected;
                self.refresh_status_menu();
                self.lastError = Some(clarify_connection_error(output));
            }
        }
    }

    fn disconnect(&mut self) {
        let args = vec![
            "logout".to_string(),
            self.username.clone().unwrap_or(String::new()),
            self.passwordDigest.clone().unwrap_or(String::new()),
        ];
        self.call_backend(args);
        self.connectionStatus = ConnectionStatus::Disconnected;
        self.screen = Screen::Exit;
    }

    fn reconnect(&mut self) {
        self.connectionStatus = ConnectionStatus::Connecting;
        self.passwordDigest = None;

        self.login();
    }
}

fn main() -> io::Result<()> {
    let _ = Command::new("tmux")
        .arg("rename-window")
        .arg("ConnectionInternat")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

fn seconds_since(ts: Option<DateTime<Local>>) -> Option<i64> {
    if ts.is_none() {
        return None;
    }
    let now = Local::now();
    let duration = now.signed_duration_since(ts.unwrap());
    Some(duration.num_seconds())
}

fn clarify_connection_error(err: String) -> String {
    let con = |description: &str| {
        format!(
            "{}\n\nErreur retournée par le serveur: {}",
            description, err
        )
    };
    if err.contains("404") {
        return con(
            "Incapable de trouver le serveur: vérifiez que vous etes connecté au bon réseau",
        );
    } else if err.contains("DNS") {
        return con("Erreur de résolution DNS: vérifiez que vous etes connecté au bon réseau, et que vous n'avez pas de VPN allumé");
    }
    return err;
}
