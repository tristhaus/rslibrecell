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
fn prng_generate_state_0_works() {
    let mut prng = Prng { state: 0 };

    assert_eq!(38, prng.get_next());
    assert_eq!(7719, prng.get_next());
    assert_eq!(21238, prng.get_next());
    assert_eq!(2437, prng.get_next());
    assert_eq!(8855, prng.get_next());
    assert_eq!(11797, prng.get_next());
    assert_eq!(8365, prng.get_next());
    assert_eq!(32285, prng.get_next());
    assert_eq!(10450, prng.get_next());
    assert_eq!(30612, prng.get_next());
}

#[test]
fn prng_generate_state_1_works() {
    let mut prng = Prng { state: 1 };

    assert_eq!(41, prng.get_next());
    assert_eq!(18467, prng.get_next());
}
