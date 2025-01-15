#[derive(Debug)]
pub struct Game {
    pub id: u16,
    pub cells: [i8; 4], // todo: something nullable
    pub foundations: [Vec<i8>; 4],
    pub columns: [Vec<i8>; 8]
}
