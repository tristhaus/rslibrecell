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
        "┃           RustLibreCell                        ┃",
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
    let title_style = Style::new().bold();
    let key_style = Style::new().blue().bold();
    expected.set_style(Rect::new(18, 0, 13, 1), title_style);
    expected.set_style(Rect::new(18, 14, 4, 1), key_style);
    expected.set_style(Rect::new(28, 14, 9, 1), key_style);

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
        "┃ └─────────────── Close <Esc> ────────────────┘ ┃",
        "┗━━━━━━━━━━━ Help <F1> Quit <CTRL-q> ━━━━━━━━━━━━┛",
    ]);
    let title_style = Style::new().bold();
    let key_style = Style::new().blue();
    let key_style_bold = Style::new().blue().bold();
    expected.set_style(Rect::new(18, 0, 13, 1), title_style);
    expected.set_style(Rect::new(4, 2, 15, 1), key_style);
    expected.set_style(Rect::new(4, 3, 15, 1), key_style);
    expected.set_style(Rect::new(4, 4, 15, 1), key_style);
    expected.set_style(Rect::new(4, 5, 15, 1), key_style);
    
    expected.set_style(Rect::new(15, 8, 7, 1), key_style);
    expected.set_style(Rect::new(40, 8, 3, 1), key_style);

    expected.set_style(Rect::new(25, 13, 6, 1), key_style_bold);
    expected.set_style(Rect::new(18, 14, 4, 1), key_style_bold);
    expected.set_style(Rect::new(28, 14, 9, 1), key_style_bold);

    assert_eq!(buf, expected);
}
