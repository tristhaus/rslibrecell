pub mod lib {

    #[derive(Debug, Clone)]
    pub struct Game {
        pub id: u16,
        pub cells: [Option<i8>; 4],
        pub foundations: [Vec<i8>; 4],
        pub columns: [Vec<i8>; 8],
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn game_can_be_cloned_yields_independent_instances() {
            let mut game = Game {
                id: 17,
                cells: [None, None, None, None],
                foundations: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
                columns: [
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ],
            };

            let clone = game.clone();

            game.id = 11;
            game.cells[1] = Some(2);
            game.foundations[2].push(3);
            game.columns[3].push(4);

            assert_eq!(17, clone.id);
            assert_eq!(11, game.id);

            assert_eq!(None, clone.cells[1]);
            assert_eq!(Some(2), game.cells[1]);

            assert!(clone.foundations[2].is_empty());
            assert_eq!(3, game.foundations[2][0]);

            assert!(clone.columns[3].is_empty());
            assert_eq!(4, game.columns[3][0]);
        }
    }
}
