use super::*;
use crossterm::event::KeyModifiers;
use ratatui::style::Style;

#[test]
fn render_startup() {
    let app = App::new();
    let mut buf = Buffer::empty(Rect::new(0, 0, 50, 15));

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
        "┗━━━━━━━━━━━ Help <F1> Quit <CTRL-q> ━━━━━━━━━━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(18, 0, 13, 1), title_style);
        expected.set_style(Rect::new(18, 14, 4, 1), key_style);
        expected.set_style(Rect::new(28, 14, 9, 1), key_style);
    }

    assert_eq!(buf, expected);
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
    let mut buf0 = Buffer::empty(Rect::new(0, 0, 50, 15));
    let mut buf1 = Buffer::empty(Rect::new(0, 0, 50, 15));
    let mut buf2 = Buffer::empty(Rect::new(0, 0, 50, 15));

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
        "┗━━━━━━━━━━━ Help <F1> Quit <CTRL-q> ━━━━━━━━━━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style = Style::new().blue().bold();
        initial_empty.set_style(Rect::new(18, 0, 13, 1), title_style);
        initial_empty.set_style(Rect::new(18, 14, 4, 1), key_style);
        initial_empty.set_style(Rect::new(28, 14, 9, 1), key_style);
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
    let mut buf = Buffer::empty(Rect::new(0, 0, 50, 15));
    app.game_from_u16_id(1);

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━━━━━━━━━━ RSLibreCell ━━━━━━━━━━━━━━━━━━┓",
        "┃                                      #1        ┃",
        "┃        ..  ..  ..  .. || ..  ..  ..  ..        ┃",
        "┃       ---------------------------------        ┃",
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
        "┗━━━━━━━━━━━ Help <F1> Quit <CTRL-q> ━━━━━━━━━━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(18, 0, 13, 1), title_style);
        expected.set_style(Rect::new(18, 14, 4, 1), key_style);
        expected.set_style(Rect::new(28, 14, 9, 1), key_style);

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
    let mut buf = Buffer::empty(Rect::new(0, 0, 50, 15));
    app.handle_key_event(KeyCode::F(1).into());

    app.render(buf.area, &mut buf);

    let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━━━━━━━━━━ RSLibreCell ━━━━━━━━━━━━━━━━━━┓",
        "┃ ┌─────────────────── Help ───────────────────┐ ┃",
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
        "┃ └─────────────── Close <Esc> ────────────────┘ ┃",
        "┗━━━━━━━━━━━ Help <F1> Quit <CTRL-q> ━━━━━━━━━━━━┛",
    ]);
    {
        let title_style = Style::new().bold();
        let key_style = Style::new().blue();
        let key_style_bold = Style::new().blue().bold();
        expected.set_style(Rect::new(18, 0, 13, 1), title_style);
        expected.set_style(Rect::new(4, 2, 4, 1), key_style);

        expected.set_style(Rect::new(4, 4, 15, 1), key_style);
        expected.set_style(Rect::new(4, 5, 15, 1), key_style);
        expected.set_style(Rect::new(4, 6, 15, 1), key_style);
        expected.set_style(Rect::new(4, 7, 15, 1), key_style);

        expected.set_style(Rect::new(15, 10, 7, 1), key_style);
        expected.set_style(Rect::new(40, 10, 3, 1), key_style);

        expected.set_style(Rect::new(25, 13, 6, 1), key_style_bold);
        expected.set_style(Rect::new(18, 14, 4, 1), key_style_bold);
        expected.set_style(Rect::new(28, 14, 9, 1), key_style_bold);
    }

    assert_eq!(buf, expected);

    app.handle_key_event(KeyCode::Esc.into());
    app.handle_key_event(KeyCode::F(2).into());
    app.handle_key_event(KeyCode::F(1).into());

    app.render(buf.area, &mut buf);
    assert_eq!(buf, expected);
}
