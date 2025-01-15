use crate::logic::Game;

pub mod logic;

fn main() {
    println!("Hello, world!");

    let game = Game {
        id: 17,
        cells: [-1, -1, -1, -1],
        foundations: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        columns: [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()]
    };

    println!("{game:?}");
}
