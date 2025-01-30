use super::*;

#[test]
fn game_can_be_cloned_yields_independent_instances() {
    let mut game = Game {
        id: 17,
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

    let clone = game.clone();

    game.id = 11;
    game.cells[1] = Some(Card::from_id(2));
    game.foundations[2].push(Card::from_id(3));
    game.columns[3].push(Card::from_id(4));

    assert_eq!(17, clone.id);
    assert_eq!(11, game.id);

    assert_eq!(None, clone.cells[1]);
    assert_eq!(Some(Card::from_id(2)), game.cells[1]);

    assert!(clone.foundations[2].is_empty());
    assert_eq!(Card::from_id(3), game.foundations[2][0]);

    assert!(clone.columns[3].is_empty());
    assert_eq!(Card::from_id(4), game.columns[3][0]);
}

#[test]
fn game_partialeq_trait_works() {
    let mut game1 = Game {
        id: 17,
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

    game1.cells[1] = Some(Card::from_id(2));
    game1.foundations[2].push(Card::from_id(3));
    game1.columns[3].push(Card::from_id(4));

    let game2 = game1.clone();

    let mut game3 = Game {
        id: 17,
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

    game3.cells[1] = Some(Card::from_id(50));
    game3.foundations[2].push(Card::from_id(3));
    game3.columns[3].push(Card::from_id(4));

    let mut game4 = Game {
        id: 17,
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

    game4.cells[1] = Some(Card::from_id(2));
    game4.foundations[2].push(Card::from_id(50));
    game4.columns[3].push(Card::from_id(4));

    let mut game5 = Game {
        id: 17,
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

    game5.cells[1] = Some(Card::from_id(2));
    game5.foundations[2].push(Card::from_id(3));
    game5.columns[3].push(Card::from_id(50));

    assert_eq!(game1, game2);
    assert_ne!(game1, game3);
    assert_ne!(game1, game4);
    assert_ne!(game1, game5);
}

#[test]
fn game_display_trait_works() {
    let reference = concat!(
        "RustLibreCell                 #42 \n",
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
        "                                 \n",
        "                                 \n"
    );
    let game = Game {
        id: 42,
        cells: [
            Some(Card::from_str("T♣")),
            None,
            None,
            Some(Card::from_str("Q♥")),
        ],
        foundations: [
            vec![Card::from_str("A♣"), Card::from_str("2♣")],
            vec![],
            vec![Card::from_str("A♥")],
            vec![Card::from_str("A♦"), Card::from_str("2♦")],
        ],
        columns: [
            vec![Card::from_str("5♠"), Card::from_str("4♦")],
            vec![
                Card::from_str("J♠"),
                Card::from_str("2♥"),
                Card::from_str("J♦"),
                Card::from_str("9♠"),
                Card::from_str("9♦"),
                Card::from_str("6♥"),
                Card::from_str("8♣"),
                Card::from_str("7♥"),
                Card::from_str("6♠"),
                Card::from_str("5♥"),
                Card::from_str("4♣"),
                Card::from_str("3♥"),
            ],
            vec![
                Card::from_str("K♠"),
                Card::from_str("7♠"),
                Card::from_str("Q♠"),
                Card::from_str("T♦"),
                Card::from_str("9♣"),
                Card::from_str("8♥"),
                Card::from_str("7♣"),
            ],
            vec![
                Card::from_str("K♦"),
                Card::from_str("6♣"),
                Card::from_str("3♣"),
                Card::from_str("8♦"),
                Card::from_str("7♦"),
                Card::from_str("6♦"),
                Card::from_str("Q♣"),
                Card::from_str("J♥"),
                Card::from_str("T♠"),
                Card::from_str("9♥"),
            ],
            vec![
                Card::from_str("A♠"),
                Card::from_str("8♠"),
                Card::from_str("3♠"),
                Card::from_str("K♥"),
                Card::from_str("T♥"),
                Card::from_str("5♦"),
                Card::from_str("4♠"),
                Card::from_str("3♦"),
                Card::from_str("2♠"),
            ],
            vec![],
            vec![Card::from_str("5♣"), Card::from_str("4♥")],
            vec![
                Card::from_str("K♣"),
                Card::from_str("Q♦"),
                Card::from_str("J♣"),
            ],
        ],
    };

    let result = format!("{}", game);

    assert_eq!(reference, result);
}

#[test]
fn game_tryfrom_ref_str_with_unicode_representation_works() {
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

    let reference = Game {
        id: 42,
        cells: [
            Some(Card::from_str("T♣")),
            None,
            None,
            Some(Card::from_str("Q♥")),
        ],
        foundations: [
            vec![Card::from_str("A♣"), Card::from_str("2♣")],
            vec![],
            vec![Card::from_str("A♥")],
            vec![Card::from_str("A♦"), Card::from_str("2♦")],
        ],
        columns: [
            vec![Card::from_str("5♠"), Card::from_str("4♦")],
            vec![
                Card::from_str("J♠"),
                Card::from_str("2♥"),
                Card::from_str("J♦"),
                Card::from_str("9♠"),
                Card::from_str("9♦"),
                Card::from_str("6♥"),
                Card::from_str("8♣"),
                Card::from_str("7♥"),
                Card::from_str("6♠"),
                Card::from_str("5♥"),
                Card::from_str("4♣"),
                Card::from_str("3♥"),
            ],
            vec![
                Card::from_str("K♠"),
                Card::from_str("7♠"),
                Card::from_str("Q♠"),
                Card::from_str("T♦"),
                Card::from_str("9♣"),
                Card::from_str("8♥"),
                Card::from_str("7♣"),
            ],
            vec![
                Card::from_str("K♦"),
                Card::from_str("6♣"),
                Card::from_str("3♣"),
                Card::from_str("8♦"),
                Card::from_str("7♦"),
                Card::from_str("6♦"),
                Card::from_str("Q♣"),
                Card::from_str("J♥"),
                Card::from_str("T♠"),
                Card::from_str("9♥"),
            ],
            vec![
                Card::from_str("A♠"),
                Card::from_str("8♠"),
                Card::from_str("3♠"),
                Card::from_str("K♥"),
                Card::from_str("T♥"),
                Card::from_str("5♦"),
                Card::from_str("4♠"),
                Card::from_str("3♦"),
                Card::from_str("2♠"),
            ],
            vec![],
            vec![Card::from_str("5♣"), Card::from_str("4♥")],
            vec![
                Card::from_str("K♣"),
                Card::from_str("Q♦"),
                Card::from_str("J♣"),
            ],
        ],
    };

    let game = Game::try_from(input).unwrap();

    assert_eq!(reference, game);
}

#[test]
fn game_tryfrom_empty_string_errors() {
    Game::try_from("").expect_err("should have error")
}

#[test]
fn game_tryfrom_no_cell_line_errors() {
    let input = "RustLibreCell                 #42

";

    Game::try_from(input).expect_err("should have error")
}

#[test]
fn game_tryfrom_bad_card_in_column_errors() {
    let input = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  A♥  2♦ \n",
        "--------------------------------- \n",
        "  5♠  J♠  K♠  K♦  XX      5♣  K♣ \n",
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

    Game::try_from(input).expect_err("should have error")
}

#[test]
fn game_tryfrom_card_missing_errors() {
    let ace_of_hearts_missing = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  ..  Q♥ || 2♣  ..  ..  2♦ \n",
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

    Game::try_from(ace_of_hearts_missing).expect_err("should have error")
}

#[test]
fn game_tryfrom_duplicated_card_errors() {
    let ace_of_diamonds_twice = concat!(
        "RustLibreCell                 #42\n",
        "\n",
        " T♣  ..  A♦  Q♥ || 2♣  ..  ..  2♦ \n",
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

    Game::try_from(ace_of_diamonds_twice).expect_err("should have error")
}
