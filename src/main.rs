#[derive(Debug)]
struct Game {
    id: u16,
    cells: [i8; 4], // todo: something nullable
    foundations: [Vec<i8>; 4],
    columns: [Vec<i8>; 8]
}

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
