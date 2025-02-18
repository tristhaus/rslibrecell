use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Clear, Paragraph, Widget, Wrap},
    DefaultTerminal, Frame,
};
use rslibrecell::{
    card::{Card, Suit},
    game_handler::GameHandler,
    r#move::{Location, Move},
};

/// The state of the app.
#[derive(Debug, PartialEq)]
enum AppState {
    /// The base behavior, meaning a game can be actively played.
    Base,
    /// The app is about to exit.
    Exit,
    /// The app is displaying the modal help dialog.
    HelpModal,
}

/// The actual app.
#[derive(Debug)]
pub struct App {
    app_state: AppState,
    game_handler: GameHandler,
    move_from: Option<Location>,
}

impl App {
    pub fn new() -> App {
        App {
            app_state: AppState::Base,
            game_handler: GameHandler::default(),
            move_from: None,
        }
    }

    /// Runs the application's main loop until the user quits.
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            match self.app_state {
                AppState::Exit => break,
                _ => {
                    terminal.draw(|frame| self.draw(frame))?;
                    self.handle_events()?;
                }
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// Updates the application's state based on user input.
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    /// Handles any key events.
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.app_state {
            AppState::Base => self.handle_key_event_base(key_event),
            AppState::Exit => panic!("should never happen"),
            AppState::HelpModal => self.handle_key_event_help_modal(key_event),
        };
    }

    fn handle_key_event_base(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.exit()
            }
            KeyCode::F(1) => self.help_modal(),
            KeyCode::F(2) => self.random_game(),
            _ => self.handle_key_event_game(key_event),
        }
    }

    fn handle_key_event_game(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.register_partial_move(Location::Cell { i: 0 }),
            KeyCode::Char('w') => self.register_partial_move(Location::Cell { i: 1 }),
            KeyCode::Char('e') => self.register_partial_move(Location::Cell { i: 2 }),
            KeyCode::Char('r') => self.register_partial_move(Location::Cell { i: 3 }),
            KeyCode::Char('a') => self.register_partial_move(Location::Column { i: 0 }),
            KeyCode::Char('s') => self.register_partial_move(Location::Column { i: 1 }),
            KeyCode::Char('d') => self.register_partial_move(Location::Column { i: 2 }),
            KeyCode::Char('f') => self.register_partial_move(Location::Column { i: 3 }),
            KeyCode::Char('j') => self.register_partial_move(Location::Column { i: 4 }),
            KeyCode::Char('k') => self.register_partial_move(Location::Column { i: 5 }),
            KeyCode::Char('l') => self.register_partial_move(Location::Column { i: 6 }),
            KeyCode::Char('ö') => self.register_partial_move(Location::Column { i: 7 }),
            KeyCode::Char('u') | KeyCode::Char('i') | KeyCode::Char('o') | KeyCode::Char('p') => {
                self.register_partial_move(Location::Foundation)
            }
            KeyCode::Char(' ') => self.clear_move(),
            KeyCode::Char('R') => self.revert(),
            _ => {}
        }
    }

    fn handle_key_event_help_modal(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.exit()
            }
            KeyCode::Esc => {
                self.base();
            }
            _ => {}
        }
    }

    fn random_game(&mut self) {
        self.game_handler.random_game();
    }

    fn base(&mut self) {
        self.app_state = AppState::Base;
    }

    fn exit(&mut self) {
        self.app_state = AppState::Exit;
    }

    fn help_modal(&mut self) {
        self.app_state = AppState::HelpModal;
    }

    #[cfg(test)]
    fn game_from_u16_id(&mut self, id: u16) {
        self.game_handler.game_from_id(id);
    }

    fn register_partial_move(&mut self, location: Location) {
        match &self.move_from {
            Some(first) => {
                let _ = self.game_handler.make_move(Move {
                    from: first.clone(),
                    to: location,
                });
                self.move_from = None;
            }
            None => {
                self.move_from = Some(location);
            }
        }
    }

    fn clear_move(&mut self) {
        self.move_from = None;
    }

    fn revert(&mut self) {
        self.move_from = None;
        let _ = self.game_handler.revert();
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" RSLibreCell ".bold());
        let instructions = Line::from(vec![
            " Help ".into(),
            "<F1>".blue().bold(),
            " Quit ".into(),
            "<CTRL-q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let mut lines: Vec<Line> = vec![];

        if let Some(game) = self.game_handler.game.as_ref() {
            render::render_game(&mut lines, game);
        }

        let board_text = Text::from(lines);

        Paragraph::new(board_text)
            .centered()
            .block(block)
            .render(area, buf);

        if let AppState::HelpModal = self.app_state {
            render::render_help_modal(area, buf);
        }
    }
}

mod render {
    use super::*;

    pub fn render_game<'a>(lines: &mut Vec<Line<'a>>, game: &'a rslibrecell::game::Game) {
        let mut title_line = String::from("                           ");
        let id = &game.id.to_string();
        for _ in 0..(5 - id.len()) {
            title_line += " ";
        }
        title_line += "#";
        title_line += &id;
        title_line += " ";

        lines.push(Line::from(title_line));

        let mut cells_foundations_span: Vec<Span> = vec![];
        for cell in &game.cells {
            match cell {
                Some(card) => {
                    cells_foundations_span.push(get_colored_representation(card));
                }
                None => {
                    cells_foundations_span.push(" .. ".into());
                }
            }
        }

        cells_foundations_span.push("||".into());

        for foundation in &game.foundations {
            match foundation.last() {
                Some(card) => {
                    cells_foundations_span.push(get_colored_representation(card));
                }
                None => {
                    cells_foundations_span.push(" .. ".into());
                }
            }
        }

        lines.push(Line::from(cells_foundations_span));
        lines.push(Line::from("--------------------------------- "));

        let mut column_spans: Vec<Vec<Span>> = vec![vec![Span::from(" ")]; 19];

        for i in 0..19 as usize {
            for column in &game.columns {
                let card = column.get(i);
                match card {
                    Some(card) => {
                        column_spans[i].push(get_colored_representation(card));
                    }
                    None => {
                        column_spans[i].push("    ".into());
                    }
                }
            }

            column_spans[i].push(" ".into());
        }

        let mut column_lines: Vec<Line> = vec![];

        for spans in column_spans {
            column_lines.push(Line::from(spans));
        }

        lines.append(&mut column_lines);
    }

    pub fn render_help_modal(area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Help ");
        let instructions = Line::from(vec![" Close ".into(), "<Esc> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered());

        let mut help_lines: Vec<Line> = vec![];
        help_lines.push(Line::from(vec![
            "<F2>".blue(),
            " to start a new random game.".into(),
        ]));
        help_lines.push(Line::from("\n"));
        help_lines.push(Line::from(vec![
            "<q> <w> <e> <r>".blue(),
            " - cells ".into(),
        ]));
        help_lines.push(Line::from(vec![
            "<u> <i> <o> <p>".blue(),
            " - foundations ".into(),
        ]));
        help_lines.push(Line::from(vec![
            "<a> <s> <d> <f>".blue(),
            " - left columns ".into(),
        ]));
        help_lines.push(Line::from(vec![
            "<j> <k> <l> <ö>".blue(),
            " - right columns ".into(),
        ]));
        help_lines.push(Line::from("\n"));
        help_lines.push(Line::from(vec![
            "Make a move by choosing the start and end of a move. ".into(),
            "<Space>".blue(),
            " to abort a move. ".into(),
            "<R>".blue(),
            " to revert the last move.".into(),
        ]));

        let help_text = Text::from(help_lines);

        let area = popup_area(area);
        Clear::default().render(area, buf);

        block.render(area, buf);

        let inner_area = area.inner(Margin {
            horizontal: 2,
            vertical: 1,
        });

        Paragraph::new(help_text)
            .wrap(Wrap { trim: true })
            .render(inner_area, buf);
    }

    /// helper function to create a centered rect using up certain percentage of the available rect `r`
    fn popup_area(area: Rect) -> Rect {
        Rect {
            x: area.x + 2,
            y: area.y + 1,
            width: area.width - 4,
            height: area.height - 2,
        }
    }

    fn get_colored_representation(card: &Card) -> Span<'_> {
        let unstyled_span = <Span<'_>>::from(format!(" {c} ", c = card.to_string()));
        match card.suit {
            Suit::Clubs | Suit::Spades => return unstyled_span.into(),
            Suit::Diamonds | Suit::Hearts => return unstyled_span.red(),
        }
    }
}

#[cfg(test)]
mod test;
