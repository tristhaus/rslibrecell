use std::cmp::min;

use crate::card::Card;
use crate::game::Game;

pub enum Location {
    Cell { i: usize },
    Foundation,
    Column { i: usize },
}

pub struct Move {
    pub from: Location,
    pub to: Location,
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

fn move_cell_cell(game: &Game, from: usize, to: usize) -> Result<Game, ()> {
    let mut game = game.clone();

    game.cells[to] = game.cells[from];
    game.cells[from] = None;

    return Ok(game);
}

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

fn move_column_cell(game: &Game, from: usize, to: usize) -> Result<Game, ()> {
    let mut game = game.clone();

    let card = game.columns[from].pop().unwrap();

    game.cells[to] = Some(card);

    return Ok(game);
}

fn move_column_foundation(game: &Game, from: usize) -> Result<Game, ()> {
    let mut game = game.clone();

    let card = game.columns[from].pop().unwrap();

    if let Ok(()) = detail::move_card_to_foundation(&mut game, card) {
        return Ok(game);
    } else {
        return Err(());
    }
}

fn move_column_column(game: &Game, from: usize, to: usize) -> Result<Game, ()> {
    if from == to {
        return Err(());
    }

    let mut empty_column_count: u16 = game
        .columns
        .iter()
        .map(|x| -> u16 {
            match x.is_empty() {
                true => return 1,
                false => return 0,
            }
        })
        .sum();
    empty_column_count -= match game.columns[to].is_empty() {
        true => 1,
        false => 0,
    };

    let empty_cell_count: u16 = game
        .cells
        .iter()
        .map(|x| -> u16 {
            match x.is_none() {
                true => return 1,
                false => return 0,
            }
        })
        .sum();

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

    pub fn move_card_to_foundation(game: &mut Game, card: Card) -> Result<(), ()> {
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

        Ok(())
    }

    pub fn fit_together(upper: &Card, lower: &Card) -> bool {
        if *upper.rank() == Rank::Ace {
            return false;
        }

        let is_red = |card: Card| -> bool {
            return *card.suit() == Suit::Hearts || *card.suit() == Suit::Diamonds;
        };

        if is_red(*lower) == is_red(*upper) {
            return false;
        }

        let over_rank = *upper.rank() as u8;
        let under_rank = *lower.rank() as u8;

        return over_rank - 1 == under_rank;
    }
}

#[cfg(test)]
mod test;
