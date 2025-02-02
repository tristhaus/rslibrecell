use super::*;

mod helper {
    use super::*;

    pub fn assert_move_succeeds(input: &str, mv: Move, reference: &str) {
        let before = Game::try_from(input).unwrap();
        let expected = Game::try_from(reference).unwrap();

        let after = apply(&before, mv).unwrap();

        assert_eq!(expected, after);
    }

    pub fn assert_move_fails(input: &str, mv: Move) {
        let before = Game::try_from(input).unwrap();

        let error = apply(&before, mv);

        assert!(error.is_err());
    }
}

#[test]
fn apply_cell_cell_works() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Cell { i: 3 },
        to: Location::Cell { i: 2 },
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  Q♥  .. || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_cannot_move_from_nonexistent_cell() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Cell { i: 4 },
        to: Location::Cell { i: 2 },
    };

    helper::assert_move_fails(input, mv);
}

#[test]
fn apply_cannot_move_to_nonexistent_cell() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Cell { i: 3 },
        to: Location::Cell { i: 4 },
    };

    helper::assert_move_fails(input, mv);
}

#[test]
fn apply_cannot_move_to_full_cell() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Cell { i: 3 },
        to: Location::Cell { i: 0 },
    };

    helper::assert_move_fails(input, mv);
}

#[test]
fn apply_cannot_move_from_empty_cell() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Cell { i: 1 },
        to: Location::Cell { i: 2 },
    };

    helper::assert_move_fails(input, mv);
}

#[test]
fn apply_cell_foundation_ace_works() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  A♥  2♦  Q♥ || 2♣  ..  ..  A♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Cell { i: 1 },
        to: Location::Foundation,
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  2♦  Q♥ || 2♣  ..  A♥  A♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_cell_foundation_two_works() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  2♦  Q♥ || 2♣  ..  A♥  A♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Cell { i: 2 },
        to: Location::Foundation,
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_cannot_move_from_foundation() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Foundation,
        to: Location::Cell { i: 2 },
    };

    helper::assert_move_fails(input, mv);
}

#[test]
fn apply_cell_to_foundation_without_correct_rank_fails() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  3♥  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Cell { i: 1 },
        to: Location::Foundation,
    };

    helper::assert_move_fails(input, mv);
}

#[test]
fn apply_column_cell_works() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 0 },
        to: Location::Cell { i: 1 },
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  4♦  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "      2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_cannot_move_from_nonexistent_column() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 8 },
        to: Location::Cell { i: 2 },
    };

    helper::assert_move_fails(input, mv);
}

#[test]
fn apply_cannot_move_to_nonexistent_column() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Cell { i: 3 },
        to: Location::Column { i: 8 },
    };

    helper::assert_move_fails(input, mv);
}

#[test]
fn apply_cannot_move_from_empty_column() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 5 },
        to: Location::Cell { i: 2 },
    };

    helper::assert_move_fails(input, mv);
}

#[test]
fn apply_column_foundation_ace_works() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  2♦  Q♥ || 2♣  ..  ..  A♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "  A♥  J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 0 },
        to: Location::Foundation,
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  2♦  Q♥ || 2♣  ..  A♥  A♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_column_foundation_two_works() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  A♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠      2♦  J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 6 },
        to: Location::Foundation,
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_column_to_foundation_without_correct_rank_fails() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣      3♥                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 3 },
        to: Location::Foundation,
    };

    helper::assert_move_fails(input, mv);
}

#[test]
fn apply_column_to_same_column_fails() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 2 },
        to: Location::Column { i: 2 },
    };

    helper::assert_move_fails(input, mv);
}

#[test]
fn apply_cell_to_empty_column_works() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Cell { i: 0 },
        to: Location::Column { i: 5 },
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " ..  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠  T♣  5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_cell_to_filled_matching_column_works() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  3♥  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Cell { i: 2 },
        to: Location::Column { i: 1 },
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_cell_to_filled_nonmatching_column_fails() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠  2♠             \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Cell { i: 0 },
        to: Location::Column { i: 0 },
    };

    helper::assert_move_fails(input, mv);
}

#[test]
fn apply_column_one_card_to_column_empty_moves_one_card() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♠  4♥  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠          Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠                 \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 6 },
        to: Location::Column { i: 5 },
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♠  4♥  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠  5♣      K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠          Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠                 \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_column_three_cards_to_column_empty_moves_one_card() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♠  4♥  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠          Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠                 \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 7 },
        to: Location::Column { i: 5 },
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♠  4♥  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠  J♣  5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠          Q♦ \n",
        "      J♦  Q♠  3♣  3♠             \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠                 \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_column_one_to_column_filled_matching_moves_one_card() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♠  4♥  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠  7♣  5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠          Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣      Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠                 \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 5 },
        to: Location::Column { i: 2 },
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♠  4♥  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠          Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠                 \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_column_to_filled_nonmatching_column_fails() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♠  4♥  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠  7♣  5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠          Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣      Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠                 \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 6 },
        to: Location::Column { i: 2 },
    };

    helper::assert_move_fails(input, mv);
}

#[test]
fn apply_column_seven_to_column_empty_supermoves_two_cards() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♠  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠                 \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 3 },
        to: Location::Column { i: 5 },
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♠  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠  T♠  5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠  9♥  4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠                         \n",
        "      5♥                         \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_column_four_to_column_filled_matching_supermoves_four_cards() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♠  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠                 \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 2 },
        to: Location::Column { i: 7 },
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♠  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠      8♦  K♥          T♦ \n",
        "      9♦      7♦  T♥          9♣ \n",
        "      6♥      6♦  5♦          8♥ \n",
        "      8♣      Q♣  4♠          7♣ \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠                 \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_column_six_to_column_filled_matching_supermoves_six_cards() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "  2♠  J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠                 \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 1 },
        to: Location::Column { i: 3 },
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "  2♠  J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "          7♣  Q♣  4♠             \n",
        "              J♥  3♦             \n",
        "              T♠                 \n",
        "              9♥                 \n",
        "              8♣                 \n",
        "              7♥                 \n",
        "              6♠                 \n",
        "              5♥                 \n",
        "              4♣                 \n",
        "              3♥                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_column_six_to_column_filled_matching_fails_run_too_large() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♠  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠                 \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 1 },
        to: Location::Column { i: 3 },
    };

    helper::assert_move_fails(input, mv);
}

#[test]
fn apply_column_three_to_column_filled_nonmatching_fails() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " ..  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠      4♥  Q♦ \n",
        "  2♠  J♦  Q♠  3♣  3♠          J♣ \n",
        "  T♣  9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      6♥  8♥  6♦  5♦             \n",
        "      8♣  7♣  Q♣  4♠             \n",
        "      7♥      J♥  3♦             \n",
        "      6♠      T♠                 \n",
        "      5♥      9♥                 \n",
        "      4♣                         \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 3 },
        to: Location::Column { i: 2 },
    };

    helper::assert_move_fails(input, mv);
}

#[test]
fn apply_column_six_to_column_filled_matching_supermoves_six_cards_using_multiple_columns() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♦  ..  Q♥ || 2♣  ..  A♥  A♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠          K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠          Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "          8♥  6♦  5♦             \n",
        "          7♣  Q♣  4♠             \n",
        "          6♥  J♥  3♦             \n",
        "          5♣  T♠                 \n",
        "          4♥  9♥                 \n",
        "              8♣                 \n",
        "              7♥                 \n",
        "              6♠                 \n",
        "              5♥                 \n",
        "              4♣                 \n",
        "              3♥                 \n",
        "              2♠                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 3 },
        to: Location::Column { i: 1 },
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♦  ..  Q♥ || 2♣  ..  A♥  A♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  A♠          K♣ \n",
        "  4♦  2♥  7♠  6♣  8♠          Q♦ \n",
        "      J♦  Q♠  3♣  3♠          J♣ \n",
        "      9♠  T♦  8♦  K♥             \n",
        "      9♦  9♣  7♦  T♥             \n",
        "      8♣  8♥  6♦  5♦             \n",
        "      7♥  7♣  Q♣  4♠             \n",
        "      6♠  6♥  J♥  3♦             \n",
        "      5♥  5♣  T♠                 \n",
        "      4♣  4♥  9♥                 \n",
        "      3♥                         \n",
        "      2♠                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_column_five_to_column_filled_matching_supermoves_four_cards_to_match() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♠  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  2♥  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  J♦  7♠  6♣  8♠      4♥  Q♦ \n",
        "      9♠  Q♠  3♣  3♠          J♣ \n",
        "      9♦  J♠  8♦  K♥             \n",
        "      6♥  T♦  7♦  T♥             \n",
        "      8♣  9♣  6♦  5♦             \n",
        "      7♥  8♥  Q♣  4♠             \n",
        "      6♠  7♣  J♥  3♦             \n",
        "      5♥      T♠                 \n",
        "      4♣      9♥                 \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 2 },
        to: Location::Column { i: 7 },
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♠  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  2♥  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  J♦  7♠  6♣  8♠      4♥  Q♦ \n",
        "      9♠  Q♠  3♣  3♠          J♣ \n",
        "      9♦  J♠  8♦  K♥          T♦ \n",
        "      6♥      7♦  T♥          9♣ \n",
        "      8♣      6♦  5♦          8♥ \n",
        "      7♥      Q♣  4♠          7♣ \n",
        "      6♠      J♥  3♦             \n",
        "      5♥      T♠                 \n",
        "      4♣      9♥                 \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_column_three_to_column_empty_supermoves_three_cards_cells_only() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  2♥  K♠  K♦  A♠      5♣  K♣ \n",
        "  4♦  J♦  7♠  6♣  8♠      4♥  Q♦ \n",
        "  2♠  9♠  Q♠  3♣  3♠          J♣ \n",
        "      9♦  J♠  8♦  K♥             \n",
        "      6♥  T♦  7♦  T♥             \n",
        "      8♣  9♣  6♦  5♦             \n",
        "      7♥  8♥  Q♣  4♠             \n",
        "      6♠  7♣  J♥  3♦             \n",
        "      5♥      T♠                 \n",
        "      4♣      9♥                 \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 4 },
        to: Location::Column { i: 5 },
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  2♥  K♠  K♦  A♠  5♦  5♣  K♣ \n",
        "  4♦  J♦  7♠  6♣  8♠  4♠  4♥  Q♦ \n",
        "  2♠  9♠  Q♠  3♣  3♠  3♦      J♣ \n",
        "      9♦  J♠  8♦  K♥             \n",
        "      6♥  T♦  7♦  T♥             \n",
        "      8♣  9♣  6♦                 \n",
        "      7♥  8♥  Q♣                 \n",
        "      6♠  7♣  J♥                 \n",
        "      5♥      T♠                 \n",
        "      4♣      9♥                 \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}

#[test]
fn apply_column_five_to_column_filled_matching_supermoves_four_cards_columns_only() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♠  5♣  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  2♥  K♠  K♦  A♠          K♣ \n",
        "  4♦  J♦  7♠  6♣  8♠          Q♦ \n",
        "  4♥  9♠  Q♠  3♣  3♠          J♣ \n",
        "      9♦  J♠  8♦  K♥             \n",
        "      6♥  T♦  7♦  T♥             \n",
        "      8♣  9♣  6♦  5♦             \n",
        "      7♥  8♥  Q♣  4♠             \n",
        "      6♠  7♣  J♥  3♦             \n",
        "      5♥      T♠                 \n",
        "      4♣      9♥                 \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    let mv = Move {
        from: Location::Column { i: 2 },
        to: Location::Column { i: 7 },
    };

    let reference = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  2♠  5♣  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  2♥  K♠  K♦  A♠          K♣ \n",
        "  4♦  J♦  7♠  6♣  8♠          Q♦ \n",
        "  4♥  9♠  Q♠  3♣  3♠          J♣ \n",
        "      9♦  J♠  8♦  K♥          T♦ \n",
        "      6♥      7♦  T♥          9♣ \n",
        "      8♣      6♦  5♦          8♥ \n",
        "      7♥      Q♣  4♠          7♣ \n",
        "      6♠      J♥  3♦             \n",
        "      5♥      T♠                 \n",
        "      4♣      9♥                 \n",
        "      3♥                         \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n",
        "                                 \n"
    );

    helper::assert_move_succeeds(input, mv, reference);
}
