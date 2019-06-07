
use std::fmt::{Display, Formatter, Error};

use crate::cell::Cell;

#[derive(Debug)]
pub struct Board {
    pub rows: usize,
    pub cols: usize,
    pub(crate) grid: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Board {
        let mut b = Board {
            grid: Vec::new(),
            rows: rows,
            cols: cols,
        };

        for r in 0..b.rows {
            for _ in 0..b.cols {
            b.grid.push(Vec::new());
                b.grid[r].push(Cell::new());
            }
        }
        b
    }

    pub fn initialize_random(&mut self, prob_density: f32) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                let v = rand::random::<f32>();
                if v >= (1. - prob_density) {
                    self.grid[r][c].is_alive = true;
                } else {
                    self.grid[r][c].is_alive = false;
                }
            }
        }
    }

    pub fn clear(&mut self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                self.grid[r][c].is_alive = false;
            }
        }
    }

    pub fn update(&mut self) {
        for r in 0..self.rows {
            for c in 0..self.cols {

                let mut alive_neighbors = 0;
                for y in r..=(r+2) {
                    for x in c..=(c+2) {
                        let x: isize = (x as isize) - 1;
                        let y: isize = (y as isize) - 1;
                        if x < 0 || x >= (self.cols as isize) {
                            continue;
                        }
                        if y < 0 || y >= (self.rows as isize) {
                            continue;
                        }
                        if x == (c as isize) && y == (r as isize) {
                            continue;
                        }

                        if self.grid[y as usize][x as usize].is_alive {
                            alive_neighbors += 1;
                        }
                    }
                }
                self.grid[r][c].update(alive_neighbors);
            }
        }

        for c in 0..self.cols {
            for r in 0..self.rows {
                self.grid[r][c].latch_state();
            }
        }
    }

    pub fn get_num_alive_cells(&self) -> usize {
        let mut cnt = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.grid[r][c].is_alive {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        // write top row of the border
        write!(f, "┌").unwrap();
        for _ in 0..self.cols {
            write!(f, "─").unwrap();
        }
        writeln!(f, "┐").unwrap();

        // write interior borders and cells
        for r in 0..self.rows {
            write!(f, "│").unwrap();
            for c in 0..self.cols {
                write!(f, "{}", self.grid[r][c]).unwrap();
            }
            writeln!(f, "│").unwrap();
        }

        // write bottom row of the border
        write!(f, "└").unwrap();
        for _ in 0..self.cols {
            write!(f, "─").unwrap();
        }
        writeln!(f, "┘").unwrap();

        Ok(())
    }
}