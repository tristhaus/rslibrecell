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

use mockall::predicate;

use crate::journey_handler::journey_repository::MockPersistJourney;

use super::*;

#[test]
fn new_reads_from_repository() {
    let mut repository = MockPersistJourney::new();
    repository
        .expect_read()
        .once()
        .return_const((GameId(123), vec![GameId(117), GameId(118)]));
    repository.expect_write().never().return_const(());

    let instance = JourneyHandler::new(repository);

    assert_eq!(GameId(123), instance.next);
    assert_eq!(vec![GameId(117), GameId(118)], instance.skipped);
    assert_eq!(
        (GameId(123), vec![GameId(117), GameId(118)]),
        instance.next_game_ids()
    );
}

#[test]
fn receive_notification_game_won_with_unrelated_works() {
    let mut repository = MockPersistJourney::new();
    repository
        .expect_read()
        .once()
        .return_const((GameId(123), vec![GameId(117), GameId(118)]));
    repository.expect_write().never().return_const(());

    let mut instance = JourneyHandler::new(repository);

    instance.receive_notification_game_won(GameId(1));

    assert_eq!(
        (GameId(123), vec![GameId(117), GameId(118)]),
        instance.next_game_ids()
    );
}

#[test]
fn receive_notification_game_won_with_next_works() {
    let mut repository = MockPersistJourney::new();
    repository
        .expect_read()
        .once()
        .return_const((GameId(123), vec![GameId(117), GameId(118)]));
    repository
        .expect_write()
        .once()
        .with(
            predicate::eq(GameId(124)),
            predicate::eq(vec![GameId(117), GameId(118)]),
        )
        .return_const(());

    let mut instance = JourneyHandler::new(repository);

    instance.receive_notification_game_won(GameId(123));

    assert_eq!(
        (GameId(124), vec![GameId(117), GameId(118)]),
        instance.next_game_ids()
    );
}

#[test]
fn receive_notification_game_won_with_a_skipped_works() {
    let mut repository = MockPersistJourney::new();
    repository
        .expect_read()
        .once()
        .return_const((GameId(123), vec![GameId(117), GameId(118)]));
    repository
        .expect_write()
        .once()
        .with(predicate::eq(GameId(123)), predicate::eq(vec![GameId(118)]))
        .return_const(());

    let mut instance = JourneyHandler::new(repository);

    instance.receive_notification_game_won(GameId(117));

    assert_eq!((GameId(123), vec![GameId(118)]), instance.next_game_ids());
}

#[test]
fn skip_next_game_works() {
    let mut repository = MockPersistJourney::new();
    repository
        .expect_read()
        .once()
        .return_const((GameId(123), vec![GameId(117), GameId(118)]));
    repository
        .expect_write()
        .once()
        .with(
            predicate::eq(GameId(124)),
            predicate::eq(vec![GameId(117), GameId(118), GameId(123)]),
        )
        .return_const(());

    let mut instance = JourneyHandler::new(repository);

    instance.skip_next_game();

    assert_eq!(
        (GameId(124), vec![GameId(117), GameId(118), GameId(123)]),
        instance.next_game_ids()
    );
}
