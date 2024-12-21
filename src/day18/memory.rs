use crate::toolbox::{Direction, Grid, Position};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct Memory {
    pub grid: Grid,
    start: Position,
    end: Position,
    pub bytes: Vec<Position>,
}

impl Memory {
    pub fn from(x: isize, y: isize, input: &str) -> Memory {
        let bytes = input.lines().map(Position::from).collect();
        Memory { grid: Grid::new(x, y), start: Position::new(0, 0), end: Position::new(x - 1, y - 1), bytes }
    }

    pub fn corrupt(&mut self, byte_nb: usize) {
        self.grid.set_element_at(&self.bytes[byte_nb], '#');
    }

    pub fn last_position(&mut self) -> Position {
        for byte in 0..self.bytes.len() {
            self.corrupt(byte);
            if self.reach_exit() == usize::MAX {
                return self.bytes[byte];
            }
        }
        unreachable!();
    }

    pub fn reach_exit(&self) -> usize {
        let mut to_visit = BinaryHeap::new();
        to_visit.push(Reverse((0, self.start)));

        let mut costs = HashMap::new();
        costs.insert(self.start, 0);

        while let Some(Reverse((cost, position))) = to_visit.pop() {
            if position == self.end {
                return cost;
            }

            for new_dir in Direction::all() {
                let new_pos = position.move_towards(&new_dir);
                if !self.grid.is_inbound(&new_pos) || self.grid.element_at(&new_pos) == &'#' {
                    continue;
                }

                let new_cost = cost + 1;

                if new_cost < *costs.get(&new_pos).unwrap_or(&usize::MAX) {
                    costs.insert(new_pos, new_cost);
                    to_visit.push(Reverse((new_cost, new_pos)));
                }
            }
        }

        usize::MAX
    }
}
