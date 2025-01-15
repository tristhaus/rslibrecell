use crate::logic::Game;

pub mod logic;

fn main() {
    println!("Hello, world!");

    let mut game = Game {
        id: 17,
        cells: [None, None, None, None],
        foundations: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        columns: [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()]
    };

    let clone = game.clone();

    game.id = 11;
    game.cells[1] = Some(2);
    game.foundations[2].push(3);

    println!("game: {game:?}");
    println!("clone: {clone:?}");
}
