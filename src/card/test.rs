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

use super::*;

#[test]
fn card_partialeq_trait_works() {
    let card1 = Card::from_id(0);
    let card2 = Card::from_id(0);
    let card3 = Card::from_id(42);

    assert_eq!(card1, card2);
    assert_ne!(card1, card3);
}

#[test]
fn card_can_be_constructed_from_id() {
    let card1 = Card::from_id(0);

    assert_eq!(card1.suit, Suit::Clubs);
    assert_eq!(card1.rank, Rank::Ace);

    let card2 = Card::from_id(42);

    assert_eq!(card2.suit, Suit::Hearts);
    assert_eq!(card2.rank, Rank::Jack);
}

#[test]
#[should_panic]
fn card_when_given_large_id_panics() {
    let _ = Card::from_id(52);
}

#[test]
fn card_display_trait_works() {
    let card1 = Card::from_id(0);

    assert_eq!(card1.to_string(), "A♣");

    let card2 = Card::from_id(42);

    assert_eq!(card2.to_string(), "J♥");
}

#[test]
fn card_tryfrom_ref_str_with_unicode_representation_works() {
    let card1 = Card::try_from("T♣").unwrap();

    assert_eq!(Card::from_id(36), card1);

    let card2 = Card::try_from("J♥").unwrap();

    assert_eq!(Card::from_id(42), card2);
}

#[test]
fn card_tryfrom_with_short_string_errors() {
    let _ = Card::try_from("T").expect_err("should have error");
}

#[test]
fn card_tryfrom_with_bad_input1_errors() {
    let _ = Card::try_from("R♣").expect_err("should have error");
}

#[test]
fn card_tryfrom_with_bad_input2_errors() {
    let _ = Card::try_from("T?").expect_err("should have error");
}
