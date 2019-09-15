/// App Module
///
/// This module just contains the logic for running the main.rs application.
///
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write};

use crate::{board, config};

pub fn app(rows: usize, cols: usize, prob_density: Option<f32>, init_filepath: Option<&Path>, update_rate: Duration) {
    let mut rows = rows;
    let mut cols = cols;

    // initialize with a file (pull out rows/cols first)
    let mut conf: Option<config::Configuration> = None;
    if let Some(p) = init_filepath {
        if let Some(ext) = p.extension() {
            if ext == "json" {
                conf = Some(config::Configuration::load_json_config(p).unwrap());
            } else {
                conf = Some(config::Configuration::load_rle_config(p).unwrap());
            }
        }
    }

    let mut board: board::Board;

    if let Some(c) = conf {
        if c.cols > cols {
            cols = c.cols;
        }
        if c.rows > rows {
            rows = c.rows;
        }
        println!("Board size: rows: {}, cols: {}", rows, cols);
        board = board::Board::new(rows, cols);
        c.apply_config(&mut board).unwrap();
    } else if let Some(density) = prob_density {
        // initialize randomly
        board = board::Board::new(rows, cols);
        board.initialize_random(density);
    } else {
        panic!("Invalid arguments! need either random probability density or configuration file.");
    }

    // continually update screen
    let screen = io::stdout();
    loop {
        {
            let mut handle = screen.lock();

            // animate on the main screen
            handle.write_all(format!("{}", board).as_bytes()).unwrap();
        }

        if board.get_num_alive_cells() == 0 {
            break;
        }

        sleep(update_rate);
        board.update();
    }
}