pub mod card;
pub mod game;

pub mod lib {
    use crate::{
        card::{Rank, Suit},
        game::Game,
    };

    pub enum Location {
        Cell { i: usize },
        Foundation,
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
            Location::Foundation => {
                return Err(());
            }
        }

        match mv.to {
            Location::Cell { i: to } => {
                if to > 3 || game.cells[to].is_some() {
                    return Err(());
                }
            }
            Location::Foundation => {}
        }

        match mv.from {
            Location::Cell { i: from } => match mv.to {
                Location::Cell { i: to } => move_cell_cell(game, from, to),
                Location::Foundation => move_cell_foundation(game, from),
            },
            Location::Foundation => {
                panic!("should never be reached")
            }
        }
    }

    fn move_cell_cell(game: &Game, from: usize, to: usize) -> Result<Game, ()> {
        let mut game = game.clone();

        game.cells[to] = game.cells[from];
        game.cells[from] = None;

        return Ok(game);
    }

    fn move_cell_foundation(game: &Game, from: usize) -> Result<Game, ()> {
        let mut game = game.clone();

        let card = game.cells[from].unwrap();

        let foundation = match card.suit() {
            Suit::Clubs => 0 as usize,
            Suit::Spades => 1 as usize,
            Suit::Hearts => 2 as usize,
            Suit::Diamonds => 3 as usize,
        };

        let foundation = &mut game.foundations[foundation];

        let foundation_card = foundation.last();

        match foundation_card {
            None => {
                if *card.rank() == Rank::Ace {
                    foundation.push(card);
                } else {
                    return Err(());
                }
            }
            Some(foundation_card) => {
                if *card.rank() == Rank::Ace {
                    return Err(());
                } else {
                    let card_rank = *card.rank() as u8;
                    let foundation_card_rank = *foundation_card.rank() as u8;

                    if card_rank - 1 == foundation_card_rank {
                        foundation.push(card);
                    } else {
                        return Err(());
                    }
                }
            }
        }

        game.cells[from] = None;

        return Ok(game);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        mod helper {
            use super::*;

            pub fn assert_move_succeeds(input: &str, mv: Move, reference: &str) {
                let before = Game::try_from(input).unwrap();
                let expected = Game::try_from(reference).unwrap();

                let after = apply(&before, mv).unwrap();

                assert_eq!(expected, after);
            }

            pub fn assert_move_fails(input: &str, mv: Move) {
                let before = Game::try_from(input).unwrap();
    
                let error = apply(&before, mv);
    
                assert!(error.is_err());
            }
        }

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

            helper::assert_move_succeeds(input, mv, reference);
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

            helper::assert_move_fails(input, mv);
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

            helper::assert_move_fails(input, mv);
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

            helper::assert_move_fails(input, mv);
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

            helper::assert_move_fails(input, mv);
        }

        #[test]
        fn apply_cell_foundation_ace_works() {
            let input = concat!(
                "RustLibreCell                 #42\n",
                "\n",
                " T♣  A♥  2♦  Q♥ || 2♣  ..  ..  A♦ \n",
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
                to: Location::Foundation,
            };

            let reference = concat!(
                "RustLibreCell                 #42\n",
                "\n",
                " T♣  ..  2♦  Q♥ || 2♣  ..  A♥  A♦ \n",
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

            helper::assert_move_succeeds(input, mv, reference);
        }

        #[test]
        fn apply_cell_foundation_two_works() {
            let input = concat!(
                "RustLibreCell                 #42\n",
                "\n",
                " T♣  ..  2♦  Q♥ || 2♣  ..  A♥  A♦ \n",
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
                from: Location::Cell { i: 2 },
                to: Location::Foundation,
            };

            let reference = concat!(
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

            helper::assert_move_succeeds(input, mv, reference);
        }

        #[test]
        fn apply_cannot_move_from_foundation() {
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
                from: Location::Foundation,
                to: Location::Cell { i: 2 },
            };

            helper::assert_move_fails(input, mv);
        }

        #[test]
        fn apply_move_to_foundation_without_correct_rank_fails() {
            let input = concat!(
                "RustLibreCell                 #42\n",
                "\n",
                " T♣  3♥  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
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
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n",
                "                                 \n"
            );

            let mv = Move {
                from: Location::Cell { i: 1 },
                to: Location::Foundation,
            };

            helper::assert_move_fails(input, mv);
        }

    }
}
