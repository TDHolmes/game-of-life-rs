/// Implementation of a single Conway's Game of Life cell
///
use std::fmt::{Display, Formatter, Error};

#[derive(Copy, Clone, Debug)]
pub(crate) struct Cell {
    pub is_alive: bool,
    pending_state: bool,
}

impl Cell {
    /// returns a new, dead cell
    pub(crate) fn new() -> Cell {
        Cell {is_alive: false, pending_state: false}
    }

    /// given the number of alive neighbors, update our pending state
    pub(crate) fn update(&mut self, alive_neighbors: u32) {
        if self.is_alive {
            if alive_neighbors <= 1 || alive_neighbors > 3 {
                self.pending_state = false;
            } else {
                self.pending_state = true;
            }
        } else if alive_neighbors == 3 {
            self.pending_state = true;   // nature, uh, finds a way
        }
    }

    /// latches the pending internal state to alive or dead
    pub(crate) fn latch_state(&mut self) {
        self.is_alive = self.pending_state;
        self.pending_state = false;
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if self.is_alive {
            write!(f, "●")
        } else {
            write!(f, " ")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_new_alive_cell() -> Cell {
        let mut c = Cell::new();
        c.is_alive = true;
        c
    }

    fn get_new_dead_cell() -> Cell {
        let mut c = Cell::new();
        c.is_alive = false;
        c
    }

    #[test]
    fn new_cell() {
        let c = Cell::new();
        assert!(c.is_alive == false);
        assert!(c.pending_state == false);
    }

    #[test]
    fn cell_dead_to_alive_direct() {
        let mut c = get_new_dead_cell();

        c.pending_state = true;
        c.latch_state();

        assert!(c.is_alive == true);
    }

    #[test]
    fn cell_alive_to_dead_direct() {
        let mut c = get_new_alive_cell();

        c.pending_state = false;
        c.latch_state();

        assert!(c.is_alive == false);
    }

    #[test]
    fn cell_too_many_neighbors() {
        let mut c = get_new_alive_cell(); c.update(4); assert!(c.pending_state == false);
        let mut c = get_new_alive_cell(); c.update(5); assert!(c.pending_state == false);
        let mut c = get_new_alive_cell(); c.update(6); assert!(c.pending_state == false);
        let mut c = get_new_alive_cell(); c.update(7); assert!(c.pending_state == false);
        let mut c = get_new_alive_cell(); c.update(8); assert!(c.pending_state == false);
    }

    #[test]
    fn cell_too_few_neighbors() {
        let mut c = get_new_alive_cell(); c.update(1); assert!(c.pending_state == false);
        let mut c = get_new_alive_cell(); c.update(0); assert!(c.pending_state == false);
    }

    #[test]
    fn cell_just_enough_neighbors() {
        let mut c = get_new_alive_cell(); c.update(2); assert!(c.pending_state == true);
        let mut c = get_new_alive_cell(); c.update(3); assert!(c.pending_state == true);
    }

    #[test]
    fn cell_reproductive_neighbors() {
        // no reproduction cases
        let mut c = get_new_dead_cell(); c.update(0); assert!(c.pending_state == false);
        let mut c = get_new_dead_cell(); c.update(1); assert!(c.pending_state == false);
        let mut c = get_new_dead_cell(); c.update(2); assert!(c.pending_state == false);
        let mut c = get_new_dead_cell(); c.update(4); assert!(c.pending_state == false);
        let mut c = get_new_dead_cell(); c.update(5); assert!(c.pending_state == false);
        let mut c = get_new_dead_cell(); c.update(6); assert!(c.pending_state == false);
        let mut c = get_new_dead_cell(); c.update(7); assert!(c.pending_state == false);
        let mut c = get_new_dead_cell(); c.update(8); assert!(c.pending_state == false);

        // only case where there should be reproduction
        let mut c = get_new_dead_cell(); c.update(3); assert!(c.pending_state == true);

    }
}