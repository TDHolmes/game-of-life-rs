/// App Module
///
/// This module just contains the logic for running the main.rs application.
///
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write};

use termion;

use crate::{board, config};

pub fn app(rows: usize, cols: usize, prob_density: Option<f32>, init_filepath: Option<&Path>, update_rate: Duration) {
    let mut board = board::Board::new(rows, cols);

    // initialize with a file (pull out rows/cols first)
    if let Some(p) = init_filepath {
        let c = config::Configuration::load_config(p).unwrap();
        c.apply_config(&mut board).unwrap();
    } else if let Some(density) = prob_density {
        // initialize randomly
        board.initialize_random(density);
    }

    // continually update screen
    let screen = io::stdout();
    loop {
        {
            let mut handle = screen.lock();

            // animate on the main screen
            handle.write_all(format!("{}{}{}",
                // Clear the screen.
                termion::clear::All,
                // Goto (1,1).
                termion::cursor::Goto(1, 1),
                board,
            ).as_bytes()).unwrap();
        }

        if board.get_num_alive_cells() == 0 {
            break;
        }

        sleep(update_rate);
        board.update();
    }
}