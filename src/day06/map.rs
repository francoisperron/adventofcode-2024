use crate::toolbox::Direction;
use crate::toolbox::Grid;
use crate::toolbox::Position;
use std::collections::HashSet;

pub struct Map {
    grid: Grid,
    guard: Guard,
}

impl Map {
    pub fn from(input: &str) -> Map {
        let grid = Grid::from(input);
        let guard = Guard::new(grid.position_of('^').unwrap());
        Map { grid, guard }
    }

    pub fn predict(&mut self) {
        while self.grid.is_inbound(&self.guard.position) {
            self.guard.patrol(&self.grid);
        }
    }

    pub fn patrolled_areas_count(&self) -> usize {
        self.guard.patrolled.len()
    }

    pub fn obstructions_count(&mut self) -> usize {
        let starting_position = self.guard.position;

        self.predict();
        let candidates = self.guard.patrolled.clone();

        candidates
            .iter()
            .filter(|&p| {
                self.grid.set_element_at(p, '#');

                let mut seen = vec![vec![[false; 4]; self.grid.max_y as usize]; self.grid.max_x as usize];
                self.guard = Guard::new(starting_position);
                while self.grid.is_inbound(&self.guard.position) && !seen[self.guard.position.x as usize][self.guard.position.y as usize][self.guard.direction as usize] {
                    seen[self.guard.position.x as usize][self.guard.position.y as usize][self.guard.direction as usize] = true;
                    self.guard.patrol(&self.grid);
                }
                let is_a_cycle = self.grid.is_inbound(&self.guard.position);

                self.grid.set_element_at(p, '.');

                is_a_cycle
            })
            .count()
    }
}

pub struct Guard {
    pub position: Position,
    pub direction: Direction,
    pub patrolled: HashSet<Position>,
}

impl Guard {
    pub fn new(position: Position) -> Guard {
        Guard { position, direction: Direction::Up, patrolled: HashSet::from([position]) }
    }

    pub fn patrol(&mut self, grid: &Grid) {
        let next_position = self.position.move_towards(&self.direction);

        if !grid.is_inbound(&next_position) {
            self.position = next_position;
            return;
        }

        if grid.element_at(&next_position) == &'#' {
            self.direction = self.direction.turn_right();
        } else {
            self.position = next_position;
            self.patrolled.insert(self.position);
        }
    }
}
