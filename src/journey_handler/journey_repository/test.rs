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
fn deserialize_with_skipped() {
    let input: Vec<u8> = vec![0x00, 0x11, 0x00, 0x02, 0x00, 0x0b, 0x02, 0x03];

    let actual = DiskJourneyRepo::deserialize(&input);

    let expected = (17, vec![11, 515]);

    assert_eq!(actual, expected);
}

#[test]
fn deserialize_without_skipped() {
    let input: Vec<u8> = vec![0x00, 0x11, 0x00, 0x00];

    let actual = DiskJourneyRepo::deserialize(&input);

    let expected = (17, vec![]);

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn deserialize_too_little_data_sub4() {
    let input: Vec<u8> = vec![0x00, 0x11, 0x00];

    _ = DiskJourneyRepo::deserialize(&input);
}

#[test]
#[should_panic]
fn deserialize_too_little_data_mismatch() {
    let input: Vec<u8> = vec![0x00, 0x11, 0x00, 0x02, 0x00, 0x0b];

    _ = DiskJourneyRepo::deserialize(&input);
}

#[test]
fn serialize_with_skipped() {
    let actual = DiskJourneyRepo::serialize(17, vec![11, 515]);

    let expected: Vec<u8> = vec![0x00, 0x11, 0x00, 0x02, 0x00, 0x0b, 0x02, 0x03];

    assert_eq!(actual, expected);
}

#[test]
fn serialize_without_skipped() {
    let actual = DiskJourneyRepo::serialize(17, vec![]);

    let expected: Vec<u8> = vec![0x00, 0x11, 0x00, 0x00];

    assert_eq!(actual, expected);
}
