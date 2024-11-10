#![allow(warnings)]
use std::io;

use config::Config;
use ratatui::{
    buffer::Buffer, crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, layout::{Alignment, Constraint, Direction, Layout, Position, Rect}, style::{Color, Modifier, Style, Stylize}, symbols, text::{Line, Span, Text}, widgets::{Block, Borders, HighlightSpacing, List, ListDirection, ListItem, ListState, Paragraph, StatefulWidget, Widget}, DefaultTerminal, Frame
};

use tui_big_text::{BigText, PixelSize};

mod config;
mod components;
mod network;

use components::{Input, InputMode, Menu, Offset};
use network::CustomClient;
//use reqwest::Result;

enum Step {
    Username,
    Password
}

#[derive(PartialEq)]
enum Screen {
    Home,
    Credentials,

    Exit
}

struct App {

    // Paramètres généraux
    config: Config,
    client: CustomClient,

    // Paramètres de l'application
    screen: Screen,
    username:  Option<String>,
    password: Option<String>,
    connected: bool,

    // Paramètre de l'entrée des identifiants
    step: Step,
    username_component: Input,
    password_component: Input,

    // Paramètre de l'écran d'accueil
    menu: Menu,

    value: String,
}

impl App {
    fn new() -> Self {
        let config = Config::init();
        config.save();

        let client = CustomClient::new(&config);

        Self {
            config,
            client,

            screen: Screen::Home,
            username: None,
            password: None,
            connected: false,

            step: Step::Username,
            username_component: Input::new("Identifiant", true),
            password_component: Input::new("Mot de passe", false),

            menu: Menu::new(
                "Actions",
                vec![
                    "Se connecter".to_string(),
                    "Quitter".to_string()
                ]),

            value: String::new(),
        }
    }

    fn draw_credentials(&mut self, frame:&mut Frame, area: Rect) {
        let [username_area, password_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
        ]).areas(area);

        self.username_component.draw(frame, username_area);
        self.password_component.draw(frame, password_area);
    }


    fn draw_home(&mut self, frame: &mut Frame, area: Rect) {
        let [help_area, list_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(3)
        ]).areas(area);

        let text = Text::from(Line::from("Que souhaitez vous faire ?")).patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));
        let help_message = Paragraph::new(text);

        frame.render_widget(&mut self.menu, list_area);
    }

    fn handle_events(&mut self, key: KeyEvent) {
        if (key.kind == KeyEventKind::Press && key.code == KeyCode::Esc) {
            self.screen = Screen::Exit;
            return;
        }

        match self.screen {
            Screen::Home => {
                if (key.kind == KeyEventKind::Press) {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.screen = Screen::Exit,
                        KeyCode::Enter => {
                            if let Some(index) = self.menu.state.selected() {
                                if index == 0 {
                                    self.screen = Screen::Credentials;
                                } else {
                                    self.screen = Screen::Exit;
                                }
                            }
                        },
                        _ => {
                            self.menu.handle_key(key);
                        }
                    }
                }
            },
            Screen::Credentials => {
                if (key.kind == KeyEventKind::Press) {
                    let input = match self.step {
                        Step::Username => &mut self.username_component,
                        Step::Password => &mut self.password_component
                    };

                    let next = input.handle_key(key.code);

                    if (next) {
                        match self.step {
                            Step::Username => {
                                self.password_component.mode = InputMode::Editing;
                                self.step = Step::Password;
                            },
                            Step::Password => {
                                self.username = Some(self.username_component.value.clone());
                                self.password = Some(self.password_component.value.clone());

                                self.screen = Screen::Home;
                            }
                        };
                    }
                }
            },
            Screen::Exit => {}
        }
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let widget = BigText::builder()
                .centered()
                .pixel_size(PixelSize::Quadrant)
                .style(Style::new().bg(Color::Red))
                .lines(vec![
                    "Internet - Intranet".white().into(),
                ])
                .build();

        frame.render_widget(widget, area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        match self.screen {
            Screen::Home => {
                let widget = Paragraph::new(" Utilisez les flèches ↓↑ pour naviguer dans le menu, Entrée pour valider. ")
                    .centered();


                frame.render_widget(widget, area);
            },
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

                let widget = Paragraph::new(text)
                    .centered();

                frame.render_widget(widget, area);
            },
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame) {

        let chunks = Layout::horizontal([
                Constraint::Fill(1),
                Constraint::Max(80),
                Constraint::Fill(1)
            ])
            .split(frame.area());

        let [_, header_area, _, screen_area, footer_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Min(8),
            Constraint::Length(1),
        ])
        .areas(chunks[1]);

        let [_, inner_screen_area, _] = Layout::horizontal([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2)
        ]).areas(screen_area);

        self.render_header(frame, header_area);
        self.render_footer(frame, footer_area);

        match self.screen {
            Screen::Home=> {
                self.draw_home(frame, inner_screen_area);
            },
            Screen::Credentials => {
                self.draw_credentials(frame, inner_screen_area);
            },
            _ => {}
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
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
                            Constraint::Fill(1)
                        ])
                        .split(frame.area());

                frame.render_widget(outer_block, chunks[1]);

                self.render(frame)
            })?;

            //terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;

            if let Event::Key(key) = event::read()? {
                self.handle_events(key);
            }

            if (self.screen == Screen::Exit) {
                return Ok(());
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
