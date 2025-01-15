#[derive(Debug, Clone)]
pub struct Game {
    pub id: u16,
    pub cells: [Option<i8>; 4],
    pub foundations: [Vec<i8>; 4],
    pub columns: [Vec<i8>; 8]
}
