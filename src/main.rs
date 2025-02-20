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
