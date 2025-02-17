//! This project implements FreeCell, specifically the original 32000 MS deals
//! and the next 32000 games derived from the same algorithm.
//! The project supports automatically moving cards to the foundations (the target area),
//! and supermoves (moving several cards at once if there are enough free cells).

use std::io;

mod app;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = app::App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
