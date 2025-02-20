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

/// Contains a certain pseudo-random number generator.
mod prng;

use std::convert::TryFrom;
use std::{collections, fmt};

use crate::card::Card;
use crate::game::prng::Prng;

/// Defines a FreeCell game.
#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    /// The ID of the game.
    pub id: u16,
    /// The cells (top-left) of the game.
    pub cells: [Option<Card>; 4],
    /// The foundations (top-right, target area) of the game.
    pub foundations: [Vec<Card>; 4],
    /// The columns (bottom) of the game.
    pub columns: [Vec<Card>; 8],
}

impl fmt::Display for Game {
    /// Provides the canonical representation of the game,
    /// which should be parseable via `try_from`.
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
            result += " \n";
        }

        write!(f, "{}", result)
    }
}

impl TryFrom<&str> for Game {
    type Error = ();

    /// Attempts to create a game from its canonical string representation,
    /// compare `fmt`.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut cells = [
            Option::<Card>::None,
            Option::<Card>::None,
            Option::<Card>::None,
            Option::<Card>::None,
        ];
        let mut foundations = [vec![], vec![], vec![], vec![]];
        let mut columns = [
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];

        let mut check_set: collections::HashSet<Card> = collections::HashSet::<Card>::new();

        let lines: Vec<&str> = value.lines().collect();

        if lines.len() < 5 {
            return Err(());
        }

        let title_line = lines[0];

        let mut read_pound_sign = false;
        let mut game_id = String::new();
        for c in title_line.chars() {
            if c == '#' {
                read_pound_sign = true;
                continue;
            }

            if read_pound_sign {
                game_id.push(c);
            }
        }

        let game_id = match game_id.trim().parse::<u16>() {
            Ok(id) => id,
            Err(_) => return Err(()),
        };

        let cells_foundations_line = lines[2];

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

                    if card.is_some() && !check_set.insert(card.unwrap()) {
                        return Err(());
                    }
                } else {
                    if card.is_some() {
                        let card = card.unwrap();
                        let foundation_index = (index - 19) / 4;

                        let rank = card.rank as u8;

                        for r in 0..(rank + 1) {
                            let id = r * 4 + card.suit as u8;
                            let foundation_card = Card::from_id(id);
                            foundations[foundation_index].push(foundation_card);
                            if !check_set.insert(foundation_card) {
                                return Err(());
                            }
                        }
                    }
                }

                card_helper.clear();
            }
        }

        for columns_line in &lines[4..] {
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
                    if !check_set.insert(card) {
                        return Err(());
                    }
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

impl Game {
    /// Generates the game associated with the given ID.
    ///
    /// # Panics
    /// The method will panic if the underlying code, especially the PRNG, panics.
    pub(crate) fn from_id(id: u16) -> Game {
        let mut prng = Prng { state: id as u32 };

        let mut game = Game {
            id,
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

        let mut deck: Vec<Card> = (0..52).map(|x| Card::from_id(x)).collect::<Vec<_>>();

        let mut column_index = 0;
        while !deck.is_empty() {
            let deck_index = prng.get_next() as usize % deck.len();

            let last_deck_index = deck.len() - 1;
            deck.swap(deck_index, last_deck_index);
            game.columns[column_index].push(deck.pop().unwrap());

            column_index = (column_index + 1) % 8;
        }

        return game;
    }

    /// Returns a flag indicating whether the game is won,
    /// *i.e.* all cards are on the foundations.
    pub(crate) fn is_won(&self) -> bool {
        let count: usize = self.foundations.iter().fold(0, |acc, x| acc + x.len());

        count == 52
    }
}

#[cfg(test)]
mod test;
