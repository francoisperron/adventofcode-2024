#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn turn_right(&self) -> Direction{
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
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
}