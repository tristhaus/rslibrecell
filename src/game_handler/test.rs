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

use crate::{
    card::Card, journey_handler::journey_repository::MockPersistJourney, r#move::Location,
};

use super::*;

#[test]
fn gamehandler_game_from_id_contains_expected() {
    let mut game_handler = helper::setup_game_handler();

    assert!(game_handler.game.is_none());

    game_handler.game_from_id(1);

    assert!(game_handler
        .game
        .as_ref()
        .is_some_and(|x| x.columns[0][0] == Card::from_str("J♦")));

    game_handler.game_from_id(2);

    assert!(game_handler
        .game
        .as_ref()
        .is_some_and(|x| x.columns[0][0] == Card::from_str("Q♦")));
}

#[test]
fn gamehandler_random_game_creates_different_game() {
    let mut game_handler = helper::setup_game_handler();
    game_handler.game_from_id(1);

    assert!(game_handler
        .game
        .as_ref()
        .is_some_and(|x| x.columns[0][0] == Card::from_str("J♦")));

    for _ in 0..10 {
        game_handler.random_game();

        if game_handler
            .game
            .as_ref()
            .is_some_and(|x| x.columns[0][0] != Card::from_str("J♦"))
        {
            return;
        }
    }

    assert!(false);
}

#[test]
fn gamehandler_make_move_works_correctly() {
    let mut game_handler = helper::setup_game_handler();
    game_handler.game_from_id(123);

    let initial = game_handler.game.as_ref().unwrap().to_string();

    let initial_reference = concat!(
        "RustLibreCell                #123 \n",
        "\n",
        " ..  ..  ..  .. || ..  ..  ..  .. \n",
        "--------------------------------- \n",
        "  7♣  8♥  7♦  6♦  3♠  6♥  K♣  3♣  \n",
        "  T♣  8♣  5♦  4♣  A♣  A♥  8♦  J♣  \n",
        "  4♥  K♦  4♠  J♠  7♠  2♥  4♦  Q♥  \n",
        "  2♠  T♥  T♠  7♥  5♠  9♠  2♣  6♠  \n",
        "  A♦  8♠  6♣  K♥  Q♦  T♦  J♥  9♥  \n",
        "  2♦  A♠  9♦  9♣  Q♠  5♥  J♦  K♠  \n",
        "  Q♣  3♥  3♦  5♣                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        ""
    );

    assert_eq!(initial_reference, initial);

    assert!(game_handler
        .make_move(Move {
            from: Location::Column { i: 6 },
            to: Location::Column { i: 0 }
        })
        .is_ok());

    assert!(game_handler
        .make_move(Move {
            from: Location::Column { i: 7 },
            to: Location::Cell { i: 1 }
        })
        .is_ok());

    let state = game_handler.game.as_ref().unwrap().to_string();

    let state_reference = concat!(
        "RustLibreCell                #123 \n",
        "\n",
        " ..  K♠  ..  .. || ..  ..  ..  .. \n",
        "--------------------------------- \n",
        "  7♣  8♥  7♦  6♦  3♠  6♥  K♣  3♣  \n",
        "  T♣  8♣  5♦  4♣  A♣  A♥  8♦  J♣  \n",
        "  4♥  K♦  4♠  J♠  7♠  2♥  4♦  Q♥  \n",
        "  2♠  T♥  T♠  7♥  5♠  9♠  2♣  6♠  \n",
        "  A♦  8♠  6♣  K♥  Q♦  T♦  J♥  9♥  \n",
        "  2♦  A♠  9♦  9♣  Q♠  5♥          \n",
        "  Q♣  3♥  3♦  5♣                  \n",
        "  J♦                              \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        ""
    );

    assert_eq!(state_reference, state);
}

#[test]
fn gamehandler_make_move_rejects_illegal_move() {
    let mut game_handler = helper::setup_game_handler();
    game_handler.game_from_id(123);

    assert!(game_handler
        .make_move(Move {
            from: Location::Column { i: 4 },
            to: Location::Column { i: 7 }
        })
        .is_err());
}

#[test]
fn gamehandler_win_entire_game_and_trigger_journey_handler() {
    let mut mock = MockPersistJourney::new();
    mock.expect_read().return_const((123, vec![100, 118]));
    mock.expect_write()
        .with(predicate::eq(123), predicate::eq(vec![118]))
        .once()
        .return_const(());

    let journey_handler = Rc::new(RefCell::new(JourneyHandler::new(mock)));
    let mut game_handler = GameHandler::new(journey_handler);

    game_handler.game_from_id(100);

    let mut make_move_and_assert = |mv: Move| {
        assert!(game_handler.make_move(mv).is_ok());
    };

    make_move_and_assert(Move {
        from: Location::Column { i: 6 },
        to: Location::Column { i: 1 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 6 },
        to: Location::Cell { i: 0 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 0 },
        to: Location::Column { i: 6 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 4 },
        to: Location::Column { i: 6 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 6 },
        to: Location::Column { i: 4 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 0 },
        to: Location::Foundation,
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 0 },
        to: Location::Column { i: 4 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 0 },
        to: Location::Column { i: 6 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 0 },
        to: Location::Cell { i: 1 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 6 },
        to: Location::Column { i: 0 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 6 },
        to: Location::Column { i: 0 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 4 },
        to: Location::Column { i: 0 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 7 },
        to: Location::Cell { i: 2 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 1 },
        to: Location::Column { i: 6 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 7 },
        to: Location::Cell { i: 3 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 5 },
        to: Location::Column { i: 2 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 3 },
        to: Location::Column { i: 5 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 3 },
        to: Location::Cell { i: 2 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 4 },
        to: Location::Column { i: 7 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 3 },
        to: Location::Column { i: 7 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 4 },
        to: Location::Column { i: 7 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 5 },
        to: Location::Column { i: 7 },
    });
    make_move_and_assert(Move {
        from: Location::Cell { i: 2 },
        to: Location::Column { i: 3 },
    });
    make_move_and_assert(Move {
        from: Location::Cell { i: 3 },
        to: Location::Column { i: 3 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 2 },
        to: Location::Column { i: 4 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 2 },
        to: Location::Cell { i: 2 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 2 },
        to: Location::Column { i: 6 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 2 },
        to: Location::Column { i: 3 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 2 },
        to: Location::Cell { i: 3 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 5 },
        to: Location::Column { i: 2 },
    });
    make_move_and_assert(Move {
        from: Location::Column { i: 1 },
        to: Location::Cell { i: 0 },
    });

    assert!(game_handler.game.as_ref().unwrap().is_won());

    // no move on won game
    assert!(game_handler
        .make_move(Move {
            from: Location::Column { i: 0 },
            to: Location::Column { i: 1 },
        })
        .is_err());

    // no revert on won game
    assert!(game_handler.revert().is_err())
}

#[test]
fn gamehandler_revert_works_correctly() {
    let mut game_handler = helper::setup_game_handler();
    game_handler.game_from_id(123);

    let initial_reference = concat!(
        "RustLibreCell                #123 \n",
        "\n",
        " ..  ..  ..  .. || ..  ..  ..  .. \n",
        "--------------------------------- \n",
        "  7♣  8♥  7♦  6♦  3♠  6♥  K♣  3♣  \n",
        "  T♣  8♣  5♦  4♣  A♣  A♥  8♦  J♣  \n",
        "  4♥  K♦  4♠  J♠  7♠  2♥  4♦  Q♥  \n",
        "  2♠  T♥  T♠  7♥  5♠  9♠  2♣  6♠  \n",
        "  A♦  8♠  6♣  K♥  Q♦  T♦  J♥  9♥  \n",
        "  2♦  A♠  9♦  9♣  Q♠  5♥  J♦  K♠  \n",
        "  Q♣  3♥  3♦  5♣                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        "                                  \n",
        ""
    );

    assert!(game_handler
        .make_move(Move {
            from: Location::Column { i: 6 },
            to: Location::Column { i: 0 }
        })
        .is_ok());

    assert!(game_handler
        .make_move(Move {
            from: Location::Column { i: 7 },
            to: Location::Cell { i: 1 }
        })
        .is_ok());

    assert!(game_handler.revert().is_ok());
    assert!(game_handler.revert().is_ok());

    let state = game_handler.game.as_ref().unwrap().to_string();

    assert_eq!(initial_reference, state);
}

#[test]
fn gamehandler_revert_errors_on_initial_state() {
    let mut game_handler = helper::setup_game_handler();
    game_handler.game_from_id(123);

    assert!(game_handler.revert().is_err());

    assert!(game_handler
        .make_move(Move {
            from: Location::Column { i: 6 },
            to: Location::Column { i: 0 }
        })
        .is_ok());

    assert!(game_handler.revert().is_ok());

    assert!(game_handler.revert().is_err());
}

mod helper {
    use super::*;

    pub fn setup_game_handler() -> GameHandler<MockPersistJourney> {
        let mut mock = MockPersistJourney::new();
        mock.expect_read().return_const((123, vec![117, 118]));
        mock.expect_write().return_const(());
        let journey_handler = Rc::new(RefCell::new(JourneyHandler::new(mock)));
        GameHandler::new(journey_handler)
    }
}
