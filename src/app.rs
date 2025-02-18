use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Clear, Paragraph, Widget, Wrap},
    DefaultTerminal, Frame,
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

    fn base(&mut self) {
        match self.app_state {
            _ => self.app_state = AppState::Base,
        }
    }

    fn exit(&mut self) {
        match self.app_state {
            _ => self.app_state = AppState::Exit,
        }
    }

    fn help_modal(&mut self) {
        match self.app_state {
            _ => self.app_state = AppState::HelpModal,
        }
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

        let title_line = String::from("RustLibreCell              ");

        lines.push(Line::from(title_line));

        let board_text = Text::from(lines);

        Paragraph::new(board_text)
            .centered()
            .block(block)
            .render(area, buf);

        if let AppState::HelpModal = self.app_state {
            let title = Line::from(" Help ");
            let instructions = Line::from(vec![" Close ".into(), "<Esc> ".blue().bold()]);
            let block = Block::bordered()
                .title(title.centered())
                .title_bottom(instructions.centered());

            let mut help_lines: Vec<Line> = vec![];
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
    }
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

#[cfg(test)]
mod test;
