use std::io::Read;

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
use serde::{Deserialize, Serialize};

/// Defines the key config to play the actual game.
#[derive(Serialize, Deserialize, Debug)]
pub struct KeyConfig {
    /// The key referring to the first cell.
    pub cell1: char,
    /// The key referring to the second cell.
    pub cell2: char,
    /// The key referring to the third cell.
    pub cell3: char,
    /// The key referring to the fourth cell.
    pub cell4: char,
    /// The first key referring to the foundations.
    pub foundation1: char,
    /// The second key referring to the foundations.
    pub foundation2: char,
    /// The third key referring to the foundations.
    pub foundation3: char,
    /// The fourth key referring to the foundations.
    pub foundation4: char,
    /// The key referring to the first column.
    pub column1: char,
    /// The key referring to the second column.
    pub column2: char,
    /// The key referring to the third column.
    pub column3: char,
    /// The key referring to the fourth column.
    pub column4: char,
    /// The key referring to the fifth column.
    pub column5: char,
    /// The key referring to the sixth column.
    pub column6: char,
    /// The key referring to the seventh column.
    pub column7: char,
    /// The key referring to the eighth column.
    pub column8: char,
}

/// Gets the config, either from disk or the default.
pub fn get_config() -> KeyConfig {
    let path = get_key_config_path();

    if let Ok(true) = std::fs::exists(&path) {
        let mut file =
            std::fs::File::open(&path).expect("unable to open key config from 'key_config.json'");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("unable to read key config string data from 'key_config.json'");

        let deserialized =
            serde_json::from_str(&data).expect("unable to parse key config from 'key_config.json'");

        deserialized
    } else {
        KeyConfig {
            cell1: 'q',
            cell2: 'w',
            cell3: 'e',
            cell4: 'r',
            foundation1: 'u',
            foundation2: 'i',
            foundation3: 'o',
            foundation4: 'p',
            column1: 'a',
            column2: 's',
            column3: 'd',
            column4: 'f',
            column5: 'j',
            column6: 'k',
            column7: 'l',
            column8: ';',
        }
    }
}

fn get_key_config_path() -> std::path::PathBuf {
    let mut data_path =
        dirs::data_dir().expect("unable to find data directory for key config data");

    data_path.push("rslibrecell");
    data_path.push("key_config.json");
    return data_path;
}
