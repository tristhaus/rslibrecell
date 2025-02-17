use std::convert::TryFrom;
use std::fmt;

/// The rank of a card.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Rank {
    /// Rank of ace, lowest.
    Ace = 0,
    /// Rank of two.
    Two = 1,
    /// Rank of three.
    Three = 2,
    /// Rank of four.
    Four = 3,
    /// Rank of five.
    Five = 4,
    /// Rank of six.
    Six = 5,
    /// Rank of seven.
    Seven = 6,
    /// Rank of eight.
    Eight = 7,
    /// Rank of nine.
    Nine = 8,
    /// Rank of ten.
    Ten = 9,
    /// Rank of jack.
    Jack = 10,
    /// Rank of queen.
    Queen = 11,
    /// Rank of king, highest.
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

/// The suit of a card.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Suit {
    /// Suit of clubs, ♣.
    Clubs = 0,
    /// Suit of diamonds, ♦.
    Diamonds = 1,
    /// Suit of hearts, ♥.
    Hearts = 2,
    /// Suit of spades, ♠.
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

/// A card, represented through an internal ID, a suit and a rank.
/// 
/// A card is unique within a FreeCell game.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    id: u8,
    /// The suit of the card.
    pub suit: Suit,
    /// The rank of the card.
    pub rank: Rank,
}

impl Card {
    /// Creates a card from an ID.
    /// 
    /// # Panics
    /// The method will panic if given an invalid `id > 51`.
    pub fn from_id(id: u8) -> Card {
        if id > 51 {
            panic!("`id` cannot be greater than 51, is: {id}")
        }

        let rank = Rank::try_from(id / 4).ok().unwrap();
        let suit = Suit::try_from(id % 4).ok().unwrap();

        Card { id, rank, suit }
    }

    /// Creates a card from its string representation.
    /// 
    /// The string representation must be a valid rank identifier
    /// followed by a valid suit symbol, such as `"8♠"` or `"K♦"`.
    /// 
    /// # Panics
    /// The method will panic if given an invalid string.
    pub fn from_str(representation: &str) -> Card {
        Card::try_from(representation).unwrap()
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

    /// Tries to create a card fomr its string representation.
    /// 
    /// The string representation must be a valid rank identifier
    /// followed by a valid suit symbol, such as `"8♠"` or `"K♦"`.
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
