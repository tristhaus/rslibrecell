use std::convert::TryFrom;
use std::fmt;

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
    pub fn from_id(id: u8) -> Card {
        if id > 51 {
            panic!("`id` cannot be greater than 51, is: {id}")
        }

        let rank = Rank::try_from(id / 4).ok().unwrap();
        let suit = Suit::try_from(id % 4).ok().unwrap();

        Card { id, rank, suit }
    }

    pub fn from_str(representation: &str) -> Card {
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

#[cfg(test)]
mod test;
