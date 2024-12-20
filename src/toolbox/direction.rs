#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Ord, PartialOrd)]
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

    pub fn from(value: char) -> Direction {
        match value {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!("Invalid direction: {}", value),
        }
    }

    pub fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn reverse(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::toolbox::direction::Direction;

    #[test]
    fn directions_from_char() {
        assert_eq!(Direction::from('^'), Direction::Up);
        assert_eq!(Direction::from('v'), Direction::Down);
        assert_eq!(Direction::from('>'), Direction::Right);
        assert_eq!(Direction::from('<'), Direction::Left);
    }

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
