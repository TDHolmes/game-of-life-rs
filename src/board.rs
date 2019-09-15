/// The board on which Game of Life is played
///
use std::fmt::{Display, Formatter, Error};
use termion;

use crate::cell::Cell;

#[derive(Debug)]
pub struct Board {
    pub rows: usize,
    pub cols: usize,

    /// The grid 2d vector is set up like this:
    ///
    /// ```text
    /// grid = [
    ///     [ v(0,0), v(1,0), (...), v(x,0) ],
    ///     [ v(0,1), v(1,1), (...), v(x,1) ],
    ///     (...),
    ///     [ v(0,y), v(1,y), (...), v(x,y) ],
    /// ]
    /// ```
    pub(crate) grid: Vec<Vec<Cell>>,
}

impl Board {
    /// Initializes a new board of the given dimensions
    ///
    /// # Example
    /// ```
    /// # use gameoflife::board::Board;
    /// let mut b: Board = Board::new(4, 2);
    /// ```
    pub fn new(rows: usize, cols: usize) -> Board {
        let mut b = Board {
            grid: Vec::new(),
            rows,
            cols,
        };

        for r in 0..b.rows {
            for _ in 0..b.cols {
            b.grid.push(Vec::new());
                b.grid[r].push(Cell::new());
            }
        }
        b
    }

    /// Configures the cells in the board to alive or dead with the given probability
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

    /// Sets all cells in the board to dead
    pub fn clear(&mut self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                self.grid[r][c].is_alive = false;
            }
        }
    }

    /// Updates all cells to their next state based on their neighbors
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

    /// returns the number of alive cells on the board.
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

    /// Iterate over all of the cells on the board
    ///
    /// This allows you to know the (x, y) location of all cells and
    /// if they are alive or not.
    pub fn iter_cells(&self) -> impl Iterator<Item = ((usize, usize), &bool)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, column)| ((x, y), &column.is_alive)))
    }
}

impl Display for Board {

    /// Displays the Game of Life board on a termial.
    ///
    /// As an optimization, we don't draw dead cells but skip to alive
    /// cells and the boarder.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        // Clear the screen and reset cursor
        write!(
            f,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
        ).unwrap();

        // write top row of the border
        write!(f, "┌").unwrap();
        for _ in 0..self.cols {
            write!(f, "─").unwrap();
        }
        writeln!(f, "┐").unwrap();

        // write interior borders and cells
        let mut x;
        let mut y = 2;
        for r in 0..self.rows {
            write!(f, "│").unwrap();
            x = 2;
            for c in 0..self.cols {
                if self.grid[r][c].is_alive {
                    write!(
                        f,
                        "{}{}",
                        termion::cursor::Goto(x, y),
                        self.grid[r][c]
                    ).unwrap();
                }
                x += 1;
            }
            writeln!(f, "{}│", termion::cursor::Goto(x, y)).unwrap();
            y += 1;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iter() {
        // configure the board with some known state
        let mut b = Board::new(5, 5);
        for (y, row) in b.grid.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if x == y {
                    cell.is_alive = true;
                } else {
                    cell.is_alive = false;
                }
            }
        }

        // check to make sure that state matches in the iterator
        for ((x, y), alive) in b.iter_cells() {
            if x == y {
                assert_eq!(*alive, true);
            } else {
                assert_eq!(*alive, false);
            }
        }
    }
}