use super::*;
use ratatui::style::Style;

#[test]
fn render() {
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
        "┗━━━━━━━━━━━━━━━━━━━ Quit <Q> ━━━━━━━━━━━━━━━━━━━┛",
    ]);
    let title_style = Style::new().bold();
    let key_style = Style::new().blue().bold();
    expected.set_style(Rect::new(18, 0, 13, 1), title_style);
    expected.set_style(Rect::new(26, 14, 4, 1), key_style);

    assert_eq!(buf, expected);
}

#[test]
fn handle_key_event_quit() -> io::Result<()> {
    let mut app = App::new();
    app.handle_key_event(KeyCode::Char('Q').into());
    assert_eq!(app.app_state, AppState::Exit);

    Ok(())
}
