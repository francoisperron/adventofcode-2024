use crate::toolbox::{Direction, Grid, Position};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct Maze {
    pub grid: Grid,
    pub start: Position,
    pub end: Position,
}

impl Maze {
    pub fn from(input: &str) -> Maze {
        let grid = Grid::from(input);
        let start = grid.position_of('S').unwrap();
        let end = grid.position_of('E').unwrap();
        Maze { grid, start, end }
    }

    pub fn shortest_path_score(&self) -> usize {
        let scores = self.shortest_paths();

        *scores.get(&(self.end, Direction::Up)).unwrap()
    }

    pub fn places_to_sit(&mut self) -> usize {
        let scores = self.shortest_paths();
        let places_to_sit = self.positions_visited_in_shortest_paths(scores);

        places_to_sit.len()
    }

    fn shortest_paths(&self) -> HashMap<(Position, Direction), usize> {
        let mut to_visit = BinaryHeap::new();
        to_visit.push(Reverse((0, self.start, Direction::Left)));

        let mut costs = HashMap::new();
        costs.insert((self.start, Direction::Left), 0);

        let mut min_cost = usize::MAX;

        while let Some(Reverse((cost, position, direction))) = to_visit.pop() {
            if position == self.end && cost < min_cost {
                min_cost = cost;
                continue;
            }

            for new_dir in Direction::all() {
                let new_pos = position.move_towards(&new_dir);
                if !self.grid.is_inbound(&new_pos) || self.grid.element_at(&new_pos) == &'#' {
                    continue;
                }
                
                let turn_cost = if new_dir == direction { 0 } else { 1000 };
                let new_cost = cost + 1 + turn_cost;

                if new_cost < *costs.get(&(new_pos, new_dir)).unwrap_or(&usize::MAX) {
                    costs.insert((new_pos, new_dir), new_cost);
                    to_visit.push(Reverse((new_cost, new_pos, new_dir)));
                }
            }
        }

        costs
    }

    fn positions_visited_in_shortest_paths(&mut self, costs: HashMap<(Position, Direction), usize>) -> HashSet<Position> {
        let min_cost = *costs.get(&(self.end, Direction::Up)).unwrap();

        let mut positions = HashSet::from([self.end]);
        let mut queue = Vec::from([(min_cost, self.end)]);

        while let Some((cost, position)) = queue.pop() {
            for new_dir in Direction::all() {
                let next_cost = *costs.get(&(position, new_dir)).unwrap_or(&usize::MAX);
                if next_cost <= cost {
                    let next_pos = position.move_towards(&new_dir.reverse());
                    if positions.insert(next_pos) && next_pos != self.start {
                        queue.push((next_cost, next_pos));
                    }
                }
            }
        }
        positions
    }
}
