use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::result;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::board::Board;

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    board: Vec<Vec<u8>>,
}

impl Configuration {
    pub fn load_config(filepath: &Path) -> Result<Configuration> {
        let display = filepath.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&filepath) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why.description()),
            Ok(_) => (),
        }

        let c: Configuration = serde_json::from_str(&s)?;
        Ok(c)
    }

    pub fn apply_config(&self, board: &mut Board) -> result::Result<(), &'static str> {
        // first, make sure that the config given can fit within the given board
        if self.board.len() > board.rows {
            return Err("This configuration requires a larger board (more rows)!")
        }
        for row in &self.board {
            if row.len() > board.cols {
                return Err("This configuration requires a larger board (more cols)!")
            }
        }

        // apply the configuration!
        board.clear();
        for (x, row) in self.board.iter().enumerate() {
            for (y, val) in row.iter().enumerate() {
                if *val != 0 {
                    board.grid[x][y].is_alive = true;
                }
            }
        }

        Ok(())
    }
}
