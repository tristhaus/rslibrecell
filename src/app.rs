/*
    RSLibreCell - a FreeCell implementation
    Copyright (C) 2025 and later: tristhaus

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::{cell::RefCell, fmt::Debug, io, rc::Rc, str};

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
    config_repository::KeyConfig,
    game_handler::GameHandler,
    journey_handler::{journey_repository::PersistJourney, JourneyHandler},
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
    /// The app is displaying the modal about dialog.
    /// `scroll` indicates the scroll position.
    AboutModal { scroll: u16 },
    /// The app is displaying the modal "game selection by Id".
    /// `id` serves as a byte buffer for an UTF8 string.
    SelectionIdModal { id: [u8; 5] },
    /// The app is displaying the modal "game selection from journey".
    SelectionJourneyModal,
}

/// The actual app.
#[derive(Debug)]
pub struct App<T>
where
    T: PersistJourney,
    T: Debug,
{
    /// The current state of the app.
    app_state: AppState,
    /// The key config.
    key_config: KeyConfig,
    /// An instance of a game handler.
    game_handler: GameHandler<T>,
    /// An instance of an implementation of `HandleJourney`
    journey_handler: Rc<RefCell<JourneyHandler<T>>>,
    /// The first part of a move as entered by the user, if any.
    move_from: Option<Location>,
}

const SPACE_ASCII_CODE: u8 = 0x20;

impl<T> App<T>
where
    T: PersistJourney,
    T: Debug,
{
    /// Creates and initializes the app.
    pub fn new(key_config: KeyConfig, journey_repository: T) -> App<T> {
        let journey_handler = Rc::new(RefCell::new(JourneyHandler::new(journey_repository)));

        App {
            app_state: AppState::Base,
            key_config,
            game_handler: GameHandler::new(journey_handler.clone()),
            journey_handler: journey_handler.clone(),
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

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// Entry point for the handling of events, such as keyboard user input.
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
            AppState::AboutModal { scroll: _ } => self.handle_key_event_about_modal(key_event),
            AppState::SelectionIdModal { id: _ } => {
                self.handle_key_event_selection_id_modal(key_event)
            }
            AppState::SelectionJourneyModal => {
                self.handle_key_event_selection_journey_modal(key_event)
            }
        };
    }

    /// Handles key events when in base state.
    fn handle_key_event_base(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.exit()
            }
            KeyCode::F(1) => self.help_modal(),
            KeyCode::F(2) => self.random_game(),
            KeyCode::F(3) => self.selection_id_modal(),
            KeyCode::Char('!') => self.selection_journey_modal(),
            KeyCode::F(12) => self.about_modal(),
            KeyCode::Char(char) => self.handle_key_event_game(char),
            _ => {}
        }
    }

    /// Handles keys related to the actual game in the base state.
    fn handle_key_event_game(&mut self, char: char) {
        if char == self.key_config.cell1 {
            self.register_partial_move(Location::Cell { i: 0 });
        } else if char == self.key_config.cell2 {
            self.register_partial_move(Location::Cell { i: 1 });
        } else if char == self.key_config.cell3 {
            self.register_partial_move(Location::Cell { i: 2 });
        } else if char == self.key_config.cell4 {
            self.register_partial_move(Location::Cell { i: 3 });
        } else if char == self.key_config.foundation1
            || char == self.key_config.foundation2
            || char == self.key_config.foundation3
            || char == self.key_config.foundation4
        {
            self.register_partial_move(Location::Foundation);
        } else if char == self.key_config.column1 {
            self.register_partial_move(Location::Column { i: 0 });
        } else if char == self.key_config.column2 {
            self.register_partial_move(Location::Column { i: 1 });
        } else if char == self.key_config.column3 {
            self.register_partial_move(Location::Column { i: 2 });
        } else if char == self.key_config.column4 {
            self.register_partial_move(Location::Column { i: 3 });
        } else if char == self.key_config.column5 {
            self.register_partial_move(Location::Column { i: 4 });
        } else if char == self.key_config.column6 {
            self.register_partial_move(Location::Column { i: 5 });
        } else if char == self.key_config.column7 {
            self.register_partial_move(Location::Column { i: 6 });
        } else if char == self.key_config.column8 {
            self.register_partial_move(Location::Column { i: 7 });
        } else if char == ' ' {
            self.clear_move();
        } else if char == 'R' {
            self.revert();
        }
    }

    /// Handles key events when the help modal is active.
    fn handle_key_event_help_modal(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.exit()
            }
            KeyCode::F(12) => {
                self.about_modal();
            }
            KeyCode::Esc => {
                self.base();
            }
            _ => {}
        }
    }

    /// Handles key events when the about modal is active.
    fn handle_key_event_about_modal(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.exit()
            }
            KeyCode::F(1) => {
                self.help_modal();
            }
            KeyCode::Up => {
                self.about_modal_scroll(false);
            }
            KeyCode::Down => {
                self.about_modal_scroll(true);
            }
            KeyCode::Esc => {
                self.base();
            }
            _ => {}
        }
    }

    /// Handles key events when the "game selection by id" modal is active.
    fn handle_key_event_selection_id_modal(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.exit()
            }
            KeyCode::F(1) => {
                self.help_modal();
            }
            KeyCode::Esc => {
                self.base();
            }
            KeyCode::Enter => {
                self.selection_id_try_start();
            }
            KeyCode::Backspace => {
                self.selection_id_modal_delete_char();
            }
            KeyCode::Char(char)
                if char == '0'
                    || char == '1'
                    || char == '2'
                    || char == '3'
                    || char == '4'
                    || char == '5'
                    || char == '6'
                    || char == '7'
                    || char == '8'
                    || char == '9' =>
            {
                self.selection_id_modal_add_char(char.to_digit(10).unwrap());
            }
            _ => {}
        }
    }

    /// Handles key events when the "game selection from journey" modal is active.
    fn handle_key_event_selection_journey_modal(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.exit()
            }
            KeyCode::Esc => {
                self.base();
            }
            KeyCode::Char('1') => {
                self.selection_through_journey_start_next();
            }
            KeyCode::Char(number)
                if number == '2'
                    || number == '3'
                    || number == '4'
                    || number == '5'
                    || number == '6'
                    || number == '7'
                    || number == '8'
                    || number == '9' =>
            {
                self.selection_through_journey_start_skipped(number);
            }
            KeyCode::Char('s') => {
                self.selection_through_journey_skip_next_game();
            }
            _ => {}
        }
    }

    /// Starts a random game.
    fn random_game(&mut self) {
        self.game_handler.random_game();
    }

    /// Switches to base state.
    fn base(&mut self) {
        self.app_state = AppState::Base;
    }

    /// Begins exiting the app.
    fn exit(&mut self) {
        self.app_state = AppState::Exit;
    }

    /// Switches to help modal.
    fn help_modal(&mut self) {
        self.app_state = AppState::HelpModal;
    }

    /// Switches to about modal.
    fn about_modal(&mut self) {
        self.app_state = AppState::AboutModal { scroll: 0 };
    }

    /// Changes the scoll value for the about modal.
    fn about_modal_scroll(&mut self, down: bool) {
        if let AppState::AboutModal { scroll: old_scroll } = self.app_state {
            if down {
                self.app_state = AppState::AboutModal {
                    scroll: old_scroll.saturating_add(1),
                }
            } else {
                self.app_state = AppState::AboutModal {
                    scroll: old_scroll.saturating_sub(1),
                }
            }
        }
    }

    /// Switches to "game selection by id" modal.
    fn selection_id_modal(&mut self) {
        self.app_state = AppState::SelectionIdModal {
            id: [SPACE_ASCII_CODE; 5],
        }
    }

    /// Removes the last character from the "game selection by id" modal.
    fn selection_id_modal_delete_char(&mut self) {
        match &self.app_state {
            AppState::SelectionIdModal { id } => {
                if id[4] != SPACE_ASCII_CODE {
                    let new_id: [u8; 5] = [SPACE_ASCII_CODE, id[0], id[1], id[2], id[3]];
                    self.app_state = AppState::SelectionIdModal { id: new_id };
                }
            }
            _ => {}
        }
    }

    /// Adds a character to the "game selection by id" modal.
    fn selection_id_modal_add_char(&mut self, new_digit: u32) {
        match self.app_state {
            AppState::SelectionIdModal { id } => {
                if id[0] == SPACE_ASCII_CODE {
                    let new_id: [u8; 5] = [id[1], id[2], id[3], id[4], 0x30 + new_digit as u8];
                    self.app_state = AppState::SelectionIdModal { id: new_id };
                }
            }
            _ => {}
        }
    }

    /// Attempts to select the game from the value
    /// entered in the "game selection by id" modal.
    fn selection_id_try_start(&mut self) -> () {
        if let AppState::SelectionIdModal { id } = &self.app_state {
            if id[4] != SPACE_ASCII_CODE {
                let id = str::from_utf8(id).unwrap().trim();
                let id = u32::from_str_radix(id, 10).unwrap();
                if 0 < id && id < 64001 {
                    self.game_from_u16_id(id as u16);
                    self.base();
                }
            }
        }
    }

    /// Starts a selected skipped game within the journey.
    fn selection_through_journey_start_next(&mut self) {
        let id: u16;
        {
            id = self.journey_handler.borrow().next_game_ids().0;
        }

        self.game_from_u16_id(id);
        self.base();
    }

    /// Starts a selected skipped game within the journey.
    fn selection_through_journey_start_skipped(&mut self, number: char) {
        let number = number.to_digit(10).unwrap() as usize;

        let index = if number == 0 { 8 } else { number - 2 };

        let items: Vec<u16>;
        {
            items = self.journey_handler.borrow().next_game_ids().1.clone();
        }

        let id = items.get(index);

        if let Some(id) = id {
            self.game_from_u16_id(*id);
            self.base();
        }
    }

    /// Moves the next game to the skipped games within the journey.
    fn selection_through_journey_skip_next_game(&mut self) {
        self.journey_handler.borrow_mut().skip_next_game()
    }

    /// Start a game from the given id.
    fn game_from_u16_id(&mut self, id: u16) {
        self.game_handler.game_from_id(id);
    }

    /// Switches to "game selection by id" modal.
    fn selection_journey_modal(&mut self) {
        self.app_state = AppState::SelectionJourneyModal;
    }

    /// Register one half of a move, and executes a move,
    /// if completed.
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

    /// Removes a registered half-move, if any.
    fn clear_move(&mut self) {
        self.move_from = None;
    }

    /// Reverts the previous, completed move.
    fn revert(&mut self) {
        self.move_from = None;
        let _ = self.game_handler.revert();
    }
}

impl<T> Widget for &mut App<T>
where
    T: PersistJourney,
    T: Debug,
{
    /// Entry point for the rendering.
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 32 || area.height < 24 {
            panic!("RSLibreCell needs at least 32 columns and 24 lines in the terminal");
        }

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
            render::provide_game_lines(&mut lines, game);
        }

        let board_text = Text::from(lines);

        Paragraph::new(board_text)
            .centered()
            .block(block)
            .render(area, buf);

        match self.app_state {
            AppState::Base => {}
            AppState::Exit => panic!("should never happen"),
            AppState::HelpModal => render::render_help_modal(&self.key_config, area, buf),
            AppState::AboutModal { scroll } => {
                let mut new_scroll = scroll;
                let mut set_scroll = |x: u16| -> () {
                    new_scroll = x;
                };
                render::render_about_modal(area, buf, scroll, &mut set_scroll);
                self.app_state = AppState::AboutModal { scroll: new_scroll };
            }
            AppState::SelectionIdModal { id } => {
                render::render_selection_id_modal(area, buf, id);
            }
            AppState::SelectionJourneyModal => render::render_selection_journey_modal(
                area,
                buf,
                self.journey_handler.borrow().next_game_ids(),
            ),
        }
    }
}

mod render {
    use super::*;
    use rslibrecell::game::Game;

    /// Provides the lines for the inner game board.
    pub(crate) fn provide_game_lines<'a>(lines: &mut Vec<Line<'a>>, game: &'a Game) {
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
        lines.push(Line::from("----------------------------------"));

        if game.is_won() {
            lines.push(Line::from(""));
            lines.push(Line::from("Congratulations, you won!"));
        } else {
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
    }

    /// Renders the help modal.
    pub(crate) fn render_help_modal(key_config: &KeyConfig, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Help ");
        let instructions = Line::from(vec![" Close ".into(), "<Esc> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered());

        let mut help_lines: Vec<Line> = vec![];
        help_lines.push(Line::from(vec![
            "<F12>".blue(),
            " to show the About box.".into(),
        ]));
        help_lines.push(Line::from(vec![
            "<F2>".cyan(),
            " to start a new random game.".into(),
        ]));
        help_lines.push(Line::from(vec![
            "<F3>".cyan(),
            " to choose a game to start.".into(),
        ]));
        help_lines.push(Line::from(vec![
            "<!>".cyan(),
            " to open the Journey box.".into(),
        ]));
        help_lines.push(Line::from("\n"));
        help_lines.push(Line::from(vec![
            format!("<{}> <{}> <{}> <{}>", key_config.cell1, key_config.cell2, key_config.cell3, key_config.cell4).cyan(),
            " - cells ".into(),
        ]));
        help_lines.push(Line::from(vec![
            format!("<{}> <{}> <{}> <{}>", key_config.foundation1, key_config.foundation2, key_config.foundation3, key_config.foundation4).cyan(),
            " - foundations ".into(),
        ]));
        help_lines.push(Line::from(vec![
            format!("<{}> <{}> <{}> <{}>", key_config.column1, key_config.column2, key_config.column3, key_config.column4).cyan(),
            " - left columns ".into(),
        ]));
        help_lines.push(Line::from(vec![
            format!("<{}> <{}> <{}> <{}>", key_config.column5, key_config.column6, key_config.column7, key_config.column8).cyan(),
            " - right columns ".into(),
        ]));
        help_lines.push(Line::from("\n"));
        help_lines.push(Line::from(vec![
            "Make a move by choosing the start and end of a move. ".into(),
            "<Space>".cyan(),
            " to abort a move. ".into(),
            "<R>".cyan(),
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

    /// Renders the about modal.
    pub(crate) fn render_about_modal<F>(
        area: Rect,
        buf: &mut Buffer,
        scroll: u16,
        set_scroll: &mut F,
    ) where
        F: FnMut(u16) -> (),
    {
        let title = Line::from(" About ");
        let instructions = Line::from(vec![
            " Scroll ".into(),
            "<Up><Down>".blue().bold(),
            " Close ".into(),
            "<Esc> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered());

        let about_lines = create_about_text();

        let about_text = Text::from(about_lines);

        let area = popup_area(area);
        Clear::default().render(area, buf);

        block.render(area, buf);

        let inner_area = area.inner(Margin {
            horizontal: 2,
            vertical: 1,
        });

        let wrapped_paragraph = Paragraph::new(about_text).wrap(Wrap { trim: false });

        let line_count = wrapped_paragraph.line_count(inner_area.width);

        let effective_scroll = line_count
            .saturating_sub(inner_area.height as usize)
            .min(scroll as usize);

        set_scroll(effective_scroll as u16);

        wrapped_paragraph
            .scroll((effective_scroll as u16, 0)) // y, x
            .render(inner_area, buf);
    }

    /// Renders the "game selection by id" modal.
    pub(crate) fn render_selection_id_modal(area: Rect, buf: &mut Buffer, id: [u8; 5]) {
        let title = Line::from(" Choose game by ID ");
        let instructions = Line::from(vec![
            " Accept ".into(),
            "<Enter>".blue().bold(),
            " Abort ".into(),
            "<Esc> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered());

        let id_representation = str::from_utf8(&id).unwrap();

        let mut entry = id_representation.underlined();

        if id[4] != SPACE_ASCII_CODE {
            let check_id = u32::from_str_radix(id_representation.trim(), 10).unwrap();
            if 0 == check_id || check_id > 64000 {
                entry = entry.red();
            }
        }

        let lines: Vec<Line<'_>> = vec!["Enter ID:".into(), entry.into()];

        let content = Text::from(lines);

        let area = popup_area(area);
        Clear::default().render(area, buf);

        block.render(area, buf);

        let label_area = Rect {
            x: area.x + 2,
            y: area.y + 2,
            width: area.width - 4,
            height: 4,
        };

        Paragraph::new(content).centered().render(label_area, buf);
    }

    /// Renders the "game selection from journey" modal.
    pub(crate) fn render_selection_journey_modal(
        area: Rect,
        buf: &mut Buffer,
        next_game_ids: (u16, Vec<u16>),
    ) {
        let title = Line::from(" Journey ");
        let instructions = Line::from(vec![" Close ".into(), "<Esc> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered());

        let mut selection_lines: Vec<Line> = vec![];

        let next_game_exists = next_game_ids.0 != 64001;
        let skipped_games_exist = next_game_ids.1.len() > 0;
        let journey_completed = !next_game_exists && !skipped_games_exist;

        if journey_completed {
            selection_lines.push(Line::from(vec!["".into()]));
            selection_lines.push(Line::from(vec!["Journey completed!".bold()]));
        } else {
            selection_lines.push(Line::from(vec!["Press number to start:".into()]));

            if next_game_exists {
                selection_lines.push(Line::from(vec![
                    "<1>".blue().bold(),
                    " (next game) : ".into(),
                    format!("{:>5}", next_game_ids.0).into(),
                ]));
                selection_lines.push(Line::from(vec![
                    "<s>".blue().bold(),
                    " to skip for now".into(),
                ]));
                selection_lines.push(Line::from(vec!["".into()]));
            }

            if skipped_games_exist {
                selection_lines.push(Line::from(vec!["Previously skipped games".into()]));

                let mut key = 2;

                for skipped in next_game_ids.1.iter().take(8) {
                    selection_lines.push(Line::from(vec![
                        format!("<{}>", key).blue().bold(),
                        format!(" : {:>5}", skipped).into(),
                    ]));
                    key += 1;
                }

                if next_game_ids.1.len() > 8 {
                    selection_lines.push(Line::from(vec!["".into()]));
                    selection_lines.push(Line::from(vec!["... more skipped games".into()]));
                }
            }
        }

        let text = Text::from(selection_lines);

        let area = popup_area(area);
        Clear::default().render(area, buf);

        block.render(area, buf);

        let inner_area = area.inner(Margin {
            horizontal: 2,
            vertical: 1,
        });

        let mut paragraph = Paragraph::new(text);

        if journey_completed {
            paragraph = paragraph.centered();
        }

        paragraph.render(inner_area, buf);
    }

    /// Helper function to create a centered rect with a fixed margin.
    fn popup_area(area: Rect) -> Rect {
        Rect {
            x: area.x + 2,
            y: area.y + 1,
            width: area.width - 4,
            height: area.height - 2,
        }
    }

    /// Gets the colored representation of a card.
    fn get_colored_representation(card: &Card) -> Span<'_> {
        let unstyled_span = <Span<'_>>::from(format!(" {c} ", c = card.to_string()));
        match card.suit {
            Suit::Clubs | Suit::Spades => return unstyled_span.into(),
            Suit::Diamonds | Suit::Hearts => return unstyled_span.red(),
        }
    }

    /// Creates the about text.
    fn create_about_text<'a>() -> Vec<Line<'a>> {
        vec![
                Line::from(vec!["RSLibreCell - a FreeCell implementation".bold()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["Copyright (c) tristhaus 2025 and later".into()]),
                Line::from(vec!["https://www.github.com/tristhaus/rslibrecell".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["For help, press ".into(), "<F1>".blue(), ".".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["RSLibreCell is free, libre, open-source software. License text below:".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec![" GNU GENERAL PUBLIC LICENSE".into()]),
                Line::from(vec![" Version 3, 29 June 2007".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec![" Copyright (C) 2007 Free Software Foundation, Inc. <https://fsf.org/> Everyone is permitted to copy and distribute verbatim copies of this license document, but changing it is not allowed.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["Preamble".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  The GNU General Public License is a free, copyleft license for software and other kinds of works.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  The licenses for most software and other practical works are designed to take away your freedom to share and change the works.  By contrast, the GNU General Public License is intended to guarantee your freedom to share and change all versions of a program--to make sure it remains free software for all its users.  We, the Free Software Foundation, use the GNU General Public License for most of our software; it applies also to any other work released this way by its authors.  You can apply it to your programs, too.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  When we speak of free software, we are referring to freedom, not price.  Our General Public Licenses are designed to make sure that you have the freedom to distribute copies of free software (and charge for them if you wish), that you receive source code or can get it if you want it, that you can change the software or use pieces of it in new free programs, and that you know you can do these things.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  To protect your rights, we need to prevent others from denying you these rights or asking you to surrender the rights.  Therefore, you have certain responsibilities if you distribute copies of the software, or if you modify it: responsibilities to respect the freedom of others.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  For example, if you distribute copies of such a program, whether gratis or for a fee, you must pass on to the recipients the same freedoms that you received.  You must make sure that they, too, receive or can get the source code.  And you must show them these terms so they know their rights.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  Developers that use the GNU GPL protect your rights with two steps: (1) assert copyright on the software, and (2) offer you this License giving you legal permission to copy, distribute and/or modify it.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  For the developers' and authors' protection, the GPL clearly explains that there is no warranty for this free software.  For both users' and authors' sake, the GPL requires that modified versions be marked as changed, so that their problems will not be attributed erroneously to authors of previous versions.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  Some devices are designed to deny users access to install or run modified versions of the software inside them, although the manufacturer can do so.  This is fundamentally incompatible with the aim of protecting users' freedom to change the software.  The systematic pattern of such abuse occurs in the area of products for individuals to use, which is precisely where it is most unacceptable.  Therefore, we have designed this version of the GPL to prohibit the practice for those products.  If such problems arise substantially in other domains, we stand ready to extend this provision to those domains in future versions of the GPL, as needed to protect the freedom of users.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  Finally, every program is threatened constantly by software patents.  States should not allow patents to restrict development and use of software on general-purpose computers, but in those that do, we wish to avoid the special danger that patents applied to a free program could make it effectively proprietary.  To prevent this, the GPL assures that patents cannot be used to render the program non-free.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  The precise terms and conditions for copying, distribution and modification follow.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["TERMS AND CONDITIONS".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  0. Definitions.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  \"This License\" refers to version 3 of the GNU General Public License.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  \"Copyright\" also means copyright-like laws that apply to other kinds of works, such as semiconductor masks.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  \"The Program\" refers to any copyrightable work licensed under this License.  Each licensee is addressed as \"you\".  \"Licensees\" and \"recipients\" may be individuals or organizations.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  To \"modify\" a work means to copy from or adapt all or part of the work in a fashion requiring copyright permission, other than the making of an exact copy.  The resulting work is called a \"modified version\" of the earlier work or a work \"based on\" the earlier work.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  A \"covered work\" means either the unmodified Program or a work based on the Program.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  To \"propagate\" a work means to do anything with it that, without permission, would make you directly or secondarily liable for infringement under applicable copyright law, except executing it on a computer or modifying a private copy.  Propagation includes copying, distribution (with or without modification), making available to the public, and in some countries other activities as well.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  To \"convey\" a work means any kind of propagation that enables other parties to make or receive copies.  Mere interaction with a user through a computer network, with no transfer of a copy, is not conveying.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  An interactive user interface displays \"Appropriate Legal Notices\" to the extent that it includes a convenient and prominently visible feature that (1) displays an appropriate copyright notice, and (2) tells the user that there is no warranty for the work (except to the extent that warranties are provided), that licensees may convey the work under this License, and how to view a copy of this License.  If the interface presents a list of user commands or options, such as a menu, a prominent item in the list meets this criterion.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  1. Source Code.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  The \"source code\" for a work means the preferred form of the work for making modifications to it.  \"Object code\" means any non-source form of a work.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  A \"Standard Interface\" means an interface that either is an official standard defined by a recognized standards body, or, in the case of interfaces specified for a particular programming language, one that is widely used among developers working in that language.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  The \"System Libraries\" of an executable work include anything, other than the work as a whole, that (a) is included in the normal form of packaging a Major Component, but which is not part of that Major Component, and (b) serves only to enable use of the work with that Major Component, or to implement a Standard Interface for which an implementation is available to the public in source code form.  A \"Major Component\", in this context, means a major essential component (kernel, window system, and so on) of the specific operating system (if any) on which the executable work runs, or a compiler used to produce the work, or an object code interpreter used to run it.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  The \"Corresponding Source\" for a work in object code form means all the source code needed to generate, install, and (for an executable work) run the object code and to modify the work, including scripts to control those activities.  However, it does not include the work's System Libraries, or general-purpose tools or generally available free programs which are used unmodified in performing those activities but which are not part of the work.  For example, Corresponding Source includes interface definition files associated with source files for the work, and the source code for shared libraries and dynamically linked subprograms that the work is specifically designed to require, such as by intimate data communication or control flow between those subprograms and other parts of the work.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  The Corresponding Source need not include anything that users can regenerate automatically from other parts of the Corresponding Source.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  The Corresponding Source for a work in source code form is that same work.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  2. Basic Permissions.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  All rights granted under this License are granted for the term of copyright on the Program, and are irrevocable provided the stated conditions are met.  This License explicitly affirms your unlimited permission to run the unmodified Program.  The output from running a covered work is covered by this License only if the output, given its content, constitutes a covered work.  This License acknowledges your rights of fair use or other equivalent, as provided by copyright law.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  You may make, run and propagate covered works that you do not convey, without conditions so long as your license otherwise remains in force.  You may convey covered works to others for the sole purpose of having them make modifications exclusively for you, or provide you with facilities for running those works, provided that you comply with the terms of this License in conveying all material for which you do not control copyright.  Those thus making or running the covered works for you must do so exclusively on your behalf, under your direction and control, on terms that prohibit them from making any copies of your copyrighted material outside their relationship with you.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  Conveying under any other circumstances is permitted solely under the conditions stated below.  Sublicensing is not allowed; section 10 makes it unnecessary.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec![                "  3. Protecting Users' Legal Rights From Anti-Circumvention Law.".into()            ]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  No covered work shall be deemed part of an effective technological measure under any applicable law fulfilling obligations under article 11 of the WIPO copyright treaty adopted on 20 December 1996, or similar laws prohibiting or restricting circumvention of such measures.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  When you convey a covered work, you waive any legal power to forbid circumvention of technological measures to the extent such circumvention is effected by exercising rights under this License with respect to the covered work, and you disclaim any intention to limit operation or modification of the work as a means of enforcing, against the work's users, your or third parties' legal rights to forbid circumvention of technological measures.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  4. Conveying Verbatim Copies.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  You may convey verbatim copies of the Program's source code as you receive it, in any medium, provided that you conspicuously and appropriately publish on each copy an appropriate copyright notice; keep intact all notices stating that this License and any non-permissive terms added in accord with section 7 apply to the code; keep intact all notices of the absence of any warranty; and give all recipients a copy of this License along with the Program.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  You may charge any price or no price for each copy that you convey, and you may offer support or warranty protection for a fee.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec![                "  5. Conveying Modified Source Versions.".into()            ]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  You may convey a work based on the Program, or the modifications to produce it from the Program, in the form of source code under the terms of section 4, provided that you also meet all of these conditions:".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["    a) The work must carry prominent notices stating that you modified it, and giving a relevant date.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["    b) The work must carry prominent notices stating that it is released under this License and any conditions added under section 7.  This requirement modifies the requirement in section 4 to \"keep intact all notices\".".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["    c) You must license the entire work, as a whole, under this License to anyone who comes into possession of a copy.  This License will therefore apply, along with any applicable section 7 additional terms, to the whole of the work, and all its parts, regardless of how they are packaged.  This License gives no permission to license the work in any other way, but it does not invalidate such permission if you have separately received it.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["    d) If the work has interactive user interfaces, each must display Appropriate Legal Notices; however, if the Program has interactive interfaces that do not display Appropriate Legal Notices, your work need not make them do so.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  A compilation of a covered work with other separate and independent works, which are not by their nature extensions of the covered work, and which are not combined with it such as to form a larger program, in or on a volume of a storage or distribution medium, is called an \"aggregate\" if the compilation and its resulting copyright are not used to limit the access or legal rights of the compilation's users beyond what the individual works permit.  Inclusion of a covered work in an aggregate does not cause this License to apply to the other parts of the aggregate.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  6. Conveying Non-Source Forms.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  You may convey a covered work in object code form under the terms of sections 4 and 5, provided that you also convey the machine-readable Corresponding Source under the terms of this License, in one of these ways:".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["    a) Convey the object code in, or embodied in, a physical product (including a physical distribution medium), accompanied by the Corresponding Source fixed on a durable physical medium customarily used for software interchange.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["    b) Convey the object code in, or embodied in, a physical product (including a physical distribution medium), accompanied by a written offer, valid for at least three years and valid for as long as you offer spare parts or customer support for that product model, to give anyone who possesses the object code either (1) a copy of the Corresponding Source for all the software in the product that is covered by this License, on a durable physical medium customarily used for software interchange, for a price no more than your reasonable cost of physically performing this conveying of source, or (2) access to copy the Corresponding Source from a network server at no charge.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["    c) Convey individual copies of the object code with a copy of the written offer to provide the Corresponding Source.  This alternative is allowed only occasionally and noncommercially, and only if you received the object code with such an offer, in accord with subsection 6b.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["    d) Convey the object code by offering access from a designated place (gratis or for a charge), and offer equivalent access to the Corresponding Source in the same way through the same place at no further charge.  You need not require recipients to copy the Corresponding Source along with the object code.  If the place to copy the object code is a network server, the Corresponding Source may be on a different server (operated by you or a third party) that supports equivalent copying facilities, provided you maintain clear directions next to the object code saying where to find the Corresponding Source.  Regardless of what server hosts the Corresponding Source, you remain obligated to ensure that it is available for as long as needed to satisfy these requirements.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["    e) Convey the object code using peer-to-peer transmission, provided you inform other peers where the object code and Corresponding Source of the work are being offered to the general public at no charge under subsection 6d.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  A separable portion of the object code, whose source code is excluded from the Corresponding Source as a System Library, need not be included in conveying the object code work.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  A \"User Product\" is either (1) a \"consumer product\", which means any tangible personal property which is normally used for personal, family, or household purposes, or (2) anything designed or sold for incorporation into a dwelling.  In determining whether a product is a consumer product, doubtful cases shall be resolved in favor of coverage.  For a particular product received by a particular user, \"normally used\" refers to a typical or common use of that class of product, regardless of the status of the particular user or of the way in which the particular user actually uses, or expects or is expected to use, the product.  A product is a consumer product regardless of whether the product has substantial commercial, industrial or non-consumer uses, unless such uses represent the only significant mode of use of the product.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  \"Installation Information\" for a User Product means any methods, procedures, authorization keys, or other information required to install and execute modified versions of a covered work in that User Product from a modified version of its Corresponding Source.  The information must suffice to ensure that the continued functioning of the modified object code is in no case prevented or interfered with solely because modification has been made.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  If you convey an object code work under this section in, or with, or specifically for use in, a User Product, and the conveying occurs as part of a transaction in which the right of possession and use of the User Product is transferred to the recipient in perpetuity or for a fixed term (regardless of how the transaction is characterized), the Corresponding Source conveyed under this section must be accompanied by the Installation Information.  But this requirement does not apply if neither you nor any third party retains the ability to install modified object code on the User Product (for example, the work has been installed in ROM).".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  The requirement to provide Installation Information does not include a requirement to continue to provide support service, warranty, or updates for a work that has been modified or installed by the recipient, or for the User Product in which it has been modified or installed.  Access to a network may be denied when the modification itself materially and adversely affects the operation of the network or violates the rules and protocols for communication across the network.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  Corresponding Source conveyed, and Installation Information provided, in accord with this section must be in a format that is publicly documented (and with an implementation available to the public in source code form), and must require no special password or key for unpacking, reading or copying.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  7. Additional Terms.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  \"Additional permissions\" are terms that supplement the terms of this License by making exceptions from one or more of its conditions.  Additional permissions that are applicable to the entire Program shall be treated as though they were included in this License, to the extent that they are valid under applicable law.  If additional permissions apply only to part of the Program, that part may be used separately under those permissions, but the entire Program remains governed by this License without regard to the additional permissions.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  When you convey a copy of a covered work, you may at your option remove any additional permissions from that copy, or from any part of it.  (Additional permissions may be written to require their own removal in certain cases when you modify the work.)  You may place additional permissions on material, added by you to a covered work, for which you have or can give appropriate copyright permission.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  Notwithstanding any other provision of this License, for material you add to a covered work, you may (if authorized by the copyright holders of that material) supplement the terms of this License with terms:".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["    a) Disclaiming warranty or limiting liability differently from the terms of sections 15 and 16 of this License; or".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["    b) Requiring preservation of specified reasonable legal notices or author attributions in that material or in the Appropriate Legal Notices displayed by works containing it; or".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["    c) Prohibiting misrepresentation of the origin of that material, or requiring that modified versions of such material be marked in reasonable ways as different from the original version; or".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["    d) Limiting the use for publicity purposes of names of licensors or authors of the material; or".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["    e) Declining to grant rights under trademark law for use of some trade names, trademarks, or service marks; or".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["    f) Requiring indemnification of licensors and authors of that material by anyone who conveys the material (or modified versions of it) with contractual assumptions of liability to the recipient, for any liability that these contractual assumptions directly impose on those licensors and authors.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  All other non-permissive additional terms are considered \"further restrictions\" within the meaning of section 10.  If the Program as you received it, or any part of it, contains a notice stating that it is governed by this License along with a term that is a further restriction, you may remove that term.  If a license document contains a further restriction but permits relicensing or conveying under this License, you may add to a covered work material governed by the terms of that license document, provided that the further restriction does not survive such relicensing or conveying.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  If you add terms to a covered work in accord with this section, you must place, in the relevant source files, a statement of the additional terms that apply to those files, or a notice indicating where to find the applicable terms.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  Additional terms, permissive or non-permissive, may be stated in the form of a separately written license, or stated as exceptions; the above requirements apply either way.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  8. Termination.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  You may not propagate or modify a covered work except as expressly provided under this License.  Any attempt otherwise to propagate or modify it is void, and will automatically terminate your rights under this License (including any patent licenses granted under the third paragraph of section 11).".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  However, if you cease all violation of this License, then your license from a particular copyright holder is reinstated (a) provisionally, unless and until the copyright holder explicitly and finally terminates your license, and (b) permanently, if the copyright holder fails to notify you of the violation by some reasonable means prior to 60 days after the cessation.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  Moreover, your license from a particular copyright holder is reinstated permanently if the copyright holder notifies you of the violation by some reasonable means, this is the first time you have received notice of violation of this License (for any work) from that copyright holder, and you cure the violation prior to 30 days after your receipt of the notice.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  Termination of your rights under this section does not terminate the licenses of parties who have received copies or rights from you under this License.  If your rights have been terminated and not permanently reinstated, you do not qualify to receive new licenses for the same material under section 10.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec![                "  9. Acceptance Not Required for Having Copies.".into(),            ]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  You are not required to accept this License in order to receive or run a copy of the Program.  Ancillary propagation of a covered work occurring solely as a consequence of using peer-to-peer transmission to receive a copy likewise does not require acceptance.  However, nothing other than this License grants you permission to propagate or modify any covered work.  These actions infringe copyright if you do not accept this License.  Therefore, by modifying or propagating a covered work, you indicate your acceptance of this License to do so.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec![                "  10. Automatic Licensing of Downstream Recipients.".into(),            ]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  Each time you convey a covered work, the recipient automatically receives a license from the original licensors, to run, modify and propagate that work, subject to this License.  You are not responsible for enforcing compliance by third parties with this License.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  An \"entity transaction\" is a transaction transferring control of an organization, or substantially all assets of one, or subdividing an organization, or merging organizations.  If propagation of a covered work results from an entity transaction, each party to that transaction who receives a copy of the work also receives whatever licenses to the work the party's predecessor in interest had or could give under the previous paragraph, plus a right to possession of the Corresponding Source of the work from the predecessor in interest, if the predecessor has it or can get it with reasonable efforts.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  You may not impose any further restrictions on the exercise of the rights granted or affirmed under this License.  For example, you may not impose a license fee, royalty, or other charge for exercise of rights granted under this License, and you may not initiate litigation (including a cross-claim or counterclaim in a lawsuit) alleging that any patent claim is infringed by making, using, selling, offering for sale, or importing the Program or any portion of it.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  11. Patents.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  A \"contributor\" is a copyright holder who authorizes use under this License of the Program or a work on which the Program is based.  The work thus licensed is called the contributor's \"contributor version\".".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  A contributor's \"essential patent claims\" are all patent claims owned or controlled by the contributor, whether already acquired or hereafter acquired, that would be infringed by some manner, permitted by this License, of making, using, or selling its contributor version, but do not include claims that would be infringed only as a consequence of further modification of the contributor version.  For purposes of this definition, \"control\" includes the right to grant patent sublicenses in a manner consistent with the requirements of this License.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  Each contributor grants you a non-exclusive, worldwide, royalty-free patent license under the contributor's essential patent claims, to make, use, sell, offer for sale, import and otherwise run, modify and propagate the contents of its contributor version.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  In the following three paragraphs, a \"patent license\" is any express agreement or commitment, however denominated, not to enforce a patent (such as an express permission to practice a patent or covenant not to sue for patent infringement).  To \"grant\" such a patent license to a party means to make such an agreement or commitment not to enforce a patent against the party.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  If you convey a covered work, knowingly relying on a patent license, and the Corresponding Source of the work is not available for anyone to copy, free of charge and under the terms of this License, through a publicly available network server or other readily accessible means, then you must either (1) cause the Corresponding Source to be so available, or (2) arrange to deprive yourself of the benefit of the patent license for this particular work, or (3) arrange, in a manner consistent with the requirements of this License, to extend the patent license to downstream recipients.  \"Knowingly relying\" means you have actual knowledge that, but for the patent license, your conveying the covered work in a country, or your recipient's use of the covered work in a country, would infringe one or more identifiable patents in that country that you have reason to believe are valid.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  If, pursuant to or in connection with a single transaction or arrangement, you convey, or propagate by procuring conveyance of, a covered work, and grant a patent license to some of the parties receiving the covered work authorizing them to use, propagate, modify or convey a specific copy of the covered work, then the patent license you grant is automatically extended to all recipients of the covered work and works based on it.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  A patent license is \"discriminatory\" if it does not include within the scope of its coverage, prohibits the exercise of, or is conditioned on the non-exercise of one or more of the rights that are specifically granted under this License.  You may not convey a covered work if you are a party to an arrangement with a third party that is in the business of distributing software, under which you make payment to the third party based on the extent of your activity of conveying the work, and under which the third party grants, to any of the parties who would receive the covered work from you, a discriminatory patent license (a) in connection with copies of the covered work conveyed by you (or copies made from those copies), or (b) primarily for and in connection with specific products or compilations that contain the covered work, unless you entered into that arrangement, or that patent license was granted, prior to 28 March 2007.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  Nothing in this License shall be construed as excluding or limiting any implied license or other defenses to infringement that may otherwise be available to you under applicable patent law.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec![                "  12. No Surrender of Others' Freedom.".into()            ]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  If conditions are imposed on you (whether by court order, agreement or otherwise) that contradict the conditions of this License, they do not excuse you from the conditions of this License.  If you cannot convey a covered work so as to satisfy simultaneously your obligations under this License and any other pertinent obligations, then as a consequence you may not convey it at all.  For example, if you agree to terms that obligate you to collect a royalty for further conveying from those to whom you convey the Program, the only way you could satisfy both those terms and this License would be to refrain entirely from conveying the Program.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec![                "  13. Use with the GNU Affero General Public License.".into(),            ]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  Notwithstanding any other provision of this License, you have permission to link or combine any covered work with a work licensed under version 3 of the GNU Affero General Public License into a single combined work, and to convey the resulting work.  The terms of this License will continue to apply to the part which is the covered work, but the special requirements of the GNU Affero General Public License, section 13, concerning interaction through a network will apply to the combination as such.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec![                "  14. Revised Versions of this License.".into()            ]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  The Free Software Foundation may publish revised and/or new versions of the GNU General Public License from time to time.  Such new versions will be similar in spirit to the present version, but may differ in detail to address new problems or concerns.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  Each version is given a distinguishing version number.  If the Program specifies that a certain numbered version of the GNU General Public License \"or any later version\" applies to it, you have the option of following the terms and conditions either of that numbered version or of any later version published by the Free Software Foundation.  If the Program does not specify a version number of the GNU General Public License, you may choose any version ever published by the Free Software Foundation.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  If the Program specifies that a proxy can decide which future versions of the GNU General Public License can be used, that proxy's public statement of acceptance of a version permanently authorizes you to choose that version for the Program.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  Later license versions may give you additional or different permissions.  However, no additional obligations are imposed on any author or copyright holder as a result of your choosing to follow a later version.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  15. Disclaimer of Warranty.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  THERE IS NO WARRANTY FOR THE PROGRAM, TO THE EXTENT PERMITTED BY APPLICABLE LAW.  EXCEPT WHEN OTHERWISE STATED IN WRITING THE COPYRIGHT HOLDERS AND/OR OTHER PARTIES PROVIDE THE PROGRAM \"AS IS\" WITHOUT WARRANTY OF ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE.  THE ENTIRE RISK AS TO THE QUALITY AND PERFORMANCE OF THE PROGRAM IS WITH YOU.  SHOULD THE PROGRAM PROVE DEFECTIVE, YOU ASSUME THE COST OF ALL NECESSARY SERVICING, REPAIR OR CORRECTION.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  16. Limitation of Liability.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  IN NO EVENT UNLESS REQUIRED BY APPLICABLE LAW OR AGREED TO IN WRITING WILL ANY COPYRIGHT HOLDER, OR ANY OTHER PARTY WHO MODIFIES AND/OR CONVEYS THE PROGRAM AS PERMITTED ABOVE, BE LIABLE TO YOU FOR DAMAGES, INCLUDING ANY GENERAL, SPECIAL, INCIDENTAL OR CONSEQUENTIAL DAMAGES ARISING OUT OF THE USE OR INABILITY TO USE THE PROGRAM (INCLUDING BUT NOT LIMITED TO LOSS OF DATA OR DATA BEING RENDERED INACCURATE OR LOSSES SUSTAINED BY YOU OR THIRD PARTIES OR A FAILURE OF THE PROGRAM TO OPERATE WITH ANY OTHER PROGRAMS), EVEN IF SUCH HOLDER OR OTHER PARTY HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec![                "  17. Interpretation of Sections 15 and 16.".into(),            ]),
                Line::from(vec!["".into()]),
                Line::from(vec!["  If the disclaimer of warranty and limitation of liability provided above cannot be given local legal effect according to their terms, reviewing courts shall apply local law that most closely approximates an absolute waiver of all civil liability in connection with the Program, unless a warranty or assumption of liability accompanies a copy of the Program in return for a fee.".into()]),
                Line::from(vec!["".into()]),
                Line::from(vec!["END OF TERMS AND CONDITIONS".into()]),
            ]
    }
}

#[cfg(test)]
mod test;
