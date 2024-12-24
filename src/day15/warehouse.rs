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
            match self.move_object(&robot_position, &direction) {
                Some(moves) => {
                    moves.iter().for_each(|(from, _, _)| self.grid.set_element_at(from, '.'));
                    moves.iter().for_each(|(_, to, value)| self.grid.set_element_at(to, *value));
                    robot_position = robot_position.move_towards(&direction);
                }
                None => continue,
            }
        }
    }

    fn move_object(&mut self, position: &Position, direction: &Direction) -> Option<Vec<(Position, Position, char)>> {
        let next_position = &position.move_towards(direction);
        let mut moves = Vec::from([(*position, *next_position, *self.grid.element_at(position))]);

        match *self.grid.element_at(next_position) {
            WALL => None,
            FREE_SPACE => Some(moves),
            BOX => {
                if let Some(new_moves) = self.move_object(next_position, direction) {
                    moves.extend(&new_moves);
                    Some(moves)
                } else {
                    None
                }
            }
            BIG_BOX_START => {
                if direction == &Direction::Left || direction == &Direction::Right {
                    if let Some(new_moves) = self.move_object(next_position, direction) {
                        moves.extend(&new_moves);
                        Some(moves)
                    } else {
                        None
                    }
                } else {
                    let start_moves = self.move_object(next_position, direction);
                    let end_moves = self.move_object(&next_position.move_towards(&Direction::Right), direction);
                    if let (Some(start_moves), Some(end_moves)) = (start_moves, end_moves) {
                        moves.extend(&start_moves);
                        moves.extend(&end_moves);
                        Some(moves)
                    } else {
                        None
                    }
                }
            }
            BIG_BOX_END => {
                if direction == &Direction::Left || direction == &Direction::Right {
                    if let Some(new_moves) = self.move_object(next_position, direction) {
                        moves.extend(&new_moves);
                        Some(moves)
                    } else {
                        None
                    }
                } else {
                    let start_moves = self.move_object(&next_position.move_towards(&Direction::Left), direction);
                    let end_moves = self.move_object(next_position, direction);
                    if let (Some(start_moves), Some(end_moves)) = (start_moves, end_moves) {
                        moves.extend(&end_moves);
                        moves.extend(&start_moves);
                        Some(moves)
                    } else {
                        None
                    }
                }
            }
            _ => panic!("Invalid object"),
        }
    }

    pub fn boxes_sum(&self) -> isize {
        let boxes: isize = self.grid.positions_of(BOX).map(|p| p.x + p.y * 100).sum();
        let big_boxes: isize = self.grid.positions_of(BIG_BOX_START).map(|p| p.x + p.y * 100).sum();
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

        assert_eq!(warehouse.grid.max_x, 14);
        assert_eq!(warehouse.grid.max_y, 7);
    }

    #[test]
    fn counts_big_boxes() {
        let warehouse = Warehouse::wider(SIMPLE_2);

        assert_eq!(warehouse.boxes_sum(), 306 + 308 + 406);
    }

    #[test]
    fn moves_big_boxes_left() {
        let input = "\
##############
##..........##
##..........##
##.[][]@....##
##..........##
##..........##
##############

<<";
        let mut warehouse = Warehouse::from(input);
        warehouse.move_robot();

        let expected = "\
##############
##..........##
##..........##
##[][]@.....##
##..........##
##..........##
##############";
        assert_eq!(warehouse.grid.print(), expected);
    }

    #[test]
    fn moves_big_boxes_right() {
        let input = "\
##############
##..........##
##..........##
##....@[][].##
##..........##
##..........##
##############

>>";
        let mut warehouse = Warehouse::from(input);
        warehouse.move_robot();

        let expected = "\
##############
##..........##
##..........##
##.....@[][]##
##..........##
##..........##
##############";
        assert_eq!(warehouse.grid.print(), expected);
    }

    #[test]
    fn moves_big_boxes_up_in_line_start() {
        let input = "\
##############
##.....#....##
##..........##
##.....[]...##
##.....[]...##
##.....@....##
##############

^";
        let mut warehouse = Warehouse::from(input);
        warehouse.move_robot();

        let expected = "\
##############
##.....#....##
##.....[]...##
##.....[]...##
##.....@....##
##..........##
##############";
        assert_eq!(warehouse.grid.print(), expected);
    }

    #[test]
    fn moves_big_boxes_up_in_line_end() {
        let input = "\
##############
##.....#....##
##..........##
##.....[]...##
##.....[]...##
##......@...##
##############

^";
        let mut warehouse = Warehouse::from(input);
        warehouse.move_robot();

        let expected = "\
##############
##.....#....##
##.....[]...##
##.....[]...##
##......@...##
##..........##
##############";
        assert_eq!(warehouse.grid.print(), expected);
    }

    #[test]
    fn moves_big_boxes_up() {
        let input = "\
##############
##.....#....##
##..........##
##....[][]..##
##.....[]...##
##.....@....##
##############

^^";
        let mut warehouse = Warehouse::from(input);
        warehouse.move_robot();

        let expected = "\
##############
##.....#....##
##....[][]..##
##.....[]...##
##.....@....##
##..........##
##############";
        assert_eq!(warehouse.grid.print(), expected);
    }

    #[test]
    fn moves_big_boxes_down_line_start() {
        let input = "\
##############
##.....@....##
##.....[]...##
##.....[]...##
##..........##
##.....#....##
##############

v";
        let mut warehouse = Warehouse::from(input);
        warehouse.move_robot();

        let expected = "\
##############
##..........##
##.....@....##
##.....[]...##
##.....[]...##
##.....#....##
##############";
        assert_eq!(warehouse.grid.print(), expected);
    }

    #[test]
    fn moves_big_boxes_down_line_end() {
        let input = "\
##############
##......@...##
##.....[]...##
##.....[]...##
##..........##
##.....#....##
##############

v";
        let mut warehouse = Warehouse::from(input);
        warehouse.move_robot();

        let expected = "\
##############
##..........##
##......@...##
##.....[]...##
##.....[]...##
##.....#....##
##############";
        assert_eq!(warehouse.grid.print(), expected);
    }

    #[test]
    fn moves_big_boxes_down() {
        let input = "\
##############
##......@...##
##.....[]...##
##....[][]..##
##..........##
##....#.....##
##############

vv";
        let mut warehouse = Warehouse::from(input);
        warehouse.move_robot();

        let expected = "\
##############
##..........##
##......@...##
##.....[]...##
##....[][]..##
##....#.....##
##############";
        assert_eq!(warehouse.grid.print(), expected);
    }
}
