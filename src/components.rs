use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, HighlightSpacing, List, ListDirection, ListItem, ListState, Paragraph,
        StatefulWidget, Widget,
    },
    DefaultTerminal, Frame,
};

// Composant d'entrée de texte
pub enum InputMode {
    Normal,
    Editing,
}

pub enum Offset {
    Left,
    Right,
}

pub struct Input {
    pub label: String,
    pub value: String,   // Valeur
    pub mode: InputMode, // Mode
    index: usize,        // Index du curseur
}

impl Input {
    pub fn new(label: &str, focus: bool) -> Self {
        Self {
            label: String::from(label),
            value: String::new(),
            mode: match focus {
                true => InputMode::Editing,
                false => InputMode::Normal,
            },
            index: 0,
        }
    }

    fn move_cursor(&mut self, offset: Offset) {
        let cursor_move = match offset {
            Offset::Left => self.index.saturating_sub(1),
            Offset::Right => self.index.saturating_add(1),
        };
        self.index = cursor_move.clamp(0, self.value.chars().count())
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.value.insert(index, new_char);
        self.move_cursor(Offset::Right);
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.index;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.value.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.value.chars().skip(current_index);

            self.value = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor(Offset::Left);
        }
    }

    fn byte_index(&self) -> usize {
        self.value
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.index)
            .unwrap_or(self.value.len())
    }

    pub fn reset_cursor(&mut self) {
        self.index = 0;
    }

    pub fn submit_input(&mut self) {
        self.value.clear();
        self.reset_cursor();
    }

    pub fn handle_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Char(to_insert) => self.enter_char(to_insert),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Left => self.move_cursor(Offset::Left),
            KeyCode::Right => self.move_cursor(Offset::Right),

            KeyCode::Enter => {
                self.mode = InputMode::Normal;
                return true;
            }
            _ => {}
        }
        return false;
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let input = Paragraph::new(self.value.as_str())
            .style(match self.mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title(self.label.as_str()));

        frame.render_widget(input, area);

        match self.mode {
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            InputMode::Normal => {}
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Editing => frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                area.x + self.index as u16 + 1,
                // Move one line down, from the border to the input line
                area.y + 1,
            )),
        }
    }
}

// Menu
pub struct Menu {
    title: String,
    options: Vec<String>,
    pub state: ListState,
}

impl Menu {
    pub fn new(title: &str, options: Vec<String>) -> Self {
        let mut state = ListState::default();

        state.select(Some(0));

        return Self {
            options,
            state,
            title: title.to_string(),
        };
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('h') | KeyCode::Left => self.state.select(None),
            KeyCode::Char('j') | KeyCode::Down => self.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.state.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.state.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.state.select_last(),
            _ => {}
        }
    }
}

impl Widget for &mut Menu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Actions").centered().italic().bold())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY);

        let list = List::new(
            self.options
                .iter()
                .map(|item| ListItem::new(Line::from(Span::raw(format!("{item}"))))),
        )
        .block(block)
        .style(Style::new().white())
        .highlight_style(Style::new().bold().yellow())
        .highlight_symbol(">> ")
        .highlight_spacing(HighlightSpacing::Always)
        .direction(ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
