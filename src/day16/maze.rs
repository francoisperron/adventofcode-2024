use crate::toolbox::{Direction, Grid};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct Maze {
    pub grid: Grid,
}

impl Maze {
    pub fn from(input: &str) -> Maze {
        Maze { grid: Grid::from(input) }
    }

    pub fn shortest_path_score(&self) -> usize {
        let start = self.grid.position_of('S').unwrap();
        let end = self.grid.position_of('E').unwrap();

        let mut to_visit = BinaryHeap::new();
        to_visit.push(Reverse((0, start, Direction::Left)));

        let mut costs = HashMap::new();
        costs.insert((start, Direction::Left), 0);

        while let Some(Reverse((cost, position, direction))) = to_visit.pop() {
            if position == end {
                return cost;
            }

            for new_dir in Direction::all() {
                let turn_cost = if new_dir == direction { 0 } else { 1000 };

                let new_pos = position.move_towards(&new_dir);
                if !self.grid.is_inbound(&new_pos) || self.grid.element_at(&new_pos) == &'#' {
                    continue;
                }

                let new_cost = cost + 1 + turn_cost;

                if new_cost < *costs.get(&(new_pos, new_dir)).unwrap_or(&usize::MAX) {
                    costs.insert((new_pos, new_dir), new_cost);
                    to_visit.push(Reverse((new_cost, new_pos, new_dir)));
                }
            }
        }

        panic!("No path to end");
    }
}
