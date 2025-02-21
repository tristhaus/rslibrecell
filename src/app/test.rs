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
use ratatui::style::Style;

#[test]
fn render_startup() {
    let mut app = App::new();
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
    let mut app = App::new();
    let mut buf = Buffer::empty(Rect::new(0, 0, 31, 24));

    app.render(buf.area, &mut buf);
}

#[test]
#[should_panic]
fn render_too_flat_should_panic() {
    let mut app = App::new();
    let mut buf = Buffer::empty(Rect::new(0, 0, 32, 23));

    app.render(buf.area, &mut buf);
}

#[test]
fn handle_key_event_random_game() {
    let mut app = App::new();
    app.handle_key_event(KeyCode::F(2).into());
    assert_eq!(app.app_state, AppState::Base);
}

#[test]
fn render_random_games() {
    let mut app = App::new();
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
    let mut app = App::new();
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
    let mut app = App::new();
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
fn handle_key_event_quit() {
    let mut app = App::new();
    let mut key: KeyEvent = KeyCode::Char('q').into();
    key.modifiers = KeyModifiers::CONTROL;
    app.handle_key_event(key);
    assert_eq!(app.app_state, AppState::Exit);
}

#[test]
fn handle_key_event_help_modal() {
    let mut app = App::new();
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
    let mut app = App::new();
    let mut buf = Buffer::empty(Rect::new(0, 0, 50, 24));
    app.handle_key_event(KeyCode::F(1).into());

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━━━━━━━━━━ RSLibreCell ━━━━━━━━━━━━━━━━━━┓",
        "┃ ┌─────────────────── Help ───────────────────┐ ┃",
        "┃ │ <F12> to show the About box.               │ ┃",
        "┃ │ <F2> to start a new random game.           │ ┃",
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

        expected.set_style(Rect::new(4, 5, 15, 1), key_style);
        expected.set_style(Rect::new(4, 6, 15, 1), key_style);
        expected.set_style(Rect::new(4, 7, 15, 1), key_style);
        expected.set_style(Rect::new(4, 8, 15, 1), key_style);

        expected.set_style(Rect::new(15, 11, 7, 1), key_style);
        expected.set_style(Rect::new(40, 11, 3, 1), key_style);

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
    let mut app = App::new();
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
    let mut app = App::new();
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
    let mut app = App::new();
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
