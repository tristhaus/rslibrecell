use rand::Rng;

use crate::game::Game;
use crate::r#move::{apply, Move};

pub struct GameHandler {
    pub game: Option<Game>,
}

impl GameHandler {
    pub fn new() -> GameHandler {
        GameHandler { game: None }
    }

    pub fn game_from_id(&mut self, id: u16) {
        self.game = Some(Game::from_id(id));
    }

    pub fn random_game(&mut self) {
        self.game = Some(Game::from_id(rand::rng().random_range(1..64000 as u16)));
    }

    pub fn make_move(&mut self, mv: Move) -> Result<(), ()> {
        if self.game.is_none() || self.game.as_mut().unwrap().is_won() {
            return Err(());
        }

        let new_state = apply(self.game.as_mut().unwrap(), mv);

        match new_state {
            Ok(new_state) => {
                self.game = Some(new_state);
                return Ok(());
            }
            Err(()) => return Err(()),
        }
    }
}

#[cfg(test)]
mod test;
