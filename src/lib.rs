/// Game of life crate that implements the game as well as the ability to load
/// Game of Life configuration files from the community.
///
/// Rules:
///   - Any live cell with fewer than two live neighbours dies, as if by underpopulation.
///   - Any live cell with two or three live neighbours lives on to the next generation.
///   - Any live cell with more than three live neighbours dies, as if by overpopulation.
///   - Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
///
pub mod board;
pub mod config;
pub mod app;  // Only for the main.rs application...

pub(crate) mod cell;