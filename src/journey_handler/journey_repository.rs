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

use mockall::automock;

#[automock]
/// Defines the behavior to persist a journey state.
pub trait PersistJourney {
    /// Reads a journey from the underlying medium.
    fn read(&self) -> (u16, Vec<u16>);

    /// Writes a journey to the underlying medium.
    fn write(&self, next: u16, skipped: Vec<u16>) -> ();
}

#[derive(Debug)]
/// Productive implementation.
pub struct DiskJourneyRepo {}

impl PersistJourney for DiskJourneyRepo {
    fn read(&self) -> (u16, Vec<u16>) {
        let data_path = DiskJourneyRepo::get_data_path();

        match std::fs::exists(&data_path) {
            Ok(result) => {
                if result {
                    let bytes = DiskJourneyRepo::read_from_file(data_path.as_path());
                    return DiskJourneyRepo::deserialize(&bytes);
                } else {
                    (1, vec![])
                }
            }
            Err(err) => panic!("std::fs error '{}'", err.kind().to_string()),
        }
    }

    fn write(&self, next: u16, skipped: Vec<u16>) -> () {
        let data_path = DiskJourneyRepo::get_data_path();

        let mut data_dir = data_path.clone();
        data_dir.pop();

        match std::fs::exists(&data_dir) {
            Ok(result) => {
                if !result {
                    match std::fs::create_dir(&data_dir) {
                        Ok(()) => {}
                        Err(err) => panic!(
                            "std::fs error '{}' regarding {}",
                            err.kind().to_string(),
                            data_dir.to_string_lossy()
                        ),
                    }
                }
            }
            Err(err) => panic!(
                "std::fs error '{}' regarding {}",
                err.kind().to_string(),
                data_dir.to_string_lossy()
            ),
        }

        let bytes = DiskJourneyRepo::serialize(next, skipped);
        DiskJourneyRepo::write_to_file(&data_path, &bytes);
    }
}

impl DiskJourneyRepo {
    /// Gets the relevant full path to the file containing the journey data.
    fn get_data_path() -> std::path::PathBuf {
        let mut data_path = dirs::data_dir()
            .unwrap_or_else(|| panic!("unable to find data directory for journey data"));

        data_path.push("rslibrecell");
        data_path.push("journey.bin");
        return data_path;
    }

    /// Helper function to read.
    fn read_from_file(path: &std::path::Path) -> Vec<u8> {
        match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(err) => panic!("on read, std::fs error '{}'", err.kind().to_string()),
        }
    }

    /// Helper function to write.
    fn write_to_file(path: &std::path::Path, bytes: &Vec<u8>) -> () {
        match std::fs::write(path, bytes) {
            Ok(bytes) => bytes,
            Err(err) => panic!("on write, std::fs error '{}'", err.kind().to_string()),
        }
    }

    /// Deserializes an appropriate number of bytes into journey data.
    pub(crate) fn deserialize(bytes: &Vec<u8>) -> (u16, Vec<u16>) {
        if bytes.len() < 4 {
            panic!("too little data (<4)")
        }

        let next_game = (bytes[0] as u16) * 256 + (bytes[1] as u16);

        let all_skipped_count = (bytes[2] as u16) * 256 + (bytes[3] as u16);

        if bytes.len() < (4 + 2 * all_skipped_count) as usize {
            panic!("too little data (mismatch in skipped)")
        }

        let mut all_skipped: Vec<u16> = vec![];

        for i in 0..(all_skipped_count as usize) {
            let skipped_game = (bytes[4 + 2 * i] as u16) * 256 + (bytes[5 + 2 * i] as u16);
            all_skipped.push(skipped_game);
        }

        return (next_game, all_skipped);
    }

    /// Serializes journey data into bytes.
    pub(crate) fn serialize(next: u16, skipped: Vec<u16>) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        let mut push = |i: u16| {
            let first = (i / 256) as u8;
            let second = (i % 256) as u8;
            result.push(first);
            result.push(second);
        };

        push(next);
        push(skipped.len() as u16);

        for skipped in skipped {
            push(skipped);
        }

        return result;
    }
}

#[cfg(test)]
mod test;
