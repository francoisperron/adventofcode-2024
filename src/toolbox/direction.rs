#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [Direction::Up, Direction::Right, Direction::Down, Direction::Left]
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::toolbox::direction::Direction;

    #[test]
    fn turns_right() {
        assert_eq!(Direction::Up.turn_right(), Direction::Right);
        assert_eq!(Direction::Right.turn_right(), Direction::Down);
        assert_eq!(Direction::Down.turn_right(), Direction::Left);
        assert_eq!(Direction::Left.turn_right(), Direction::Up);
    }

    #[test]
    fn turns_left() {
        assert_eq!(Direction::Up.turn_left(), Direction::Left);
        assert_eq!(Direction::Left.turn_left(), Direction::Down);
        assert_eq!(Direction::Down.turn_left(), Direction::Right);
        assert_eq!(Direction::Right.turn_left(), Direction::Up);
    }
}
