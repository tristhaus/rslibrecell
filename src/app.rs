use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

/// The state of the app.
#[derive(Debug, PartialEq)]
enum AppState {
    /// The base behavior, meaning a game can be actively played.
    Base,
    /// The app is about to exit.
    Exit,
}

/// The actual app.
#[derive(Debug)]
pub struct App {
    app_state: AppState,
}

impl App {
    pub fn new() -> App {
        App {
            app_state: AppState::Base,
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
        match key_event.code {
            KeyCode::Char('q') if key_event.modifiers.contains(KeyModifiers::CONTROL) => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        match self.app_state {
            _ => self.app_state = AppState::Exit,
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" RSLibreCell ".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<CTRL-q> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let mut lines: Vec<Line> = vec![];

        let title_line = String::from("RustLibreCell              ");

        lines.push(Line::from(title_line));

        let board_text = Text::from(lines);

        Paragraph::new(board_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod test;
