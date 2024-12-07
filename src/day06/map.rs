use std::collections::HashSet;
use crate::toolbox::Grid;
use crate::toolbox::Position;
use crate::toolbox::Direction;

pub struct Map {
    grid: Grid,
    guard: Guard
}

impl Map {
    pub fn from(input: &str) -> Map {
        let grid = Grid::from(input);
        let position = grid.position_of(&'^').unwrap();
        let guard = Guard { position, direction: Direction::Up, patrolled: HashSet::from([position]) };
        Map { grid, guard }
    }
    
    pub fn predict(&mut self) {
        while self.grid.is_inbound(&self.guard.position) {
            self.guard.patrol(&self.grid);    
        }        
    }
    
    pub fn patrolled_area_count(&self) -> usize {
        self.guard.patrolled.len() - 1
    }
}

pub struct Guard {
    pub position: Position,
    pub direction: Direction,
    pub patrolled: HashSet<Position>,
}

impl Guard {
    pub fn patrol(&mut self, grid: &Grid) {
        let next_position = self.position.move_towards(&self.direction);
        
        if grid.element_at(&next_position) == &'#' {
            self.direction = self.direction.turn_right();
        }
        else { 
            self.position = next_position;
            self.patrolled.insert(self.position);
        }
    }
}