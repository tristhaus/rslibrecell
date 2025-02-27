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

use journey_repository::PersistJourney;

/// Productive implementation of `HandleJourney`.
#[derive(Debug)]
pub struct JourneyHandler<T>
where
    T: PersistJourney,
{
    next: u16,
    repository: T,
    skipped: Vec<u16>,
}

impl<T> JourneyHandler<T>
where
    T: PersistJourney,
{
    /// Obtains the ID of the next game and any skipped games.
    pub fn next_game_ids(&self) -> (u16, Vec<u16>) {
        (self.next, self.skipped.clone())
    }

    /// Receives the notification that a game was won,
    /// as indicated by its ID.
    pub fn receive_notification_game_won(&mut self, id: u16) -> () {
        if id == self.next {
            self.next = self.next + 1;
            self.persist();
        } else {
            let position = self.skipped.iter().position(|x| *x == id);

            if let Some(position) = position {
                _ = self.skipped.remove(position);
                self.persist();
            }
        }
    }

    /// Skips the next game, marks it as such,
    /// and moves to the game after that.
    pub fn skip_next_game(&mut self) -> () {
        if self.next > 64000 {
            return;
        }

        self.skipped.push(self.next);
        self.next = self.next + 1;
        self.persist();
    }
}

impl<T> JourneyHandler<T>
where
    T: PersistJourney,
{
    /// Constructs a new `JourneyHandler` instance.
    pub fn new(repository: T) -> JourneyHandler<T> {
        let data = repository.read();

        JourneyHandler {
            next: data.0,
            skipped: data.1.clone(),
            repository,
        }
    }

    /// Persists the current state of the journey using the repository.
    fn persist(&self) -> () {
        self.repository.write(self.next, self.skipped.clone());
    }
}

/// Contains logic to persist a journey.
pub mod journey_repository;

#[cfg(test)]
mod test;
