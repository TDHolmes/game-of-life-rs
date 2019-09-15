/// Loads configurations of Conway's Game of Life boards.
///
/// Allows both loading of my own json format or the more common
/// formats in the GoL community.
///
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::result;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use regex::Regex;

use crate::board::Board;

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub rows: usize,
    pub cols: usize,
    board: Vec<Vec<u8>>,
}

impl Configuration {
    pub fn load_json_config(filepath: &Path) -> Result<Configuration> {
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
        if let Err(why) = file.read_to_string(&mut s) {
            panic!("couldn't read {}: {}", display, why.description());
        }

        let c: Configuration = serde_json::from_str(&s)?;
        Ok(c)
    }

    pub fn load_rle_config(filepath: &Path) -> result::Result<Configuration, &'static str> {
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
        if let Err(why) = file.read_to_string(&mut s) {
            panic!("couldn't read {}: {}", display, why.description());
        }

        let vec = parse_rle_string(&s)?;

        Ok(Configuration {
            rows: vec.len(),
            cols: vec[0].len(),
            board: vec,
        })
    }

    pub fn apply_config(&self, board: &mut Board) -> result::Result<(), &'static str> {
        // first, make sure that the config given can fit within the given board
        if self.board.len() > board.rows {
            return Err("This configuration requires a larger board (more rows)!")
        }
        for row in &self.board {
            if row.len() > board.cols {
                eprintln!("conf cols: {}, board cols: {}", row.len(), board.cols);
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


/// Parse Run Length Encoded (RLE) config strings. Returns a parsed 2d vector of the board
/// described by the configuration given, if valid.
///
/// For more info on the encoding, see [this link](http://www.conwaylife.com/wiki/Run_Length_Encoded)
fn parse_rle_string(rle_str: &str) -> result::Result<Vec<Vec<u8>>, &'static str> {
    static CONWAY_LIFE_TYPE: &str = "b3/s23";
    static _DEAD_CELL: &str = "b";
    static ALIVE_CELL: &str = "o";
    static EOL: &str = "$";
    static EOB: &str = "!";

    // board and dimensions
    let mut board: Vec<Vec<u8>> = Vec::new();
    let mut x: usize = 0;
    let mut y: usize = 0;

    // tracking vars for filling in the board as we go
    let mut sub_x: usize = 0;
    let mut sub_y: usize = 0;

    let re_dimensions = Regex::new(r"\s*x\s*=\s*(\d+),\s*y\s*=\s*(\d+)").unwrap();
    let re_life_type = Regex::new(r".*[type|rule]\s*=\s*([\w/]+)").unwrap();
    let re_board_desc = Regex::new(r"(\d*[bo$]|[!])").unwrap();
    let re_numbers = Regex::new(r"(\d+)").unwrap();

    for original_line in rle_str.lines() {
        let line = original_line.to_ascii_lowercase();

        // skip comments and other config
        if line.starts_with('#') && line[1..].starts_with('c') {
            continue;
        }
        // TODO: handle other configuration elements
        //        N: name
        //        O: who/when made
        //      R/P: coordinates of where the top left corner of the pattern goes
        //        r: the Game Of Life rules. Usually not used as it goes on the dimensions line

        // Handle dimensions and board type
        let mut matched_dim_or_type = false;
        if re_dimensions.is_match(&line) {
            matched_dim_or_type = true;
            if let Some(captures) = re_dimensions.captures(&line) {
                x = captures[1].parse().unwrap();
                y = captures[2].parse().unwrap();

                // initialize board with all dead cells
                for _ in 0..y {
                    board.push(vec![0; x]);
                }
            }
        }
        if re_life_type.is_match(&line) {
            matched_dim_or_type = true;
            if let Some(captures) = re_life_type.captures(&line) {
                if captures[1].to_ascii_lowercase() != *CONWAY_LIFE_TYPE {
                    return Err("Specified life type not Conway! Cannot play config.");
                }
            }
        }

        if matched_dim_or_type {
            continue;
        }

        // lines for describing the board
        if re_board_desc.is_match(&line) {
            if x == 0 || y == 0 {
                return Err("Invalid board dimensions!");
            }

            for cap in re_board_desc.captures_iter(&line) {

                // first, check for termination
                if cap[0] == *EOB {
                    sub_x = 0;
                    sub_y = 0;
                    break;
                }

                // Some sort of cell specifier
                let mut num_vals: usize = 1;
                if let Some(number_match) = re_numbers.captures(&cap[0]) {
                    if let Ok(parsed_val) = number_match[1].parse() {
                        num_vals = parsed_val;
                    }
                }

                if cap[0].find(ALIVE_CELL).is_some() {
                    for _ in 0..num_vals {
                        board[sub_y][sub_x] = 1;
                        sub_x += 1;
                    }
                } else if cap[0].find(EOL).is_some() {
                    sub_y += num_vals;
                    sub_x = 0;
                } else {
                    sub_x += num_vals;
                }
            }
        }
    }

    Ok(board)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rle_load_glider_no_type() {
        let glider_rle = "#C This is a glider.
x = 3, y = 3
bo$2bo$3o!";

        let expected_vec: Vec<Vec<u8>> = vec![
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![1, 1, 1],
        ];

        match parse_rle_string(&glider_rle) {
            Ok(board) => assert!(
                expected_vec == board,
                "Board did not match! expected: {:?}, got: {:?}",
                expected_vec,
                board
            ),
            Err(error) => assert!(false, error),
        }
    }

    #[test]
    fn rle_load_glider_type() {
        let glider_rle = "#C This is a glider.
x = 3, y = 3, type = b3/s23
bo$2bo$3o!";

        let expected_vec: Vec<Vec<u8>> = vec![
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![1, 1, 1],
        ];

        match parse_rle_string(&glider_rle) {
            Ok(board) => assert!(
                expected_vec == board,
                "Board did not match! expected: {:?}, got: {:?}",
                expected_vec,
                board
            ),
            Err(error) => assert!(false, error),
        }
    }

    #[test]
    fn rle_load_invalid_size() {
        let glider_rle = "#C This is a glider.
x = x, y = 3
bo$2bo$3o!";

        match parse_rle_string(&glider_rle) {
            Ok(_) => assert!(false, "Board should not have initialized properly!"),
            Err(err) => eprintln!("Errored out properly: {}", err),
        }
    }

    #[test]
    fn rle_load_invalid_type() {
        let glider_rle = "#C This is a glider.
x = 3, y = 3, type = B36/S23
bo$2bo$3o!";

        match parse_rle_string(&glider_rle) {
            Ok(_) => assert!(false, "Board should not have initialized properly!"),
            Err(err) => eprintln!("Errored out properly: {}", err),
        }
    }

    #[test]
    fn rle_load_valid_type() {
        let glider_rle = "#C This is a glider.
x = 3, y = 3, type = B3/S23
bo$2bo$3o!";

        match parse_rle_string(&glider_rle) {
            Ok(_) => println!("Parsed properly!"),
            Err(err) => assert!(false, "Errored out incorrectly: {}", err),
        }
    }
}