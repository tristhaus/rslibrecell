pub mod lib {
    use std::convert::TryFrom;
    use std::{collections, fmt};

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub enum Rank {
        Ace = 0,
        Two = 1,
        Three = 2,
        Four = 3,
        Five = 4,
        Six = 5,
        Seven = 6,
        Eight = 7,
        Nine = 8,
        Ten = 9,
        Jack = 10,
        Queen = 11,
        King = 12,
    }

    impl TryFrom<u8> for Rank {
        type Error = ();

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                x if x == Rank::Ace as u8 => Ok(Rank::Ace),
                x if x == Rank::Two as u8 => Ok(Rank::Two),
                x if x == Rank::Three as u8 => Ok(Rank::Three),
                x if x == Rank::Four as u8 => Ok(Rank::Four),
                x if x == Rank::Five as u8 => Ok(Rank::Five),
                x if x == Rank::Six as u8 => Ok(Rank::Six),
                x if x == Rank::Seven as u8 => Ok(Rank::Seven),
                x if x == Rank::Eight as u8 => Ok(Rank::Eight),
                x if x == Rank::Nine as u8 => Ok(Rank::Nine),
                x if x == Rank::Ten as u8 => Ok(Rank::Ten),
                x if x == Rank::Jack as u8 => Ok(Rank::Jack),
                x if x == Rank::Queen as u8 => Ok(Rank::Queen),
                x if x == Rank::King as u8 => Ok(Rank::King),
                _ => Err(()),
            }
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub enum Suit {
        Clubs = 0,
        Diamonds = 1,
        Hearts = 2,
        Spades = 3,
    }

    impl TryFrom<u8> for Suit {
        type Error = ();

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                x if x == Suit::Clubs as u8 => Ok(Suit::Clubs),
                x if x == Suit::Diamonds as u8 => Ok(Suit::Diamonds),
                x if x == Suit::Hearts as u8 => Ok(Suit::Hearts),
                x if x == Suit::Spades as u8 => Ok(Suit::Spades),
                _ => Err(()),
            }
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Card {
        id: u8,
        suit: Suit,
        rank: Rank,
    }

    impl Card {
        #[allow(dead_code)] // todo : remove once `new` is used
        fn new(id: u8) -> Card {
            if id > 51 {
                panic!("`id` cannot be greater than 51, is: {id}")
            }

            let rank = Rank::try_from(id / 4).ok().unwrap();
            let suit = Suit::try_from(id % 4).ok().unwrap();

            Card { id, rank, suit }
        }

        #[allow(dead_code)] // todo : remove once `from` is used - or mark as cfg test
        fn from(representation: &str) -> Card {
            Card::try_from(representation).unwrap()
        }

        pub fn suit(&self) -> &Suit {
            &self.suit
        }

        pub fn rank(&self) -> &Rank {
            &self.rank
        }
    }

    impl fmt::Display for Card {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let rank = match self.rank {
                Rank::Ace => "A",
                Rank::Two => "2",
                Rank::Three => "3",
                Rank::Four => "4",
                Rank::Five => "5",
                Rank::Six => "6",
                Rank::Seven => "7",
                Rank::Eight => "8",
                Rank::Nine => "9",
                Rank::Ten => "T",
                Rank::Jack => "J",
                Rank::Queen => "Q",
                Rank::King => "K",
            };

            let suit = match self.suit {
                Suit::Clubs => "♣",
                Suit::Diamonds => "♦",
                Suit::Hearts => "♥",
                Suit::Spades => "♠",
            };

            write!(f, "{}{}", rank, suit)
        }
    }

    impl TryFrom<&str> for Card {
        type Error = ();

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.len() < 3 {
                return Err(());
            }

            let mut chars = value.chars();

            let rank = match chars.next().unwrap() {
                'A' => Rank::Ace,
                '2' => Rank::Two,
                '3' => Rank::Three,
                '4' => Rank::Four,
                '5' => Rank::Five,
                '6' => Rank::Six,
                '7' => Rank::Seven,
                '8' => Rank::Eight,
                '9' => Rank::Nine,
                'T' => Rank::Ten,
                'J' => Rank::Jack,
                'Q' => Rank::Queen,
                'K' => Rank::King,
                _ => return Err(()),
            };

            let suit = match chars.next().unwrap() {
                '♣' => Suit::Clubs,
                '♦' => Suit::Diamonds,
                '♥' => Suit::Hearts,
                '♠' => Suit::Spades,
                _ => return Err(()),
            };

            let id = (rank as u8) * 4 + (suit as u8);

            Ok(Card { id, rank, suit })
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Game {
        pub id: u16,
        pub cells: [Option<Card>; 4],
        pub foundations: [Vec<Card>; 4],
        pub columns: [Vec<Card>; 8],
    }

    impl fmt::Display for Game {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut result = String::from("RustLibreCell              ");

            let id = self.id.to_string();

            for _ in 0..(5 - id.len()) {
                result += " ";
            }

            result += "#";
            result += &id;

            result += " \n\n";

            for cell in &self.cells {
                match cell {
                    None => result.push_str(" .. "),
                    Some(cell) => result = format!("{} {} ", result, cell),
                }
            }

            result += "||";

            for foundation in &self.foundations {
                if foundation.len() == 0 {
                    result += " .. "
                } else {
                    result = format!("{} {} ", result, foundation[foundation.len() - 1])
                }
            }

            result += "\n--------------------------------- \n";

            for i in 0..19 {
                result += " ";
                for column in &self.columns {
                    match column.get(i) {
                        None => result += "    ",
                        Some(card) => result = format!("{} {} ", result, card),
                    }
                }
                result += "\n";
            }

            write!(f, "{}", result)
        }
    }

    impl TryFrom<&str> for Game {
        type Error = ();

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let mut cells = [
                Option::<Card>::None,
                Option::<Card>::None,
                Option::<Card>::None,
                Option::<Card>::None,
            ];
            let mut foundations = [
                Vec::<Card>::new(),
                Vec::<Card>::new(),
                Vec::<Card>::new(),
                Vec::<Card>::new(),
            ];
            let mut columns = [
                Vec::<Card>::new(),
                Vec::<Card>::new(),
                Vec::<Card>::new(),
                Vec::<Card>::new(),
                Vec::<Card>::new(),
                Vec::<Card>::new(),
                Vec::<Card>::new(),
                Vec::<Card>::new(),
            ];

            let mut check_set: collections::HashSet<Card> = collections::HashSet::<Card>::new();

            let mut iter = value.lines();

            let title_line = match iter.next() {
                Some(line) => line,
                None => return Err(()),
            };

            let mut read_game_id = false;
            let mut game_id = String::new();
            for c in title_line.chars() {
                if c == '#' {
                    read_game_id = true;
                    continue;
                }

                if read_game_id {
                    game_id.push(c);
                }
            }

            let game_id = match game_id.parse::<u16>() {
                Ok(id) => id,
                Err(_) => return Err(()),
            };

            iter.next(); // skip empty line between title and cells || foundations

            let cells_foundations_line = match iter.next() {
                Some(line) => line,
                None => return Err(()),
            };

            let mut working_on_cells = true;
            let mut card_helper = String::new();

            for (index, ch) in cells_foundations_line.chars().enumerate() {
                if ch == ' ' {
                    continue;
                }

                if ch == '|' {
                    working_on_cells = false;
                    continue;
                }

                card_helper.push(ch);

                if card_helper.len() > 1 {
                    let card = match Card::try_from(card_helper.as_str()) {
                        Ok(card) => Some(card),
                        Err(_) => None,
                    };

                    if working_on_cells {
                        let cell_index = (index - 1) / 4;
                        cells[cell_index] = card;

                        if card.is_some() {
                            check_set.insert(card.unwrap());
                        }
                    } else {
                        if card.is_some() {
                            let card = card.unwrap();
                            let foundation_index = (index - 19) / 4;

                            let rank = card.rank as u8;

                            for r in 0..(rank + 1) {
                                let id = r * 4 + card.suit as u8;
                                let foundation_card = Card::new(id);
                                foundations[foundation_index].push(foundation_card);
                                check_set.insert(foundation_card);
                            }
                        }
                    }

                    card_helper.clear();
                }
            }

            iter.next(); // skip divider between "cells || foundations" and "columns"

            loop {
                let columns_line = match iter.next() {
                    Some(line) => line,
                    None => break,
                };

                for (index, ch) in columns_line.chars().enumerate() {
                    if ch == ' ' {
                        continue;
                    }

                    card_helper.push(ch);

                    if card_helper.len() > 1 {
                        let card = match Card::try_from(card_helper.as_str()) {
                            Ok(card) => Some(card),
                            Err(_) => return Err(()),
                        };

                        let card = card.unwrap();
                        let column_index = (index - 2) / 4;
                        columns[column_index].push(card);
                        check_set.insert(card);
                        card_helper.clear();
                    }
                }
            }

            if check_set.len() != 52 {
                return Err(());
            }

            Ok(Game {
                id: game_id,
                cells,
                foundations,
                columns,
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn card_partialeq_trait_works() {
            let card1 = Card::new(0);
            let card2 = Card::new(0);
            let card3 = Card::new(42);

            assert_eq!(card1, card2);
            assert_ne!(card1, card3);
        }

        #[test]
        fn card_can_be_constructed_from_id() {
            let card1 = Card::new(0);

            assert_eq!(card1.suit(), &Suit::Clubs);
            assert_eq!(card1.rank(), &Rank::Ace);

            let card2 = Card::new(42);

            assert_eq!(card2.suit(), &Suit::Hearts);
            assert_eq!(card2.rank(), &Rank::Jack);
        }

        #[test]
        #[should_panic]
        fn card_when_given_large_id_panics() {
            let _ = Card::new(52);
        }

        #[test]
        fn card_display_trait_works() {
            let card1 = Card::new(0);

            assert_eq!(card1.to_string(), "A♣");

            let card2 = Card::new(42);

            assert_eq!(card2.to_string(), "J♥");
        }

        #[test]
        fn card_tryfrom_ref_str_with_unicode_representation_works() {
            let card1 = Card::try_from("T♣").unwrap();

            assert_eq!(Card::new(36), card1);

            let card2 = Card::try_from("J♥").unwrap();

            assert_eq!(Card::new(42), card2);
        }

        #[test]
        fn card_tryfrom_with_short_string_errors() {
            let _ = Card::try_from("T").expect_err("should have error");
        }

        #[test]
        fn card_tryfrom_with_bad_input1_errors() {
            let _ = Card::try_from("R♣").expect_err("should have error");
        }

        #[test]
        fn card_tryfrom_with_bad_input2_errors() {
            let _ = Card::try_from("T?").expect_err("should have error");
        }

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
            game.cells[1] = Some(Card::new(2));
            game.foundations[2].push(Card::new(3));
            game.columns[3].push(Card::new(4));

            assert_eq!(17, clone.id);
            assert_eq!(11, game.id);

            assert_eq!(None, clone.cells[1]);
            assert_eq!(Some(Card::new(2)), game.cells[1]);

            assert!(clone.foundations[2].is_empty());
            assert_eq!(Card::new(3), game.foundations[2][0]);

            assert!(clone.columns[3].is_empty());
            assert_eq!(Card::new(4), game.columns[3][0]);
        }

        #[test]
        fn game_partialeq_trait_works() {
            let mut game1 = Game {
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

            game1.cells[1] = Some(Card::new(2));
            game1.foundations[2].push(Card::new(3));
            game1.columns[3].push(Card::new(4));

            let game2 = game1.clone();

            let mut game3 = Game {
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

            game3.cells[1] = Some(Card::new(50));
            game3.foundations[2].push(Card::new(3));
            game3.columns[3].push(Card::new(4));

            let mut game4 = Game {
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

            game4.cells[1] = Some(Card::new(2));
            game4.foundations[2].push(Card::new(50));
            game4.columns[3].push(Card::new(4));

            let mut game5 = Game {
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

            game5.cells[1] = Some(Card::new(2));
            game5.foundations[2].push(Card::new(3));
            game5.columns[3].push(Card::new(50));

            assert_eq!(game1, game2);
            assert_ne!(game1, game3);
            assert_ne!(game1, game4);
            assert_ne!(game1, game5);
        }

        #[test]
        fn game_display_trait_works() {
            let reference = "RustLibreCell                 #42 

 T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ 
--------------------------------- 
  5♠  J♠  K♠  K♦  A♠      5♣  K♣ 
  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ 
      J♦  Q♠  3♣  3♠          J♣ 
      9♠  T♦  8♦  K♥             
      9♦  9♣  7♦  T♥             
      6♥  8♥  6♦  5♦             
      8♣  7♣  Q♣  4♠             
      7♥      J♥  3♦             
      6♠      T♠  2♠             
      5♥      9♥                 
      4♣                         
      3♥                         
                                 
                                 
                                 
                                 
                                 
                                 
                                 
";

            let game = Game {
                id: 42,
                cells: [Some(Card::from("T♣")), None, None, Some(Card::from("Q♥"))],
                foundations: [
                    vec![Card::from("A♣"), Card::from("2♣")],
                    vec![],
                    vec![Card::from("A♥")],
                    vec![Card::from("A♦"), Card::from("2♦")],
                ],
                columns: [
                    vec![Card::from("5♠"), Card::from("4♦")],
                    vec![
                        Card::from("J♠"),
                        Card::from("2♥"),
                        Card::from("J♦"),
                        Card::from("9♠"),
                        Card::from("9♦"),
                        Card::from("6♥"),
                        Card::from("8♣"),
                        Card::from("7♥"),
                        Card::from("6♠"),
                        Card::from("5♥"),
                        Card::from("4♣"),
                        Card::from("3♥"),
                    ],
                    vec![
                        Card::from("K♠"),
                        Card::from("7♠"),
                        Card::from("Q♠"),
                        Card::from("T♦"),
                        Card::from("9♣"),
                        Card::from("8♥"),
                        Card::from("7♣"),
                    ],
                    vec![
                        Card::from("K♦"),
                        Card::from("6♣"),
                        Card::from("3♣"),
                        Card::from("8♦"),
                        Card::from("7♦"),
                        Card::from("6♦"),
                        Card::from("Q♣"),
                        Card::from("J♥"),
                        Card::from("T♠"),
                        Card::from("9♥"),
                    ],
                    vec![
                        Card::from("A♠"),
                        Card::from("8♠"),
                        Card::from("3♠"),
                        Card::from("K♥"),
                        Card::from("T♥"),
                        Card::from("5♦"),
                        Card::from("4♠"),
                        Card::from("3♦"),
                        Card::from("2♠"),
                    ],
                    vec![],
                    vec![Card::from("5♣"), Card::from("4♥")],
                    vec![Card::from("K♣"), Card::from("Q♦"), Card::from("J♣")],
                ],
            };

            let result = format!("{}", game);

            assert_eq!(reference, result);
        }

        #[test]
        fn game_tryfrom_ref_str_with_unicode_representation_works() {
            let input = "RustLibreCell                 #42

 T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ 
--------------------------------- 
  5♠  J♠  K♠  K♦  A♠      5♣  K♣ 
  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ 
      J♦  Q♠  3♣  3♠          J♣ 
      9♠  T♦  8♦  K♥             
      9♦  9♣  7♦  T♥             
      6♥  8♥  6♦  5♦             
      8♣  7♣  Q♣  4♠             
      7♥      J♥  3♦             
      6♠      T♠  2♠             
      5♥      9♥                 
      4♣                         
      3♥                         
                                 
                                 
                                 
                                 
                                 
                                 ";

            let reference = Game {
                id: 42,
                cells: [Some(Card::from("T♣")), None, None, Some(Card::from("Q♥"))],
                foundations: [
                    vec![Card::from("A♣"), Card::from("2♣")],
                    vec![],
                    vec![Card::from("A♥")],
                    vec![Card::from("A♦"), Card::from("2♦")],
                ],
                columns: [
                    vec![Card::from("5♠"), Card::from("4♦")],
                    vec![
                        Card::from("J♠"),
                        Card::from("2♥"),
                        Card::from("J♦"),
                        Card::from("9♠"),
                        Card::from("9♦"),
                        Card::from("6♥"),
                        Card::from("8♣"),
                        Card::from("7♥"),
                        Card::from("6♠"),
                        Card::from("5♥"),
                        Card::from("4♣"),
                        Card::from("3♥"),
                    ],
                    vec![
                        Card::from("K♠"),
                        Card::from("7♠"),
                        Card::from("Q♠"),
                        Card::from("T♦"),
                        Card::from("9♣"),
                        Card::from("8♥"),
                        Card::from("7♣"),
                    ],
                    vec![
                        Card::from("K♦"),
                        Card::from("6♣"),
                        Card::from("3♣"),
                        Card::from("8♦"),
                        Card::from("7♦"),
                        Card::from("6♦"),
                        Card::from("Q♣"),
                        Card::from("J♥"),
                        Card::from("T♠"),
                        Card::from("9♥"),
                    ],
                    vec![
                        Card::from("A♠"),
                        Card::from("8♠"),
                        Card::from("3♠"),
                        Card::from("K♥"),
                        Card::from("T♥"),
                        Card::from("5♦"),
                        Card::from("4♠"),
                        Card::from("3♦"),
                        Card::from("2♠"),
                    ],
                    vec![],
                    vec![Card::from("5♣"), Card::from("4♥")],
                    vec![Card::from("K♣"), Card::from("Q♦"), Card::from("J♣")],
                ],
            };

            let game = Game::try_from(input).unwrap();

            assert_eq!(reference, game);
        }

        #[test]
        fn game_tryfrom_empty_string_errors() {
            Game::try_from("").expect_err("should have error")
        }

        #[test]
        fn game_tryfrom_no_cell_line_errors() {
            let input = "RustLibreCell                 #42

";

            Game::try_from(input).expect_err("should have error")
        }

        #[test]
        fn game_tryfrom_bad_card_in_column_errors() {
            let input = "RustLibreCell                 #42

 T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ 
--------------------------------- 
  5♠  J♠  K♠  K♦  XX      5♣  K♣ 
  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ 
      J♦  Q♠  3♣  3♠          J♣ 
      9♠  T♦  8♦  K♥             
      9♦  9♣  7♦  T♥             
      6♥  8♥  6♦  5♦             
      8♣  7♣  Q♣  4♠             
      7♥      J♥  3♦             
      6♠      T♠  2♠             
      5♥      9♥                 
      4♣                         
      3♥                         
                                 
                                 
                                 
                                 
                                 
                                 ";

            Game::try_from(input).expect_err("should have error")
        }

        #[test]
        fn game_tryfrom_card_missing_errors() {
            let ace_of_hearts_missing = "RustLibreCell                 #42

 T♣  ..  ..  Q♥ || 2♣  ..  ..  2♦ 
--------------------------------- 
  5♠  J♠  K♠  K♦  A♠      5♣  K♣ 
  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ 
      J♦  Q♠  3♣  3♠          J♣ 
      9♠  T♦  8♦  K♥             
      9♦  9♣  7♦  T♥             
      6♥  8♥  6♦  5♦             
      8♣  7♣  Q♣  4♠             
      7♥      J♥  3♦             
      6♠      T♠  2♠             
      5♥      9♥                 
      4♣                         
      3♥                         
                                 
                                 
                                 
                                 
                                 
                                 ";

            Game::try_from(ace_of_hearts_missing).expect_err("should have error")
        }

        #[test]
        fn game_tryfrom_duplicated_card_errors() {
            let ace_of_diamonds_twice = "RustLibreCell                 #42

 T♣  ..  A♦  Q♥ || 2♣  ..  ..  2♦ 
--------------------------------- 
  5♠  J♠  K♠  K♦  A♠      5♣  K♣ 
  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ 
      J♦  Q♠  3♣  3♠          J♣ 
      9♠  T♦  8♦  K♥             
      9♦  9♣  7♦  T♥             
      6♥  8♥  6♦  5♦             
      8♣  7♣  Q♣  4♠             
      7♥      J♥  3♦             
      6♠      T♠  2♠             
      5♥      9♥                 
      4♣                         
      3♥                         
                                 
                                 
                                 
                                 
                                 
                                 ";

            Game::try_from(ace_of_diamonds_twice).expect_err("should have error")
        }
    }
}
