use crate::toolbox::{Direction, Grid, Position};

const BOX: char = 'O';
const BIG_BOX_START: char = '[';
const BIG_BOX_END: char = ']';
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
        Warehouse { grid: Grid::from(grid), moves: Self::extract_moves(moves) }
    }

    pub fn wider(input: &str) -> Warehouse {
        let (grid, moves) = input.split_once("\n\n").unwrap();
        let wider_grid = grid
            .lines()
            .map(|line| line.chars().flat_map(Self::wider_object).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        Warehouse { grid: Grid::from(&wider_grid), moves: Self::extract_moves(moves) }
    }

    fn extract_moves(moves: &str) -> Vec<Direction> {
        moves.lines().flat_map(|line| line.chars()).map(Direction::from).collect::<Vec<_>>()
    }

    fn wider_object(c: char) -> [char; 2] {
        match c {
            BOX => [BIG_BOX_START, BIG_BOX_END],
            ROBOT => [ROBOT, FREE_SPACE],
            WALL => [WALL, WALL],
            FREE_SPACE => [FREE_SPACE, FREE_SPACE],
            _ => unreachable!(),
        }
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
        let boxes: isize = self.grid.positions_of(BOX).map(|p| p.x + p.y * 100).sum();
        let big_boxes: isize = self.grid.positions_of(BIG_BOX_START).map(|p| p.x + p.y * 100).inspect(|c| print!("{} ", c)).sum();
        boxes + big_boxes
    }
}

#[cfg(test)]
mod tests {
    use crate::day15::tests::SIMPLE_2;
    use crate::day15::warehouse::Warehouse;

    #[test]
    fn creates_wider_warehouse() {
        let warehouse = Warehouse::wider(SIMPLE_2);

        warehouse.grid.print();
        assert_eq!(warehouse.grid.max_x, 14);
        assert_eq!(warehouse.grid.max_y, 7);
    }

    #[test]
    fn counts_big_boxes() {
        let warehouse = Warehouse::wider(SIMPLE_2);

        assert_eq!(warehouse.boxes_sum(), 306 + 308 + 406);
    }
}
