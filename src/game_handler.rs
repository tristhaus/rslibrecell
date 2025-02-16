use rand::Rng;

use crate::game::Game;
use crate::r#move::{apply, automove, Move};

/// A structure to hold a game and its history.
pub struct GameHandler {
    /// The current game in its current state, if any.
    pub game: Option<Game>,
    history: Vec<Game>,
}

impl GameHandler {
    /// Creates a new default instance.
    pub fn new() -> GameHandler {
        GameHandler {
            game: None,
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
    /// with a random game defined by an ID in the range 1 to 64000.
    pub fn random_game(&mut self) {
        self.game = Some(Game::from_id(rand::rng().random_range(1..64001 as u16)));
        self.history.clear();
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

        loop {
            let automoved = automove(&new_state);

            if let Some(automoved) = automoved {
                new_state = automoved
            } else {
                break;
            }
        }

        self.game = Some(new_state);
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
