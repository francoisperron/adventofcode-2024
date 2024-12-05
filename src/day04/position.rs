use std::ops::{Add, Mul};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Ord, PartialOrd)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }

    pub fn around() -> [Position; 8] {
        [
            Position::new(-1, 1),
            Position::new(0, 1),
            Position::new(1, 1),
            Position::new(-1, 0),
            Position::new(1, 0),
            Position::new(-1, -1),
            Position::new(0, -1),
            Position::new(1, -1),
        ]
    }

    pub fn around_me(&self) -> [Position; 8] {
        Position::around().map(|d| Position::new(self.x + d.x, self.y + d.y))
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Self) -> Self::Output {
        Position::new(self.x + other.x, self.y + other.y)
    }
}

impl Mul<usize> for Position {
    type Output = Position;

    fn mul(self, rhs: usize) -> Self::Output {
        Position::new(self.x * rhs as isize, self.y * rhs as isize)
    }
}

#[cfg(test)]
mod tests {
    use crate::day04::position::Position;

    #[test]
    fn adds_positions() {
        let p1 = Position::new(1, 2);
        let p2 = Position::new(3, 4);
        let p3 = Position::new(4, 6);

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn gets_positions_around() {
        let p = Position::new(1, 1);
        let around = p.around_me();

        assert_eq!(around.len(), 8);
        assert_eq!(around[0], Position::new(1 - 1, 1 + 1));
        assert_eq!(around[1], Position::new(1, 1 + 1));
        assert_eq!(around[2], Position::new(1 + 1, 1 + 1));
        assert_eq!(around[3], Position::new(1 - 1, 1));
        assert_eq!(around[4], Position::new(1 + 1, 1));
        assert_eq!(around[5], Position::new(1 - 1, 1 - 1));
        assert_eq!(around[6], Position::new(1, 1 - 1));
        assert_eq!(around[7], Position::new(1 + 1, 1 - 1));
    }
}
