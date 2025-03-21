/*
    RSLibreCell - a FreeCell implementation
    Copyright (C) 2025 and later: tristhaus

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::cmp::min;

use crate::{
    card::{Card, Rank, Suit},
    game::Game,
};

/// Defines the `to` and `from` locations of a move.
#[derive(Clone, Debug)]
pub enum Location {
    /// The location is a cell with some 0-based index.
    Cell {
        /// The 0-based index of the cell.
        i: usize,
    },
    /// The location is the card-appropriate foundation.
    Foundation,
    /// The location is a column with some 0-based index.
    Column {
        /// The 0-based index of the column.
        i: usize,
    },
}

/// Defines a move for a game.
#[derive(Clone)]
pub struct Move {
    /// The initial location of the card(s).
    pub from: Location,
    /// The final location of the card(s).
    pub to: Location,
}

/// Applies a move to the game.
///
/// # Panics
/// The method will only panic in case of an internal bug.
pub(crate) fn apply(game: &Game, mv: Move) -> Result<Game, ()> {
    match mv.from {
        Location::Cell { i: from } => {
            if from > 3 || game.cells[from].is_none() {
                return Err(());
            }
        }
        Location::Foundation => {
            return Err(());
        }
        Location::Column { i: from } => {
            if from > 7 || game.columns[from].is_empty() {
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
        Location::Foundation => {}
        Location::Column { i: to } => {
            if to > 7 {
                return Err(());
            }
        }
    }

    match mv.from {
        Location::Cell { i: from } => match mv.to {
            Location::Cell { i: to } => move_cell_cell(game, from, to),
            Location::Foundation => move_cell_foundation(game, from),
            Location::Column { i: to } => move_cell_column(game, from, to),
        },
        Location::Foundation => {
            panic!("should never be reached")
        }
        Location::Column { i: from } => match mv.to {
            Location::Cell { i: to } => move_column_cell(game, from, to),
            Location::Foundation => move_column_foundation(game, from),
            Location::Column { i: to } => move_column_column(game, from, to),
        },
    }
}

/// Performs **one** automove, *i.e.* the moving of a card
/// to its foundation if it definitely is no longer useful.
///
/// If an automove was performed, `Some` is returned, `None` otherwise.
/// The rules for an automove are somewhat involved, but boil down to:
/// > All cards that could be placed on the automoved card are
/// > * already placed on a foundation or
/// > * can be placed there, once accessible
pub(crate) fn automove(game: &Game) -> Option<Game> {
    let game = game.clone();

    for (i, column) in game.columns.iter().enumerate() {
        let card = match column.last() {
            Some(card) => card,
            None => continue,
        };

        if !check(&game, *card) {
            continue;
        }

        let mv = Move {
            from: Location::Column { i },
            to: Location::Foundation,
        };

        return Some(apply(&game, mv).unwrap());
    }

    for (i, cell) in game.cells.iter().enumerate() {
        let card = match cell {
            Some(card) => card,
            None => continue,
        };

        if !check(&game, *card) {
            continue;
        }

        let mv = Move {
            from: Location::Cell { i },
            to: Location::Foundation,
        };

        return Some(apply(&game, mv).unwrap());
    }

    return None;

    fn check(game: &Game, card: Card) -> bool {
        if card.rank == Rank::Ace {
            return true;
        }

        let own_foundation = &game.foundations[detail::find_foundation_for(card.suit)];

        if own_foundation.len() == 0
            || own_foundation.last().unwrap().rank as i8 != card.rank as i8 - 1
        {
            return false;
        }

        let other_foundation_same_color = match card.suit {
            Suit::Clubs => &game.foundations[detail::find_foundation_for(Suit::Spades)],
            Suit::Diamonds => &game.foundations[detail::find_foundation_for(Suit::Hearts)],
            Suit::Hearts => &game.foundations[detail::find_foundation_for(Suit::Diamonds)],
            Suit::Spades => &game.foundations[detail::find_foundation_for(Suit::Clubs)],
        };

        let other_foundation_same_color_rank = match other_foundation_same_color.last() {
            Some(card) => card.rank as i8,
            None => -1 as i8,
        };

        let other_color_min_rank = match card.suit {
            Suit::Clubs | Suit::Spades => {
                let heart_rank =
                    match &game.foundations[detail::find_foundation_for(Suit::Hearts)].last() {
                        Some(card) => card.rank as i8,
                        None => -1 as i8,
                    };
                let diamond_rank =
                    match &game.foundations[detail::find_foundation_for(Suit::Diamonds)].last() {
                        Some(card) => card.rank as i8,
                        None => -1 as i8,
                    };

                min(heart_rank, diamond_rank)
            }
            Suit::Diamonds | Suit::Hearts => {
                let club_rank =
                    match &game.foundations[detail::find_foundation_for(Suit::Clubs)].last() {
                        Some(card) => card.rank as i8,
                        None => -1 as i8,
                    };
                let spade_rank =
                    match &game.foundations[detail::find_foundation_for(Suit::Spades)].last() {
                        Some(card) => card.rank as i8,
                        None => -1 as i8,
                    };

                min(club_rank, spade_rank)
            }
        };

        let own_foundation_rank = own_foundation.last().unwrap().rank as i8;

        return (own_foundation_rank - other_color_min_rank < 2)
            && (own_foundation_rank <= other_color_min_rank
                || other_color_min_rank - other_foundation_same_color_rank < 2);
    }
}

// `from`, `to` are indices
fn move_cell_cell(game: &Game, from: usize, to: usize) -> Result<Game, ()> {
    let mut game = game.clone();

    game.cells[to] = game.cells[from];
    game.cells[from] = None;

    return Ok(game);
}

// `from` is an index
fn move_cell_foundation(game: &Game, from: usize) -> Result<Game, ()> {
    let card = game.cells[from].unwrap();

    let mut game = game.clone();

    if let Ok(()) = detail::move_card_to_foundation(&mut game, card) {
        game.cells[from] = None;

        return Ok(game);
    } else {
        return Err(());
    }
}

// `from`, `to` are indices
fn move_cell_column(game: &Game, from: usize, to: usize) -> Result<Game, ()> {
    let lower = game.cells[from].unwrap();

    if !game.columns[to].is_empty() {
        let upper = game.columns[to].last().unwrap();
        if !detail::fit_together(upper, &lower) {
            return Err(());
        }
    }

    let mut game = game.clone();

    game.columns[to].push(lower);
    game.cells[from] = None;

    Ok(game)
}

// `from`, `to` are indices
fn move_column_cell(game: &Game, from: usize, to: usize) -> Result<Game, ()> {
    let mut game = game.clone();

    let card = game.columns[from].pop().unwrap();

    game.cells[to] = Some(card);

    return Ok(game);
}

// `from` is an index
fn move_column_foundation(game: &Game, from: usize) -> Result<Game, ()> {
    let mut game = game.clone();

    let card = game.columns[from].pop().unwrap();

    if let Ok(()) = detail::move_card_to_foundation(&mut game, card) {
        return Ok(game);
    } else {
        return Err(());
    }
}

// `from`, `to` are indices
fn move_column_column(game: &Game, from: usize, to: usize) -> Result<Game, ()> {
    if from == to {
        return Err(());
    }

    let mut empty_column_count: u16 = game.columns.iter().filter(|x| x.is_empty()).count() as u16;
    empty_column_count -= match game.columns[to].is_empty() {
        true => 1,
        false => 0,
    };

    let empty_cell_count: u16 = game.cells.iter().filter(|x| x.is_none()).count() as u16;
    let mut max_move_size = empty_cell_count + 1;

    for _i in 0..empty_column_count {
        max_move_size *= 2;
    }

    max_move_size = min(13, max_move_size);

    // look for a continuous run in the `from` column
    let mut run = 1;

    let from_count = game.columns[from].len();
    for i in 0..(from_count - 1) {
        let upper = game.columns[from][from_count - (i + 2)];
        let lower = game.columns[from][from_count - (i + 1)];

        if detail::fit_together(&upper, &lower) {
            run += 1;
        } else {
            break;
        }
    }

    max_move_size = min(run, max_move_size);

    // now cut down move such that it fits onto the `to` bottom card
    let to_card = game.columns[to].last();
    if to_card.is_some() {
        while max_move_size > 0 {
            let from_top_card =
                game.columns[from][(from_count as usize) - (max_move_size as usize)];

            if detail::fit_together(to_card.unwrap(), &from_top_card) {
                break;
            }

            max_move_size -= 1;
        }
    }

    if max_move_size == 0 {
        return Err(());
    }

    let mut game = game.clone();

    let mut stash: Vec<Card> = vec![];

    // actually move cards
    for _i in 0..max_move_size {
        stash.push(game.columns[from].pop().unwrap())
    }

    for _i in 0..max_move_size {
        game.columns[to].push(stash.pop().unwrap())
    }

    return Ok(game);
}

mod detail {
    use crate::{
        card::{Card, Rank, Suit},
        game::Game,
    };

    pub(super) fn move_card_to_foundation(game: &mut Game, card: Card) -> Result<(), ()> {
        let foundation = find_foundation_for(card.suit);

        let foundation = &mut game.foundations[foundation];

        let foundation_card = foundation.last();

        match foundation_card {
            None => {
                if card.rank == Rank::Ace {
                    foundation.push(card);
                } else {
                    return Err(());
                }
            }
            Some(foundation_card) => {
                if card.rank == Rank::Ace {
                    return Err(());
                } else {
                    let card_rank = card.rank as u8;
                    let foundation_card_rank = foundation_card.rank as u8;

                    if card_rank - 1 == foundation_card_rank {
                        foundation.push(card);
                    } else {
                        return Err(());
                    }
                }
            }
        }

        Ok(())
    }

    pub(super) fn find_foundation_for(suit: Suit) -> usize {
        let foundation = match suit {
            Suit::Clubs => 0 as usize,
            Suit::Spades => 1 as usize,
            Suit::Hearts => 2 as usize,
            Suit::Diamonds => 3 as usize,
        };

        return foundation;
    }

    /// Returns a flag indicating whether the two cards, with the lower card placed below
    /// the upper card on a column, will legally fit together.
    ///
    /// Examples:
    /// * upper `6♣`, lower `5♥` yields `true`
    /// * upper `6♣`, lower `7♥` yields `false`
    /// * upper `6♣`, lower `5♠` yields `false`
    pub(super) fn fit_together(upper: &Card, lower: &Card) -> bool {
        if upper.rank == Rank::Ace {
            return false;
        }

        if is_red_func(*lower) == is_red_func(*upper) {
            return false;
        }

        let over_rank = upper.rank as u8;
        let under_rank = lower.rank as u8;

        return over_rank - 1 == under_rank;

        fn is_red_func(card: Card) -> bool {
            card.suit == Suit::Hearts || card.suit == Suit::Diamonds
        }
    }
}

#[cfg(test)]
mod test;
