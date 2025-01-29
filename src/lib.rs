pub mod card;
pub mod game;

pub mod lib {
    use crate::game::Game;

    pub enum Location {
        Cell { i: usize },
        // todo Foundation,
        // todo Column { i: usize }
    }

    pub struct Move {
        from: Location,
        to: Location,
    }

    pub fn apply(game: &Game, mv: Move) -> Result<Game, ()> {
        match mv.from {
            Location::Cell { i: from } => {
                if from > 3 || game.cells[from].is_none() {
                    return Err(());
                }
            }
        }

        match mv.to {
            Location::Cell { i: to } => {
                if to > 3 || game.cells[to].is_some() {
                    return Err(());
                }
            }
        }

        match mv.from {
            Location::Cell { i: from } => match mv.to {
                Location::Cell { i: to } => move_cell_cell(from, to, game),
            },
        }
    }

    fn move_cell_cell(from: usize, to: usize, game: &Game) -> Result<Game, ()> {
        let mut game = game.clone();

        game.cells[to] = game.cells[from];
        game.cells[from] = None;

        return Ok(game);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn apply_cell_cell_works() {
            let input = concat!(
                "RustLibreCell                 #42\n",
                "\n",
                " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
                "--------------------------------- \n",
                "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
                "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
                "      J♦  Q♠  3♣  3♠          J♣ \n",
                "      9♠  T♦  8♦  K♥             \n",
                "      9♦  9♣  7♦  T♥             \n",
                "      6♥  8♥  6♦  5♦             \n",
                "      8♣  7♣  Q♣  4♠             \n",
                "      7♥      J♥  3♦             \n",
                "      6♠      T♠  2♠             \n",
                "      5♥      9♥                 \n",
                "      4♣                         \n",
                "      3♥                         \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n"
            );

            let mv = Move {
                from: Location::Cell { i: 3 },
                to: Location::Cell { i: 2 },
            };

            let reference = concat!(
                "RustLibreCell                 #42\n",
                "\n",
                " T♣  ..  Q♥  .. || 2♣  ..  A♥  2♦ \n",
                "--------------------------------- \n",
                "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
                "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
                "      J♦  Q♠  3♣  3♠          J♣ \n",
                "      9♠  T♦  8♦  K♥             \n",
                "      9♦  9♣  7♦  T♥             \n",
                "      6♥  8♥  6♦  5♦             \n",
                "      8♣  7♣  Q♣  4♠             \n",
                "      7♥      J♥  3♦             \n",
                "      6♠      T♠  2♠             \n",
                "      5♥      9♥                 \n",
                "      4♣                         \n",
                "      3♥                         \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n"
            );

            let before = Game::try_from(input).unwrap();
            let expected = Game::try_from(reference).unwrap();

            let after = apply(&before, mv).unwrap();

            assert_eq!(expected, after);
        }

        #[test]
        fn apply_cannot_move_from_nonexistent_cell() {
            let input = concat!(
                "RustLibreCell                 #42\n",
                "\n",
                " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
                "--------------------------------- \n",
                "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
                "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
                "      J♦  Q♠  3♣  3♠          J♣ \n",
                "      9♠  T♦  8♦  K♥             \n",
                "      9♦  9♣  7♦  T♥             \n",
                "      6♥  8♥  6♦  5♦             \n",
                "      8♣  7♣  Q♣  4♠             \n",
                "      7♥      J♥  3♦             \n",
                "      6♠      T♠  2♠             \n",
                "      5♥      9♥                 \n",
                "      4♣                         \n",
                "      3♥                         \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n"
            );

            let mv = Move {
                from: Location::Cell { i: 4 },
                to: Location::Cell { i: 2 },
            };

            let before = Game::try_from(input).unwrap();

            let error = apply(&before, mv);

            assert!(error.is_err());
        }

        #[test]
        fn apply_cannot_move_to_nonexistent_cell() {
            let input = concat!(
                "RustLibreCell                 #42\n",
                "\n",
                " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
                "--------------------------------- \n",
                "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
                "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
                "      J♦  Q♠  3♣  3♠          J♣ \n",
                "      9♠  T♦  8♦  K♥             \n",
                "      9♦  9♣  7♦  T♥             \n",
                "      6♥  8♥  6♦  5♦             \n",
                "      8♣  7♣  Q♣  4♠             \n",
                "      7♥      J♥  3♦             \n",
                "      6♠      T♠  2♠             \n",
                "      5♥      9♥                 \n",
                "      4♣                         \n",
                "      3♥                         \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n"
            );

            let mv = Move {
                from: Location::Cell { i: 3 },
                to: Location::Cell { i: 4 },
            };

            let before = Game::try_from(input).unwrap();

            let error = apply(&before, mv);

            assert!(error.is_err());
        }

        #[test]
        fn apply_cannot_move_to_full_cell() {
            let input = concat!(
                "RustLibreCell                 #42\n",
                "\n",
                " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
                "--------------------------------- \n",
                "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
                "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
                "      J♦  Q♠  3♣  3♠          J♣ \n",
                "      9♠  T♦  8♦  K♥             \n",
                "      9♦  9♣  7♦  T♥             \n",
                "      6♥  8♥  6♦  5♦             \n",
                "      8♣  7♣  Q♣  4♠             \n",
                "      7♥      J♥  3♦             \n",
                "      6♠      T♠  2♠             \n",
                "      5♥      9♥                 \n",
                "      4♣                         \n",
                "      3♥                         \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n"
            );

            let mv = Move {
                from: Location::Cell { i: 3 },
                to: Location::Cell { i: 0 },
            };

            let before = Game::try_from(input).unwrap();

            let error = apply(&before, mv);

            assert!(error.is_err());
        }

        #[test]
        fn apply_cannot_move_from_empty_cell() {
            let input = concat!(
                "RustLibreCell                 #42\n",
                "\n",
                " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
                "--------------------------------- \n",
                "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
                "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
                "      J♦  Q♠  3♣  3♠          J♣ \n",
                "      9♠  T♦  8♦  K♥             \n",
                "      9♦  9♣  7♦  T♥             \n",
                "      6♥  8♥  6♦  5♦             \n",
                "      8♣  7♣  Q♣  4♠             \n",
                "      7♥      J♥  3♦             \n",
                "      6♠      T♠  2♠             \n",
                "      5♥      9♥                 \n",
                "      4♣                         \n",
                "      3♥                         \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n"
            );

            let mv = Move {
                from: Location::Cell { i: 1 },
                to: Location::Cell { i: 2 },
            };

            let before = Game::try_from(input).unwrap();

            let error = apply(&before, mv);

            assert!(error.is_err());
        }
    }
}
