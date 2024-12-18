use crate::toolbox::{Direction, Grid, Position};

const BOX: char = 'O';
const ROBOT: char = '@';
const WALL: char = '#';
const FREE_SPACE: char = '.';

pub struct Warehouse {
    pub grid: Grid,
    pub moves: Vec<Direction>,
}

impl Warehouse {
    pub fn from(input: &str) -> Warehouse {
        let (grid, moves) = input.split_once("\n\n").unwrap();
        Warehouse { grid: Grid::from(grid), moves: moves.lines().flat_map(|line| line.chars()).map(Direction::from).collect::<Vec<_>>() }
    }

    pub fn move_robot(&mut self) {
        let mut robot_position = self.grid.position_of(ROBOT).unwrap();
        for direction in self.moves.clone() {
            robot_position = self.move_object(&robot_position, &direction);
        }
    }

    pub fn move_object(&mut self, position: &Position, direction: &Direction) -> Position {
        let next_position = &position.move_towards(direction);
        let object_at_next_position = *self.grid.element_at(next_position);

        match object_at_next_position {
            WALL => *position,
            FREE_SPACE => {
                self.grid.swap_elements(position, next_position);
                *next_position
            }
            BOX => {
                let changed_position = self.move_object(next_position, direction);
                if changed_position != *next_position {
                    self.grid.swap_elements(position, next_position);
                    *next_position
                } else {
                    *position
                }
            }
            _ => panic!("Invalid object: {}", object_at_next_position),
        }
    }

    pub fn boxes_sum(&self) -> isize {
        self.grid.positions_of(BOX).map(|p| p.x + p.y * 100).sum()
    }
}
