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

use std::{cell::RefCell, fmt::Debug, rc::Rc};

use rand::Rng;

use crate::{game::Game, journey_handler::{journey_repository::PersistJourney, JourneyHandler}, r#move::{apply, automove, Move}};

/// A structure to hold a game and its history.
#[derive(Debug)]
pub struct GameHandler<T>
where
    T: PersistJourney,
    T: Debug,
{
    /// The current game in its current state, if any.
    pub game: Option<Game>,
    journey_handler: Rc<RefCell<JourneyHandler<T>>>,
    history: Vec<Game>,
}

impl<T> GameHandler<T>
where
    T: PersistJourney,
    T: Debug,
{
    /// Creates a new instance containing the given journey handler.
    pub fn new(journey_handler: Rc<RefCell<JourneyHandler<T>>>) -> Self {
        GameHandler {
            game: None,
            journey_handler: journey_handler.clone(),
            history: vec![],
        }
    }

    /// Replaces the currently held game and its history (if any)
    /// with the game defined by the given ID.
    pub fn game_from_id(&mut self, id: u16) {
        self.game = Some(Game::from_id(id));
        self.history.clear();
    }

    /// Replaces the currently held game and its history (if any)
    /// with a random solvable game defined by an ID in the range 1 to 64000.
    pub fn random_game(&mut self) {
        // note that other known unsolvable games are currently out of scope
        // they are: 146_692, 186_216, 455_889, 495_505, 512_118, 517_776, 781_948
        const UNSOLVABLE_GAME: u16 = 11_982;

        loop {
            let candidate = rand::rng().random_range(1u16..64001u16);

            if candidate != UNSOLVABLE_GAME {
                self.game_from_id(candidate);
                break;
            }
        }
    }

    /// Make a move on the currently held game.
    pub fn make_move(&mut self, mv: Move) -> Result<(), ()> {
        if self.game.is_none() || self.game.as_mut().unwrap().is_won() {
            return Err(());
        }

        let new_state = apply(self.game.as_mut().unwrap(), mv);

        if let Err(()) = new_state {
            return Err(());
        }

        self.history.push(self.game.as_mut().unwrap().clone());

        let mut new_state = new_state.unwrap();

        while let Some(automoved) = automove(&new_state) {
            new_state = automoved
        }

        self.game = Some(new_state);

        if self.game.as_ref().unwrap().is_won() {
            self.journey_handler
                .borrow_mut()
                .receive_notification_game_won(self.game.as_ref().unwrap().id);
        }

        return Ok(());
    }

    /// Return the held game to its state before the last move, if any.
    pub fn revert(&mut self) -> Result<(), ()> {
        if self.history.is_empty() || self.game.as_ref().is_some_and(|x| x.is_won()) {
            return Err(());
        }

        self.game = Some(self.history.pop().unwrap());

        return Ok(());
    }
}

#[cfg(test)]
mod test;
