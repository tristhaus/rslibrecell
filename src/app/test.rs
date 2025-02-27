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

use super::*;
use crossterm::event::KeyModifiers;
use mockall::predicate;
use ratatui::style::Style;
use rslibrecell::journey_handler::journey_repository::MockPersistJourney;

#[test]
fn render_startup() {
    let mut app = helper::setup_app();
    let mut buf = Buffer::empty(Rect::new(0, 0, 50, 24));

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━━━━━━━━━━ RSLibreCell ━━━━━━━━━━━━━━━━━━┓",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┗━━━━━━━━━━━ Help <F1> Quit <CTRL-q> ━━━━━━━━━━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(18, 0, 13, 1), title_style);
        expected.set_style(Rect::new(18, 23, 4, 1), key_style);
        expected.set_style(Rect::new(28, 23, 9, 1), key_style);
    }

    assert_eq!(buf, expected);
}

#[test]
#[should_panic]
fn render_too_narrow_should_panic() {
    let mut app = helper::setup_app();
    let mut buf = Buffer::empty(Rect::new(0, 0, 31, 24));

    app.render(buf.area, &mut buf);
}

#[test]
#[should_panic]
fn render_too_flat_should_panic() {
    let mut app = helper::setup_app();
    let mut buf = Buffer::empty(Rect::new(0, 0, 32, 23));

    app.render(buf.area, &mut buf);
}

#[test]
fn handle_key_event_random_game() {
    let mut app = helper::setup_app();
    assert!(app.game_handler.game.is_none());
    app.handle_key_event(KeyCode::F(2).into());
    assert_eq!(app.app_state, AppState::Base);
    assert!(app.game_handler.game.is_some());
}

#[test]
fn render_random_games() {
    let mut app = helper::setup_app();
    let mut buf0 = Buffer::empty(Rect::new(0, 0, 50, 24));
    let mut buf1 = Buffer::empty(Rect::new(0, 0, 50, 24));
    let mut buf2 = Buffer::empty(Rect::new(0, 0, 50, 24));

    app.render(buf0.area, &mut buf0);

    let mut initial_empty = Buffer::with_lines(vec![
        "┏━━━━━━━━━━━━━━━━━ RSLibreCell ━━━━━━━━━━━━━━━━━━┓",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┗━━━━━━━━━━━ Help <F1> Quit <CTRL-q> ━━━━━━━━━━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style = Style::new().blue().bold();
        initial_empty.set_style(Rect::new(18, 0, 13, 1), title_style);
        initial_empty.set_style(Rect::new(18, 23, 4, 1), key_style);
        initial_empty.set_style(Rect::new(28, 23, 9, 1), key_style);
    }

    assert_eq!(buf0, initial_empty);

    app.handle_key_event(KeyCode::F(2).into());
    app.render(buf1.area, &mut buf1);

    app.handle_key_event(KeyCode::F(2).into());
    app.render(buf2.area, &mut buf2);

    assert_ne!(buf1, buf2);
    assert_ne!(buf1, initial_empty);
    assert_ne!(buf2, initial_empty);
}

#[test]
fn render_fixed_game() {
    let mut app = helper::setup_app();
    let mut buf = Buffer::empty(Rect::new(0, 0, 50, 24));
    app.game_from_u16_id(1);

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━━━━━━━━━━ RSLibreCell ━━━━━━━━━━━━━━━━━━┓",
        "┃                                      #1        ┃",
        "┃        ..  ..  ..  .. || ..  ..  ..  ..        ┃",
        "┃       ----------------------------------       ┃",
        "┃         J♦  2♦  9♥  J♣  5♦  7♥  7♣  5♥         ┃",
        "┃         K♦  K♣  9♠  5♠  A♦  Q♣  K♥  3♥         ┃",
        "┃         2♠  K♠  9♦  Q♦  J♠  A♠  A♥  3♣         ┃",
        "┃         4♣  5♣  T♠  Q♥  4♥  A♣  4♦  7♠         ┃",
        "┃         3♠  T♦  4♠  T♥  8♥  2♣  J♥  7♦         ┃",
        "┃         6♦  8♠  8♦  Q♠  6♣  3♦  8♣  T♣         ┃",
        "┃         6♠  9♣  2♥  6♥                         ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┗━━━━━━━━━━━ Help <F1> Quit <CTRL-q> ━━━━━━━━━━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(18, 0, 13, 1), title_style);
        expected.set_style(Rect::new(18, 23, 4, 1), key_style);
        expected.set_style(Rect::new(28, 23, 9, 1), key_style);

        let red_style = Style::new().red();
        expected.set_style(Rect::new(9, 4, 4, 1), red_style);
        expected.set_style(Rect::new(13, 4, 4, 1), red_style);
        expected.set_style(Rect::new(17, 4, 4, 1), red_style);
        expected.set_style(Rect::new(25, 4, 4, 1), red_style);
        expected.set_style(Rect::new(29, 4, 4, 1), red_style);
        expected.set_style(Rect::new(37, 4, 4, 1), red_style);
        expected.set_style(Rect::new(9, 5, 4, 1), red_style);
        expected.set_style(Rect::new(25, 5, 4, 1), red_style);
        expected.set_style(Rect::new(33, 5, 4, 1), red_style);
        expected.set_style(Rect::new(37, 5, 4, 1), red_style);
        expected.set_style(Rect::new(17, 6, 4, 1), red_style);
        expected.set_style(Rect::new(21, 6, 4, 1), red_style);
        expected.set_style(Rect::new(33, 6, 4, 1), red_style);
        expected.set_style(Rect::new(21, 7, 4, 1), red_style);
        expected.set_style(Rect::new(25, 7, 4, 1), red_style);
        expected.set_style(Rect::new(33, 7, 4, 1), red_style);
        expected.set_style(Rect::new(13, 8, 4, 1), red_style);
        expected.set_style(Rect::new(21, 8, 4, 1), red_style);
        expected.set_style(Rect::new(25, 8, 4, 1), red_style);
        expected.set_style(Rect::new(33, 8, 4, 1), red_style);
        expected.set_style(Rect::new(37, 8, 4, 1), red_style);
        expected.set_style(Rect::new(9, 9, 4, 1), red_style);
        expected.set_style(Rect::new(17, 9, 4, 1), red_style);
        expected.set_style(Rect::new(29, 9, 4, 1), red_style);
        expected.set_style(Rect::new(17, 10, 4, 1), red_style);
        expected.set_style(Rect::new(21, 10, 4, 1), red_style);
    }

    assert_eq!(buf, expected);

    app.handle_key_event(KeyCode::F(1).into());
    app.handle_key_event(KeyCode::Esc.into());

    app.render(buf.area, &mut buf);

    assert_eq!(buf, expected);
}

#[test]
fn render_fixed_game_use_game_keys() {
    let mut app = helper::setup_app();
    let mut buf = Buffer::empty(Rect::new(0, 0, 50, 24));
    app.game_from_u16_id(1);

    app.handle_key_event(KeyCode::Char('k').into());
    app.handle_key_event(KeyCode::Char('q').into());

    app.handle_key_event(KeyCode::Char('k').into());
    app.handle_key_event(KeyCode::Char('w').into());

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━━━━━━━━━━ RSLibreCell ━━━━━━━━━━━━━━━━━━┓",
        "┃                                      #1        ┃",
        "┃        3♦  ..  ..  .. || 2♣  A♠  ..  ..        ┃",
        "┃       ----------------------------------       ┃",
        "┃         J♦  2♦  9♥  J♣  5♦  7♥  7♣  5♥         ┃",
        "┃         K♦  K♣  9♠  5♠  A♦  Q♣  K♥  3♥         ┃",
        "┃         2♠  K♠  9♦  Q♦  J♠      A♥  3♣         ┃",
        "┃         4♣  5♣  T♠  Q♥  4♥      4♦  7♠         ┃",
        "┃         3♠  T♦  4♠  T♥  8♥      J♥  7♦         ┃",
        "┃         6♦  8♠  8♦  Q♠  6♣      8♣  T♣         ┃",
        "┃         6♠  9♣  2♥  6♥                         ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┗━━━━━━━━━━━ Help <F1> Quit <CTRL-q> ━━━━━━━━━━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(18, 0, 13, 1), title_style);
        expected.set_style(Rect::new(18, 23, 4, 1), key_style);
        expected.set_style(Rect::new(28, 23, 9, 1), key_style);

        let red_style = Style::new().red();
        expected.set_style(Rect::new(8, 2, 4, 1), red_style);
        expected.set_style(Rect::new(9, 4, 4, 1), red_style);
        expected.set_style(Rect::new(13, 4, 4, 1), red_style);
        expected.set_style(Rect::new(17, 4, 4, 1), red_style);
        expected.set_style(Rect::new(25, 4, 4, 1), red_style);
        expected.set_style(Rect::new(29, 4, 4, 1), red_style);
        expected.set_style(Rect::new(37, 4, 4, 1), red_style);
        expected.set_style(Rect::new(9, 5, 4, 1), red_style);
        expected.set_style(Rect::new(25, 5, 4, 1), red_style);
        expected.set_style(Rect::new(33, 5, 4, 1), red_style);
        expected.set_style(Rect::new(37, 5, 4, 1), red_style);
        expected.set_style(Rect::new(17, 6, 4, 1), red_style);
        expected.set_style(Rect::new(21, 6, 4, 1), red_style);
        expected.set_style(Rect::new(33, 6, 4, 1), red_style);
        expected.set_style(Rect::new(21, 7, 4, 1), red_style);
        expected.set_style(Rect::new(25, 7, 4, 1), red_style);
        expected.set_style(Rect::new(33, 7, 4, 1), red_style);
        expected.set_style(Rect::new(13, 8, 4, 1), red_style);
        expected.set_style(Rect::new(21, 8, 4, 1), red_style);
        expected.set_style(Rect::new(25, 8, 4, 1), red_style);
        expected.set_style(Rect::new(33, 8, 4, 1), red_style);
        expected.set_style(Rect::new(37, 8, 4, 1), red_style);
        expected.set_style(Rect::new(9, 9, 4, 1), red_style);
        expected.set_style(Rect::new(17, 9, 4, 1), red_style);
        expected.set_style(Rect::new(17, 10, 4, 1), red_style);
        expected.set_style(Rect::new(21, 10, 4, 1), red_style);
    }

    assert_eq!(buf, expected);

    app.handle_key_event(KeyCode::Char('a').into());
    app.handle_key_event(KeyCode::Char('w').into());

    app.handle_key_event(KeyCode::Char('a').into());
    app.handle_key_event(KeyCode::Char(' ').into());

    assert!(app.move_from.is_none());

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━━━━━━━━━━ RSLibreCell ━━━━━━━━━━━━━━━━━━┓",
        "┃                                      #1        ┃",
        "┃        3♦  6♠  ..  .. || 2♣  A♠  ..  ..        ┃",
        "┃       ----------------------------------       ┃",
        "┃         J♦  2♦  9♥  J♣  5♦  7♥  7♣  5♥         ┃",
        "┃         K♦  K♣  9♠  5♠  A♦  Q♣  K♥  3♥         ┃",
        "┃         2♠  K♠  9♦  Q♦  J♠      A♥  3♣         ┃",
        "┃         4♣  5♣  T♠  Q♥  4♥      4♦  7♠         ┃",
        "┃         3♠  T♦  4♠  T♥  8♥      J♥  7♦         ┃",
        "┃         6♦  8♠  8♦  Q♠  6♣      8♣  T♣         ┃",
        "┃             9♣  2♥  6♥                         ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┗━━━━━━━━━━━ Help <F1> Quit <CTRL-q> ━━━━━━━━━━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(18, 0, 13, 1), title_style);
        expected.set_style(Rect::new(18, 23, 4, 1), key_style);
        expected.set_style(Rect::new(28, 23, 9, 1), key_style);

        let red_style = Style::new().red();
        expected.set_style(Rect::new(8, 2, 4, 1), red_style);
        expected.set_style(Rect::new(9, 4, 4, 1), red_style);
        expected.set_style(Rect::new(13, 4, 4, 1), red_style);
        expected.set_style(Rect::new(17, 4, 4, 1), red_style);
        expected.set_style(Rect::new(25, 4, 4, 1), red_style);
        expected.set_style(Rect::new(29, 4, 4, 1), red_style);
        expected.set_style(Rect::new(37, 4, 4, 1), red_style);
        expected.set_style(Rect::new(9, 5, 4, 1), red_style);
        expected.set_style(Rect::new(25, 5, 4, 1), red_style);
        expected.set_style(Rect::new(33, 5, 4, 1), red_style);
        expected.set_style(Rect::new(37, 5, 4, 1), red_style);
        expected.set_style(Rect::new(17, 6, 4, 1), red_style);
        expected.set_style(Rect::new(21, 6, 4, 1), red_style);
        expected.set_style(Rect::new(33, 6, 4, 1), red_style);
        expected.set_style(Rect::new(21, 7, 4, 1), red_style);
        expected.set_style(Rect::new(25, 7, 4, 1), red_style);
        expected.set_style(Rect::new(33, 7, 4, 1), red_style);
        expected.set_style(Rect::new(13, 8, 4, 1), red_style);
        expected.set_style(Rect::new(21, 8, 4, 1), red_style);
        expected.set_style(Rect::new(25, 8, 4, 1), red_style);
        expected.set_style(Rect::new(33, 8, 4, 1), red_style);
        expected.set_style(Rect::new(37, 8, 4, 1), red_style);
        expected.set_style(Rect::new(9, 9, 4, 1), red_style);
        expected.set_style(Rect::new(17, 9, 4, 1), red_style);
        expected.set_style(Rect::new(17, 10, 4, 1), red_style);
        expected.set_style(Rect::new(21, 10, 4, 1), red_style);
    }

    assert_eq!(buf, expected);

    app.handle_key_event(KeyCode::Char('a').into());
    app.handle_key_event(KeyCode::Char('R').into());

    assert!(app.move_from.is_none());

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━━━━━━━━━━ RSLibreCell ━━━━━━━━━━━━━━━━━━┓",
        "┃                                      #1        ┃",
        "┃        3♦  ..  ..  .. || 2♣  A♠  ..  ..        ┃",
        "┃       ----------------------------------       ┃",
        "┃         J♦  2♦  9♥  J♣  5♦  7♥  7♣  5♥         ┃",
        "┃         K♦  K♣  9♠  5♠  A♦  Q♣  K♥  3♥         ┃",
        "┃         2♠  K♠  9♦  Q♦  J♠      A♥  3♣         ┃",
        "┃         4♣  5♣  T♠  Q♥  4♥      4♦  7♠         ┃",
        "┃         3♠  T♦  4♠  T♥  8♥      J♥  7♦         ┃",
        "┃         6♦  8♠  8♦  Q♠  6♣      8♣  T♣         ┃",
        "┃         6♠  9♣  2♥  6♥                         ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┗━━━━━━━━━━━ Help <F1> Quit <CTRL-q> ━━━━━━━━━━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(18, 0, 13, 1), title_style);
        expected.set_style(Rect::new(18, 23, 4, 1), key_style);
        expected.set_style(Rect::new(28, 23, 9, 1), key_style);

        let red_style = Style::new().red();
        expected.set_style(Rect::new(8, 2, 4, 1), red_style);
        expected.set_style(Rect::new(9, 4, 4, 1), red_style);
        expected.set_style(Rect::new(13, 4, 4, 1), red_style);
        expected.set_style(Rect::new(17, 4, 4, 1), red_style);
        expected.set_style(Rect::new(25, 4, 4, 1), red_style);
        expected.set_style(Rect::new(29, 4, 4, 1), red_style);
        expected.set_style(Rect::new(37, 4, 4, 1), red_style);
        expected.set_style(Rect::new(9, 5, 4, 1), red_style);
        expected.set_style(Rect::new(25, 5, 4, 1), red_style);
        expected.set_style(Rect::new(33, 5, 4, 1), red_style);
        expected.set_style(Rect::new(37, 5, 4, 1), red_style);
        expected.set_style(Rect::new(17, 6, 4, 1), red_style);
        expected.set_style(Rect::new(21, 6, 4, 1), red_style);
        expected.set_style(Rect::new(33, 6, 4, 1), red_style);
        expected.set_style(Rect::new(21, 7, 4, 1), red_style);
        expected.set_style(Rect::new(25, 7, 4, 1), red_style);
        expected.set_style(Rect::new(33, 7, 4, 1), red_style);
        expected.set_style(Rect::new(13, 8, 4, 1), red_style);
        expected.set_style(Rect::new(21, 8, 4, 1), red_style);
        expected.set_style(Rect::new(25, 8, 4, 1), red_style);
        expected.set_style(Rect::new(33, 8, 4, 1), red_style);
        expected.set_style(Rect::new(37, 8, 4, 1), red_style);
        expected.set_style(Rect::new(9, 9, 4, 1), red_style);
        expected.set_style(Rect::new(17, 9, 4, 1), red_style);
        expected.set_style(Rect::new(17, 10, 4, 1), red_style);
        expected.set_style(Rect::new(21, 10, 4, 1), red_style);
    }

    assert_eq!(buf, expected);
}

#[test]
fn render_fixed_won_game() {
    let mut app = helper::setup_app();
    let mut buf = Buffer::empty(Rect::new(0, 0, 50, 24));
    app.game_from_u16_id(100);

    let mut punch_key = |key: char| {
        app.handle_key_event(KeyCode::Char(key).into());
    };

    {
        punch_key('l');
        punch_key('s');

        punch_key('l');
        punch_key('q');

        punch_key('a');
        punch_key('l');

        punch_key('j');
        punch_key('l');

        punch_key('l');
        punch_key('j');

        punch_key('a');
        punch_key('i');

        punch_key('a');
        punch_key('j');

        punch_key('a');
        punch_key('l');

        punch_key('a');
        punch_key('w');

        punch_key('l');
        punch_key('a');

        punch_key('l');
        punch_key('a');

        punch_key('j');
        punch_key('a');

        punch_key('ö');
        punch_key('e');

        punch_key('s');
        punch_key('l');

        punch_key('ö');
        punch_key('r');

        punch_key('k');
        punch_key('d');

        punch_key('f');
        punch_key('k');

        punch_key('f');
        punch_key('e');

        punch_key('j');
        punch_key('ö');

        punch_key('f');
        punch_key('ö');

        punch_key('j');
        punch_key('ö');

        punch_key('k');
        punch_key('ö');

        punch_key('e');
        punch_key('f');

        punch_key('r');
        punch_key('f');

        punch_key('d');
        punch_key('j');

        punch_key('d');
        punch_key('e');

        punch_key('d');
        punch_key('l');

        punch_key('d');
        punch_key('f');

        punch_key('d');
        punch_key('r');

        punch_key('k');
        punch_key('d');

        punch_key('s');
        punch_key('q');
    }

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━━━━━━━━━━ RSLibreCell ━━━━━━━━━━━━━━━━━━┓",
        "┃                                    #100        ┃",
        "┃        ..  ..  ..  .. || K♣  K♠  K♥  K♦        ┃",
        "┃       ----------------------------------       ┃",
        "┃                                                ┃",
        "┃            Congratulations, you won!           ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┃                                                ┃",
        "┗━━━━━━━━━━━ Help <F1> Quit <CTRL-q> ━━━━━━━━━━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(18, 0, 13, 1), title_style);
        expected.set_style(Rect::new(18, 23, 4, 1), key_style);
        expected.set_style(Rect::new(28, 23, 9, 1), key_style);

        let red_style = Style::new().red();
        expected.set_style(Rect::new(34, 2, 8, 1), red_style);
    }

    assert_eq!(buf, expected);
}

#[test]
fn handle_key_event_quit() {
    let mut app = helper::setup_app();
    let mut key: KeyEvent = KeyCode::Char('q').into();
    key.modifiers = KeyModifiers::CONTROL;
    app.handle_key_event(key);
    assert_eq!(app.app_state, AppState::Exit);
}

#[test]
fn handle_key_event_help_modal() {
    let mut app = helper::setup_app();
    app.handle_key_event(KeyCode::F(1).into());
    assert_eq!(app.app_state, AppState::HelpModal);

    app.handle_key_event(KeyCode::Esc.into());
    assert_eq!(app.app_state, AppState::Base);

    app.handle_key_event(KeyCode::F(1).into());
    assert_eq!(app.app_state, AppState::HelpModal);

    let mut key: KeyEvent = KeyCode::Char('q').into();
    key.modifiers = KeyModifiers::CONTROL;
    app.handle_key_event(key);
    assert_eq!(app.app_state, AppState::Exit);
}

#[test]
fn render_help_modal() {
    let mut app = helper::setup_app();
    let mut buf = Buffer::empty(Rect::new(0, 0, 50, 24));
    app.handle_key_event(KeyCode::F(1).into());

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━━━━━━━━━━ RSLibreCell ━━━━━━━━━━━━━━━━━━┓",
        "┃ ┌─────────────────── Help ───────────────────┐ ┃",
        "┃ │ <F12> to show the About box.               │ ┃",
        "┃ │ <F2> to start a new random game.           │ ┃",
        "┃ │ <F3> to choose a game to start.            │ ┃",
        "┃ │ <!> to open the Journey box.               │ ┃",
        "┃ │                                            │ ┃",
        "┃ │ <q> <w> <e> <r> - cells                    │ ┃",
        "┃ │ <u> <i> <o> <p> - foundations              │ ┃",
        "┃ │ <a> <s> <d> <f> - left columns             │ ┃",
        "┃ │ <j> <k> <l> <ö> - right columns            │ ┃",
        "┃ │                                            │ ┃",
        "┃ │ Make a move by choosing the start and end  │ ┃",
        "┃ │ of a move. <Space> to abort a move. <R> to │ ┃",
        "┃ │ revert the last move.                      │ ┃",
        "┃ │                                            │ ┃",
        "┃ │                                            │ ┃",
        "┃ │                                            │ ┃",
        "┃ │                                            │ ┃",
        "┃ │                                            │ ┃",
        "┃ │                                            │ ┃",
        "┃ │                                            │ ┃",
        "┃ └─────────────── Close <Esc> ────────────────┘ ┃",
        "┗━━━━━━━━━━━ Help <F1> Quit <CTRL-q> ━━━━━━━━━━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style = Style::new().blue();
        let key_style_bold = Style::new().blue().bold();
        expected.set_style(Rect::new(18, 0, 13, 1), title_style);
        expected.set_style(Rect::new(4, 2, 5, 1), key_style);
        expected.set_style(Rect::new(4, 3, 4, 1), key_style);
        expected.set_style(Rect::new(4, 4, 4, 1), key_style);
        expected.set_style(Rect::new(4, 5, 3, 1), key_style);

        expected.set_style(Rect::new(4, 7, 15, 1), key_style);
        expected.set_style(Rect::new(4, 8, 15, 1), key_style);
        expected.set_style(Rect::new(4, 9, 15, 1), key_style);
        expected.set_style(Rect::new(4, 10, 15, 1), key_style);

        expected.set_style(Rect::new(15, 13, 7, 1), key_style);
        expected.set_style(Rect::new(40, 13, 3, 1), key_style);

        expected.set_style(Rect::new(25, 22, 6, 1), key_style_bold);
        expected.set_style(Rect::new(18, 23, 4, 1), key_style_bold);
        expected.set_style(Rect::new(28, 23, 9, 1), key_style_bold);
    }

    assert_eq!(buf, expected);

    app.handle_key_event(KeyCode::Esc.into());
    app.handle_key_event(KeyCode::F(2).into());
    app.handle_key_event(KeyCode::F(1).into());

    app.render(buf.area, &mut buf);
    assert_eq!(buf, expected);
}

#[test]
fn handle_key_event_about_modal() {
    let mut app = helper::setup_app();
    app.handle_key_event(KeyCode::F(12).into());
    assert_eq!(app.app_state, AppState::AboutModal { scroll: 0 });

    app.handle_key_event(KeyCode::Down.into());
    assert_eq!(app.app_state, AppState::AboutModal { scroll: 1 });

    app.handle_key_event(KeyCode::Esc.into());
    assert_eq!(app.app_state, AppState::Base);

    app.handle_key_event(KeyCode::F(12).into());
    assert_eq!(app.app_state, AppState::AboutModal { scroll: 0 });

    app.handle_key_event(KeyCode::Down.into());
    assert_eq!(app.app_state, AppState::AboutModal { scroll: 1 });

    app.handle_key_event(KeyCode::Up.into());
    assert_eq!(app.app_state, AppState::AboutModal { scroll: 0 });

    let mut key: KeyEvent = KeyCode::Char('q').into();
    key.modifiers = KeyModifiers::CONTROL;
    app.handle_key_event(key);
    assert_eq!(app.app_state, AppState::Exit);
}

#[test]
fn switch_help_about() {
    let mut app = helper::setup_app();
    app.handle_key_event(KeyCode::F(1).into());
    assert_eq!(app.app_state, AppState::HelpModal);

    app.handle_key_event(KeyCode::F(12).into());
    assert_eq!(app.app_state, AppState::AboutModal { scroll: 0 });

    app.handle_key_event(KeyCode::Esc.into());
    assert_eq!(app.app_state, AppState::Base);

    app.handle_key_event(KeyCode::F(12).into());
    assert_eq!(app.app_state, AppState::AboutModal { scroll: 0 });

    app.handle_key_event(KeyCode::F(1).into());
    assert_eq!(app.app_state, AppState::HelpModal);
}

#[test]
fn render_about_modal() {
    let mut app = helper::setup_app();
    let mut buf0 = Buffer::empty(Rect::new(0, 0, 100, 24));
    app.handle_key_event(KeyCode::F(12).into());

    app.render(buf0.area, &mut buf0);

    let mut expected0 = Buffer::with_lines(vec![
        "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ RSLibreCell ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓",
        "┃ ┌─────────────────────────────────────────── About ────────────────────────────────────────────┐ ┃",
        "┃ │ RSLibreCell - a FreeCell implementation                                                      │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │ Copyright (c) tristhaus 2025 and later                                                       │ ┃",
        "┃ │ https://www.github.com/tristhaus/rslibrecell                                                 │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │ For help, press <F1>.                                                                        │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │ RSLibreCell is free, libre, open-source software. License text below:                        │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │  GNU GENERAL PUBLIC LICENSE                                                                  │ ┃",
        "┃ │  Version 3, 29 June 2007                                                                     │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │  Copyright (C) 2007 Free Software Foundation, Inc. <https://fsf.org/> Everyone is permitted  │ ┃",
        "┃ │ to copy and distribute verbatim copies of this license document, but changing it is not      │ ┃",
        "┃ │ allowed.                                                                                     │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │ Preamble                                                                                     │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │   The GNU General Public License is a free, copyleft license for software and other kinds of │ ┃",
        "┃ │ works.                                                                                       │ ┃",
        "┃ └─────────────────────────────── Scroll <Up><Down> Close <Esc> ────────────────────────────────┘ ┃",
        "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Help <F1> Quit <CTRL-q> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style = Style::new().blue();
        let key_style_bold = Style::new().blue().bold();
        expected0.set_style(Rect::new(43, 0, 13, 1), title_style);
        expected0.set_style(Rect::new(4, 2, 39, 1), title_style);

        expected0.set_style(Rect::new(20, 7, 4, 1), key_style);

        expected0.set_style(Rect::new(42, 22, 10, 1), key_style_bold);
        expected0.set_style(Rect::new(59, 22, 6, 1), key_style_bold);
        expected0.set_style(Rect::new(43, 23, 4, 1), key_style_bold);
        expected0.set_style(Rect::new(53, 23, 9, 1), key_style_bold);
    }

    assert_eq!(buf0, expected0);

    app.handle_key_event(KeyCode::Esc.into());
    app.handle_key_event(KeyCode::F(2).into());
    app.handle_key_event(KeyCode::F(12).into());

    app.render(buf0.area, &mut buf0);
    assert_eq!(buf0, expected0);

    let mut buf1 = Buffer::empty(Rect::new(0, 0, 100, 24));

    app.handle_key_event(KeyCode::Down.into());
    app.render(buf1.area, &mut buf1);

    let mut expected1 = Buffer::with_lines(vec![
        "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ RSLibreCell ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓",
        "┃ ┌─────────────────────────────────────────── About ────────────────────────────────────────────┐ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │ Copyright (c) tristhaus 2025 and later                                                       │ ┃",
        "┃ │ https://www.github.com/tristhaus/rslibrecell                                                 │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │ For help, press <F1>.                                                                        │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │ RSLibreCell is free, libre, open-source software. License text below:                        │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │  GNU GENERAL PUBLIC LICENSE                                                                  │ ┃",
        "┃ │  Version 3, 29 June 2007                                                                     │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │  Copyright (C) 2007 Free Software Foundation, Inc. <https://fsf.org/> Everyone is permitted  │ ┃",
        "┃ │ to copy and distribute verbatim copies of this license document, but changing it is not      │ ┃",
        "┃ │ allowed.                                                                                     │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │ Preamble                                                                                     │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │   The GNU General Public License is a free, copyleft license for software and other kinds of │ ┃",
        "┃ │ works.                                                                                       │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ └─────────────────────────────── Scroll <Up><Down> Close <Esc> ────────────────────────────────┘ ┃",
        "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Help <F1> Quit <CTRL-q> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style = Style::new().blue();
        let key_style_bold = Style::new().blue().bold();
        expected1.set_style(Rect::new(43, 0, 13, 1), title_style);

        expected1.set_style(Rect::new(20, 6, 4, 1), key_style);

        expected1.set_style(Rect::new(42, 22, 10, 1), key_style_bold);
        expected1.set_style(Rect::new(59, 22, 6, 1), key_style_bold);
        expected1.set_style(Rect::new(43, 23, 4, 1), key_style_bold);
        expected1.set_style(Rect::new(53, 23, 9, 1), key_style_bold);
    }

    assert_eq!(buf1, expected1);

    for _ in 0..9999 {
        app.handle_key_event(KeyCode::Down.into());
    }

    let mut buf2 = Buffer::empty(Rect::new(0, 0, 100, 24));

    app.render(buf2.area, &mut buf2);

    let mut expected2 = Buffer::with_lines(vec![
        "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ RSLibreCell ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓",
        "┃ ┌─────────────────────────────────────────── About ────────────────────────────────────────────┐ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │   16. Limitation of Liability.                                                               │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │   IN NO EVENT UNLESS REQUIRED BY APPLICABLE LAW OR AGREED TO IN WRITING WILL ANY COPYRIGHT   │ ┃",
        "┃ │ HOLDER, OR ANY OTHER PARTY WHO MODIFIES AND/OR CONVEYS THE PROGRAM AS PERMITTED ABOVE, BE    │ ┃",
        "┃ │ LIABLE TO YOU FOR DAMAGES, INCLUDING ANY GENERAL, SPECIAL, INCIDENTAL OR CONSEQUENTIAL       │ ┃",
        "┃ │ DAMAGES ARISING OUT OF THE USE OR INABILITY TO USE THE PROGRAM (INCLUDING BUT NOT LIMITED TO │ ┃",
        "┃ │ LOSS OF DATA OR DATA BEING RENDERED INACCURATE OR LOSSES SUSTAINED BY YOU OR THIRD PARTIES   │ ┃",
        "┃ │ OR A FAILURE OF THE PROGRAM TO OPERATE WITH ANY OTHER PROGRAMS), EVEN IF SUCH HOLDER OR      │ ┃",
        "┃ │ OTHER PARTY HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.                             │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │   17. Interpretation of Sections 15 and 16.                                                  │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │   If the disclaimer of warranty and limitation of liability provided above cannot be given   │ ┃",
        "┃ │ local legal effect according to their terms, reviewing courts shall apply local law that     │ ┃",
        "┃ │ most closely approximates an absolute waiver of all civil liability in connection with the   │ ┃",
        "┃ │ Program, unless a warranty or assumption of liability accompanies a copy of the Program in   │ ┃",
        "┃ │ return for a fee.                                                                            │ ┃",
        "┃ │                                                                                              │ ┃",
        "┃ │ END OF TERMS AND CONDITIONS                                                                  │ ┃",
        "┃ └─────────────────────────────── Scroll <Up><Down> Close <Esc> ────────────────────────────────┘ ┃",
        "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Help <F1> Quit <CTRL-q> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style_bold = Style::new().blue().bold();
        expected2.set_style(Rect::new(43, 0, 13, 1), title_style);

        expected2.set_style(Rect::new(42, 22, 10, 1), key_style_bold);
        expected2.set_style(Rect::new(59, 22, 6, 1), key_style_bold);
        expected2.set_style(Rect::new(43, 23, 4, 1), key_style_bold);
        expected2.set_style(Rect::new(53, 23, 9, 1), key_style_bold);
    }

    assert_eq!(buf2, expected2);

    app.handle_key_event(KeyCode::Esc.into());
    app.handle_key_event(KeyCode::F(12).into());

    app.render(buf0.area, &mut buf0);
    assert_eq!(buf0, expected0);
}

#[test]
fn handle_key_event_selection_id_modal_valid() {
    const EMPTY_ENTRY_STATE: AppState = AppState::SelectionIdModal {
        id: [SPACE_ASCII_CODE; 5],
    };

    let mut app = helper::setup_app();
    app.handle_key_event(KeyCode::F(3).into());
    assert_eq!(app.app_state, EMPTY_ENTRY_STATE);

    app.handle_key_event(KeyCode::Esc.into());
    assert_eq!(app.app_state, AppState::Base);

    app.handle_key_event(KeyCode::F(3).into());
    app.handle_key_event(KeyCode::Char('1').into());
    app.handle_key_event(KeyCode::Char('9').into());
    assert_eq!(
        app.app_state,
        AppState::SelectionIdModal {
            id: [
                SPACE_ASCII_CODE,
                SPACE_ASCII_CODE,
                SPACE_ASCII_CODE,
                0x31,
                0x39
            ]
        }
    );

    app.handle_key_event(KeyCode::Backspace.into());
    assert_eq!(
        app.app_state,
        AppState::SelectionIdModal {
            id: [
                SPACE_ASCII_CODE,
                SPACE_ASCII_CODE,
                SPACE_ASCII_CODE,
                SPACE_ASCII_CODE,
                0x31
            ]
        }
    );

    app.handle_key_event(KeyCode::Char('2').into());
    app.handle_key_event(KeyCode::Char('3').into());
    assert_eq!(
        app.app_state,
        AppState::SelectionIdModal {
            id: [SPACE_ASCII_CODE, SPACE_ASCII_CODE, 0x31, 0x32, 0x33]
        }
    );

    app.handle_key_event(KeyCode::Enter.into());
    assert_eq!(app.app_state, AppState::Base);
    assert_eq!(123, app.game_handler.game.as_ref().unwrap().id);

    app.handle_key_event(KeyCode::F(3).into());
    assert_eq!(app.app_state, EMPTY_ENTRY_STATE);

    let mut key: KeyEvent = KeyCode::Char('q').into();
    key.modifiers = KeyModifiers::CONTROL;
    app.handle_key_event(key);
    assert_eq!(app.app_state, AppState::Exit);
}

// invalid data for the selection by id rejects the <ENTER> key
#[test]
fn handle_key_event_selection_id_modal_invalid() {
    const EMPTY_ENTRY_STATE: AppState = AppState::SelectionIdModal {
        id: [SPACE_ASCII_CODE; 5],
    };

    let mut app = helper::setup_app();

    app.handle_key_event(KeyCode::F(3).into());
    assert_eq!(app.app_state, EMPTY_ENTRY_STATE);

    // no data entered
    app.handle_key_event(KeyCode::Enter.into());
    assert_eq!(app.app_state, EMPTY_ENTRY_STATE);

    // invalid chars
    app.handle_key_event(KeyCode::Char('a').into());
    app.handle_key_event(KeyCode::Char('-').into());
    assert_eq!(app.app_state, EMPTY_ENTRY_STATE);

    // id 0 is invalid
    app.handle_key_event(KeyCode::Char('0').into());
    assert_eq!(
        app.app_state,
        AppState::SelectionIdModal {
            id: [
                SPACE_ASCII_CODE,
                SPACE_ASCII_CODE,
                SPACE_ASCII_CODE,
                SPACE_ASCII_CODE,
                0x30
            ]
        }
    );
    app.handle_key_event(KeyCode::Enter.into());
    assert_eq!(
        app.app_state,
        AppState::SelectionIdModal {
            id: [
                SPACE_ASCII_CODE,
                SPACE_ASCII_CODE,
                SPACE_ASCII_CODE,
                SPACE_ASCII_CODE,
                0x30
            ]
        }
    );

    // reset modal
    app.handle_key_event(KeyCode::Esc.into());
    app.handle_key_event(KeyCode::F(3).into());

    // id 64001 is invalid
    app.handle_key_event(KeyCode::Char('6').into());
    app.handle_key_event(KeyCode::Char('4').into());
    app.handle_key_event(KeyCode::Char('0').into());
    app.handle_key_event(KeyCode::Char('0').into());
    app.handle_key_event(KeyCode::Char('1').into());
    assert_eq!(
        app.app_state,
        AppState::SelectionIdModal {
            id: [0x36, 0x34, 0x30, 0x30, 0x31]
        }
    );
    app.handle_key_event(KeyCode::Enter.into());
    assert_eq!(
        app.app_state,
        AppState::SelectionIdModal {
            id: [0x36, 0x34, 0x30, 0x30, 0x31]
        }
    );

    // reset modal
    app.handle_key_event(KeyCode::Esc.into());
    app.handle_key_event(KeyCode::F(3).into());

    // id outside of u16 does not panic
    app.handle_key_event(KeyCode::Char('7').into());
    app.handle_key_event(KeyCode::Char('0').into());
    app.handle_key_event(KeyCode::Char('7').into());
    app.handle_key_event(KeyCode::Char('8').into());
    app.handle_key_event(KeyCode::Char('9').into());
    assert_eq!(
        app.app_state,
        AppState::SelectionIdModal {
            id: [0x37, 0x30, 0x37, 0x38, 0x39]
        }
    );
    app.handle_key_event(KeyCode::Enter.into());
    assert_eq!(
        app.app_state,
        AppState::SelectionIdModal {
            id: [0x37, 0x30, 0x37, 0x38, 0x39]
        }
    );
}

#[test]
fn render_selection_id_modal_valid() {
    let mut app = helper::setup_app();
    let mut buf = Buffer::empty(Rect::new(0, 0, 32, 24));
    app.handle_key_event(KeyCode::F(3).into());

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━ RSLibreCell ━━━━━━━━━┓",
        "┃ ┌─── Choose game by ID ────┐ ┃",
        "┃ │                          │ ┃",
        "┃ │         Enter ID:        │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ └Accept <Enter> Abort <Esc>┘ ┃",
        "┗━━ Help <F1> Quit <CTRL-q> ━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style_bold = Style::new().blue().bold();
        let input_style = Style::new().underlined();
        expected.set_style(Rect::new(9, 0, 13, 1), title_style);

        expected.set_style(Rect::new(14, 4, 5, 1), input_style);

        expected.set_style(Rect::new(10, 22, 7, 1), key_style_bold);
        expected.set_style(Rect::new(24, 22, 5, 1), key_style_bold);
        expected.set_style(Rect::new(9, 23, 4, 1), key_style_bold);
        expected.set_style(Rect::new(19, 23, 9, 1), key_style_bold);
    }

    assert_eq!(buf, expected);

    app.handle_key_event(KeyCode::Char('1').into());
    app.handle_key_event(KeyCode::Char('2').into());
    app.handle_key_event(KeyCode::Char('3').into());

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━ RSLibreCell ━━━━━━━━━┓",
        "┃ ┌─── Choose game by ID ────┐ ┃",
        "┃ │                          │ ┃",
        "┃ │         Enter ID:        │ ┃",
        "┃ │             123          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ └Accept <Enter> Abort <Esc>┘ ┃",
        "┗━━ Help <F1> Quit <CTRL-q> ━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style_bold = Style::new().blue().bold();
        let input_style = Style::new().underlined();
        expected.set_style(Rect::new(9, 0, 13, 1), title_style);

        expected.set_style(Rect::new(14, 4, 5, 1), input_style);

        expected.set_style(Rect::new(10, 22, 7, 1), key_style_bold);
        expected.set_style(Rect::new(24, 22, 5, 1), key_style_bold);
        expected.set_style(Rect::new(9, 23, 4, 1), key_style_bold);
        expected.set_style(Rect::new(19, 23, 9, 1), key_style_bold);
    }

    assert_eq!(buf, expected);

    app.handle_key_event(KeyCode::Char('9').into());
    app.handle_key_event(KeyCode::Char('8').into());
    app.handle_key_event(KeyCode::Char('7').into());

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━ RSLibreCell ━━━━━━━━━┓",
        "┃ ┌─── Choose game by ID ────┐ ┃",
        "┃ │                          │ ┃",
        "┃ │         Enter ID:        │ ┃",
        "┃ │           12398          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ └Accept <Enter> Abort <Esc>┘ ┃",
        "┗━━ Help <F1> Quit <CTRL-q> ━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style_bold = Style::new().blue().bold();
        let input_style = Style::new().underlined();
        expected.set_style(Rect::new(9, 0, 13, 1), title_style);

        expected.set_style(Rect::new(14, 4, 5, 1), input_style);

        expected.set_style(Rect::new(10, 22, 7, 1), key_style_bold);
        expected.set_style(Rect::new(24, 22, 5, 1), key_style_bold);
        expected.set_style(Rect::new(9, 23, 4, 1), key_style_bold);
        expected.set_style(Rect::new(19, 23, 9, 1), key_style_bold);
    }

    assert_eq!(buf, expected);
}

#[test]
fn render_selection_id_modal_invalid() {
    let mut app = helper::setup_app();
    let mut buf = Buffer::empty(Rect::new(0, 0, 32, 24));
    app.handle_key_event(KeyCode::F(3).into());
    app.handle_key_event(KeyCode::Char('0').into());

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━ RSLibreCell ━━━━━━━━━┓",
        "┃ ┌─── Choose game by ID ────┐ ┃",
        "┃ │                          │ ┃",
        "┃ │         Enter ID:        │ ┃",
        "┃ │               0          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ └Accept <Enter> Abort <Esc>┘ ┃",
        "┗━━ Help <F1> Quit <CTRL-q> ━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style_bold = Style::new().blue().bold();
        let input_style_invalud = Style::new().underlined().red();
        expected.set_style(Rect::new(9, 0, 13, 1), title_style);

        expected.set_style(Rect::new(14, 4, 5, 1), input_style_invalud);

        expected.set_style(Rect::new(10, 22, 7, 1), key_style_bold);
        expected.set_style(Rect::new(24, 22, 5, 1), key_style_bold);
        expected.set_style(Rect::new(9, 23, 4, 1), key_style_bold);
        expected.set_style(Rect::new(19, 23, 9, 1), key_style_bold);
    }

    assert_eq!(buf, expected);

    app.handle_key_event(KeyCode::Backspace.into());
    app.handle_key_event(KeyCode::Char('6').into());
    app.handle_key_event(KeyCode::Char('4').into());
    app.handle_key_event(KeyCode::Char('0').into());
    app.handle_key_event(KeyCode::Char('0').into());
    app.handle_key_event(KeyCode::Char('1').into());

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━ RSLibreCell ━━━━━━━━━┓",
        "┃ ┌─── Choose game by ID ────┐ ┃",
        "┃ │                          │ ┃",
        "┃ │         Enter ID:        │ ┃",
        "┃ │           64001          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ └Accept <Enter> Abort <Esc>┘ ┃",
        "┗━━ Help <F1> Quit <CTRL-q> ━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style_bold = Style::new().blue().bold();
        let input_style_invalud = Style::new().underlined().red();
        expected.set_style(Rect::new(9, 0, 13, 1), title_style);

        expected.set_style(Rect::new(14, 4, 5, 1), input_style_invalud);

        expected.set_style(Rect::new(10, 22, 7, 1), key_style_bold);
        expected.set_style(Rect::new(24, 22, 5, 1), key_style_bold);
        expected.set_style(Rect::new(9, 23, 4, 1), key_style_bold);
        expected.set_style(Rect::new(19, 23, 9, 1), key_style_bold);
    }

    assert_eq!(buf, expected);
}

#[test]
fn handle_key_event_selection_journey_modal() {
    let mut mock = MockPersistJourney::new();
    mock.expect_read().return_const((123, vec![117, 118]));
    mock.expect_write().return_const(());
    let mut app = App::new(mock);

    // open modal
    app.handle_key_event(KeyCode::Char('!').into());
    assert_eq!(AppState::SelectionJourneyModal, app.app_state);

    // start next game
    app.handle_key_event(KeyCode::Char('1').into());
    assert_eq!(AppState::Base, app.app_state);
    assert!(app.game_handler.game.as_ref().is_some_and(|x| x.id == 123));

    // start second skipped game
    app.handle_key_event(KeyCode::Char('!').into());
    app.handle_key_event(KeyCode::Char('3').into());
    assert_eq!(AppState::Base, app.app_state);
    assert!(app.game_handler.game.as_ref().is_some_and(|x| x.id == 118));

    // skip another game
    app.handle_key_event(KeyCode::Char('!').into());
    app.handle_key_event(KeyCode::Char('s').into());
    app.handle_key_event(KeyCode::Char('4').into());
    assert_eq!(AppState::Base, app.app_state);
    assert!(app.game_handler.game.as_ref().is_some_and(|x| x.id == 123));

    // after skip, new next game is there
    app.handle_key_event(KeyCode::Char('!').into());
    app.handle_key_event(KeyCode::Char('1').into());
    assert_eq!(AppState::Base, app.app_state);
    assert!(app.game_handler.game.as_ref().is_some_and(|x| x.id == 124));

    // ignore key when no skipped game is attached to it
    app.handle_key_event(KeyCode::Char('!').into());
    app.handle_key_event(KeyCode::Char('5').into());
    assert_eq!(AppState::SelectionJourneyModal, app.app_state);
}

#[test]
fn make_journey_handle_next_won_game() {
    let mut mock = MockPersistJourney::new();
    mock.expect_read().once().return_const((100, vec![55, 66]));
    mock.expect_write()
        .with(predicate::eq(101), predicate::eq(vec![55, 66]))
        .once()
        .return_const(());
    let mut app = App::new(mock);

    app.selection_through_journey_start_next();

    let mut punch_key = |key: char| {
        app.handle_key_event(KeyCode::Char(key).into());
    };

    {
        punch_key('l');
        punch_key('s');

        punch_key('l');
        punch_key('q');

        punch_key('a');
        punch_key('l');

        punch_key('j');
        punch_key('l');

        punch_key('l');
        punch_key('j');

        punch_key('a');
        punch_key('i');

        punch_key('a');
        punch_key('j');

        punch_key('a');
        punch_key('l');

        punch_key('a');
        punch_key('w');

        punch_key('l');
        punch_key('a');

        punch_key('l');
        punch_key('a');

        punch_key('j');
        punch_key('a');

        punch_key('ö');
        punch_key('e');

        punch_key('s');
        punch_key('l');

        punch_key('ö');
        punch_key('r');

        punch_key('k');
        punch_key('d');

        punch_key('f');
        punch_key('k');

        punch_key('f');
        punch_key('e');

        punch_key('j');
        punch_key('ö');

        punch_key('f');
        punch_key('ö');

        punch_key('j');
        punch_key('ö');

        punch_key('k');
        punch_key('ö');

        punch_key('e');
        punch_key('f');

        punch_key('r');
        punch_key('f');

        punch_key('d');
        punch_key('j');

        punch_key('d');
        punch_key('e');

        punch_key('d');
        punch_key('l');

        punch_key('d');
        punch_key('f');

        punch_key('d');
        punch_key('r');

        punch_key('k');
        punch_key('d');

        punch_key('s');
        punch_key('q');
    }
}

#[test]
fn make_journey_handle_skipped_won_game() {
    let mut mock = MockPersistJourney::new();
    mock.expect_read().once().return_const((121, vec![44, 100]));
    mock.expect_write()
        .with(predicate::eq(121), predicate::eq(vec![44]))
        .once()
        .return_const(());
    let mut app = App::new(mock);

    app.selection_through_journey_start_skipped('3');

    let mut punch_key = |key: char| {
        app.handle_key_event(KeyCode::Char(key).into());
    };

    {
        punch_key('l');
        punch_key('s');

        punch_key('l');
        punch_key('q');

        punch_key('a');
        punch_key('l');

        punch_key('j');
        punch_key('l');

        punch_key('l');
        punch_key('j');

        punch_key('a');
        punch_key('i');

        punch_key('a');
        punch_key('j');

        punch_key('a');
        punch_key('l');

        punch_key('a');
        punch_key('w');

        punch_key('l');
        punch_key('a');

        punch_key('l');
        punch_key('a');

        punch_key('j');
        punch_key('a');

        punch_key('ö');
        punch_key('e');

        punch_key('s');
        punch_key('l');

        punch_key('ö');
        punch_key('r');

        punch_key('k');
        punch_key('d');

        punch_key('f');
        punch_key('k');

        punch_key('f');
        punch_key('e');

        punch_key('j');
        punch_key('ö');

        punch_key('f');
        punch_key('ö');

        punch_key('j');
        punch_key('ö');

        punch_key('k');
        punch_key('ö');

        punch_key('e');
        punch_key('f');

        punch_key('r');
        punch_key('f');

        punch_key('d');
        punch_key('j');

        punch_key('d');
        punch_key('e');

        punch_key('d');
        punch_key('l');

        punch_key('d');
        punch_key('f');

        punch_key('d');
        punch_key('r');

        punch_key('k');
        punch_key('d');

        punch_key('s');
        punch_key('q');
    }
}

#[test]
fn render_selection_journey_modal_no_skipped() {
    let mut mock = MockPersistJourney::new();
    mock.expect_read().return_const((1, vec![]));
    mock.expect_write().return_const(());
    let mut app = App::new(mock);

    let mut buf = Buffer::empty(Rect::new(0, 0, 32, 24));
    app.handle_key_event(KeyCode::Char('!').into());

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━ RSLibreCell ━━━━━━━━━┓",
        "┃ ┌──────── Journey ─────────┐ ┃",
        "┃ │ Press number to start:   │ ┃",
        "┃ │ <1> (next game) :     1  │ ┃",
        "┃ │ <s> to skip for now      │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ └────── Close <Esc> ───────┘ ┃",
        "┗━━ Help <F1> Quit <CTRL-q> ━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style_bold = Style::new().blue().bold();
        expected.set_style(Rect::new(9, 0, 13, 1), title_style);

        expected.set_style(Rect::new(4, 3, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 4, 3, 1), key_style_bold);

        expected.set_style(Rect::new(16, 22, 6, 1), key_style_bold);
        expected.set_style(Rect::new(9, 23, 4, 1), key_style_bold);
        expected.set_style(Rect::new(19, 23, 9, 1), key_style_bold);
    }

    assert_eq!(buf, expected);
}

#[test]
fn render_selection_journey_modal_no_next_game() {
    let mut mock = MockPersistJourney::new();
    mock.expect_read()
        .return_const((64001, vec![111, 222, 12345]));
    mock.expect_write().return_const(());
    let mut app = App::new(mock);

    let mut buf = Buffer::empty(Rect::new(0, 0, 32, 24));
    app.handle_key_event(KeyCode::Char('!').into());

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━ RSLibreCell ━━━━━━━━━┓",
        "┃ ┌──────── Journey ─────────┐ ┃",
        "┃ │ Press number to start:   │ ┃",
        "┃ │ Previously skipped games │ ┃",
        "┃ │ <2> :   111              │ ┃",
        "┃ │ <3> :   222              │ ┃",
        "┃ │ <4> : 12345              │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ └────── Close <Esc> ───────┘ ┃",
        "┗━━ Help <F1> Quit <CTRL-q> ━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style_bold = Style::new().blue().bold();
        expected.set_style(Rect::new(9, 0, 13, 1), title_style);

        expected.set_style(Rect::new(4, 4, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 5, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 6, 3, 1), key_style_bold);

        expected.set_style(Rect::new(16, 22, 6, 1), key_style_bold);
        expected.set_style(Rect::new(9, 23, 4, 1), key_style_bold);
        expected.set_style(Rect::new(19, 23, 9, 1), key_style_bold);
    }

    assert_eq!(buf, expected);
}

#[test]
fn render_selection_journey_modal_next_game_and_eight_skipped_games() {
    let mut mock = MockPersistJourney::new();
    mock.expect_read()
        .return_const((23442, vec![111, 222, 333, 444, 555, 666, 777, 888]));
    mock.expect_write().return_const(());
    let mut app = App::new(mock);

    let mut buf = Buffer::empty(Rect::new(0, 0, 32, 24));
    app.handle_key_event(KeyCode::Char('!').into());

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━ RSLibreCell ━━━━━━━━━┓",
        "┃ ┌──────── Journey ─────────┐ ┃",
        "┃ │ Press number to start:   │ ┃",
        "┃ │ <1> (next game) : 23442  │ ┃",
        "┃ │ <s> to skip for now      │ ┃",
        "┃ │                          │ ┃",
        "┃ │ Previously skipped games │ ┃",
        "┃ │ <2> :   111              │ ┃",
        "┃ │ <3> :   222              │ ┃",
        "┃ │ <4> :   333              │ ┃",
        "┃ │ <5> :   444              │ ┃",
        "┃ │ <6> :   555              │ ┃",
        "┃ │ <7> :   666              │ ┃",
        "┃ │ <8> :   777              │ ┃",
        "┃ │ <9> :   888              │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ └────── Close <Esc> ───────┘ ┃",
        "┗━━ Help <F1> Quit <CTRL-q> ━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style_bold = Style::new().blue().bold();
        expected.set_style(Rect::new(9, 0, 13, 1), title_style);

        expected.set_style(Rect::new(4, 3, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 4, 3, 1), key_style_bold);

        expected.set_style(Rect::new(4, 7, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 8, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 9, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 10, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 11, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 12, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 13, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 14, 3, 1), key_style_bold);

        expected.set_style(Rect::new(16, 22, 6, 1), key_style_bold);
        expected.set_style(Rect::new(9, 23, 4, 1), key_style_bold);
        expected.set_style(Rect::new(19, 23, 9, 1), key_style_bold);
    }

    assert_eq!(buf, expected);
}

#[test]
fn render_selection_journey_modal_next_game_and_many_skipped_games() {
    let mut mock = MockPersistJourney::new();
    mock.expect_read()
        .return_const((23442, vec![111, 222, 333, 444, 555, 666, 777, 888, 999]));
    mock.expect_write().return_const(());
    let mut app = App::new(mock);

    let mut buf = Buffer::empty(Rect::new(0, 0, 32, 24));
    app.handle_key_event(KeyCode::Char('!').into());

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━ RSLibreCell ━━━━━━━━━┓",
        "┃ ┌──────── Journey ─────────┐ ┃",
        "┃ │ Press number to start:   │ ┃",
        "┃ │ <1> (next game) : 23442  │ ┃",
        "┃ │ <s> to skip for now      │ ┃",
        "┃ │                          │ ┃",
        "┃ │ Previously skipped games │ ┃",
        "┃ │ <2> :   111              │ ┃",
        "┃ │ <3> :   222              │ ┃",
        "┃ │ <4> :   333              │ ┃",
        "┃ │ <5> :   444              │ ┃",
        "┃ │ <6> :   555              │ ┃",
        "┃ │ <7> :   666              │ ┃",
        "┃ │ <8> :   777              │ ┃",
        "┃ │ <9> :   888              │ ┃",
        "┃ │                          │ ┃",
        "┃ │ ... more skipped games   │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ └────── Close <Esc> ───────┘ ┃",
        "┗━━ Help <F1> Quit <CTRL-q> ━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style_bold = Style::new().blue().bold();
        expected.set_style(Rect::new(9, 0, 13, 1), title_style);

        expected.set_style(Rect::new(4, 3, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 4, 3, 1), key_style_bold);

        expected.set_style(Rect::new(4, 7, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 8, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 9, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 10, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 11, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 12, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 13, 3, 1), key_style_bold);
        expected.set_style(Rect::new(4, 14, 3, 1), key_style_bold);

        expected.set_style(Rect::new(16, 22, 6, 1), key_style_bold);
        expected.set_style(Rect::new(9, 23, 4, 1), key_style_bold);
        expected.set_style(Rect::new(19, 23, 9, 1), key_style_bold);
    }

    assert_eq!(buf, expected);
}

#[test]
fn render_selection_journey_modal_completed() {
    let mut mock = MockPersistJourney::new();
    mock.expect_read().return_const((64001, vec![]));
    mock.expect_write().return_const(());
    let mut app = App::new(mock);

    let mut buf = Buffer::empty(Rect::new(0, 0, 32, 24));
    app.handle_key_event(KeyCode::Char('!').into());

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━ RSLibreCell ━━━━━━━━━┓",
        "┃ ┌──────── Journey ─────────┐ ┃",
        "┃ │                          │ ┃",
        "┃ │    Journey completed!    │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ │                          │ ┃",
        "┃ └────── Close <Esc> ───────┘ ┃",
        "┗━━ Help <F1> Quit <CTRL-q> ━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style_bold = Style::new().blue().bold();
        expected.set_style(Rect::new(9, 0, 13, 1), title_style);

        expected.set_style(Rect::new(7, 3, 18, 1), title_style);

        expected.set_style(Rect::new(16, 22, 6, 1), key_style_bold);
        expected.set_style(Rect::new(9, 23, 4, 1), key_style_bold);
        expected.set_style(Rect::new(19, 23, 9, 1), key_style_bold);
    }

    assert_eq!(buf, expected);
}

mod helper {
    use super::*;

    pub fn setup_app() -> App<MockPersistJourney> {
        let mut mock = MockPersistJourney::new();
        mock.expect_read().return_const((123, vec![117, 118]));
        mock.expect_write().return_const(());
        App::new(mock)
    }
}
