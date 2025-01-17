pub mod lib {
    use std::convert::TryFrom;
    use std::fmt;

    #[derive(Copy, Clone, Debug, PartialEq)]
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

    #[derive(Copy, Clone, Debug, PartialEq)]
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

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Card {
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

            Card { rank, suit }
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

            Ok(Card { rank, suit })
        }
    }

    #[derive(Debug, Clone)]
    pub struct Game {
        pub id: u16,
        pub cells: [Option<Card>; 4],
        pub foundations: [Vec<Card>; 4],
        pub columns: [Vec<Card>; 8],
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
    }
}
