use std::convert::TryFrom;
use std::{collections, fmt};

use crate::card::Card;

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

                        let rank = *card.rank() as u8;

                        for r in 0..(rank + 1) {
                            let id = r * 4 + *card.suit() as u8;
                            let foundation_card = Card::from_id(id);
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

impl Game {
    pub fn is_won(&self) -> bool {
        let count: usize = self.foundations.iter().map(|x| x.len()).sum();

        count == 52
    }
}

#[cfg(test)]
mod test;
