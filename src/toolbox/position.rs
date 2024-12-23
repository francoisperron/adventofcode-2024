use crate::toolbox::direction::Direction;
use itertools::Itertools;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Ord, PartialOrd)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }

    pub fn from(input: &str) -> Position {
        let (x, y) = input.split(',').map(|part| part.parse::<isize>().unwrap()).collect_tuple().unwrap();
        Position { x, y }
    }

    pub fn around() -> [Position; 8] {
        [Position::new(-1, 1), Position::new(0, 1), Position::new(1, 1), Position::new(-1, 0), Position::new(1, 0), Position::new(-1, -1), Position::new(0, -1), Position::new(1, -1)]
    }

    pub fn around_me(&self) -> impl Iterator<Item = Position> {
        Position::around().map(|d| Position::new(self.x + d.x, self.y + d.y)).into_iter()
    }

    pub fn around_4() -> [Position; 4] {
        [Position::new(0, 1), Position::new(-1, 0), Position::new(1, 0), Position::new(0, -1)]
    }

    pub fn around_me_4(&self) -> impl Iterator<Item = Position> {
        Position::around_4().map(|d| Position::new(self.x + d.x, self.y + d.y)).into_iter()
    }

    pub fn move_towards(&self, direction: &Direction) -> Position {
        match direction {
            Direction::Up => *self + Position::new(0, -1),
            Direction::Down => *self + Position::new(0, 1),
            Direction::Left => *self + Position::new(-1, 0),
            Direction::Right => *self + Position::new(1, 0),
        }
    }

    pub fn corner(&self, direction: &Direction) -> (Position, Position, Position) {
        let corner_3 = self.move_towards(direction);
        let corner_2 = corner_3.move_towards(&direction.turn_left());
        let corner_1 = corner_2.move_towards(&direction.turn_left().turn_left());
        (corner_1, corner_2, corner_3)
    }

    pub fn manhattan_distance(&self, other: &Position) -> usize {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Self) -> Self::Output {
        Position::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Self) -> Self::Output {
        Position::new(self.x - other.x, self.y - other.y)
    }
}

impl Neg for Position {
    type Output = Position;

    fn neg(self) -> Self::Output {
        Position::new(-self.x, -self.y)
    }
}

impl Mul<usize> for Position {
    type Output = Position;

    fn mul(self, factor: usize) -> Self::Output {
        Position::new(self.x * factor as isize, self.y * factor as isize)
    }
}

#[cfg(test)]
mod tests {
    use crate::toolbox::direction::Direction;
    use crate::toolbox::position::Position;

    #[test]
    fn adds_positions() {
        let p1 = Position::new(1, 2);
        let p2 = Position::new(3, 4);
        let p3 = Position::new(4, 6);

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn subtracts_positions() {
        let p1 = Position::new(4, 6);
        let p2 = Position::new(3, 4);
        let p3 = Position::new(1, 2);

        assert_eq!(p1 - p2, p3);
    }

    #[test]
    fn negates_position() {
        let p = Position::new(1, 2);

        assert_eq!(-p, Position::new(-1, -2));
    }

    #[test]
    fn multiplies_by_factor() {
        let p = Position::new(1, 2);
        let p2 = p * 2;

        assert_eq!(p2, Position::new(2, 4));
    }

    #[test]
    fn moves_towards() {
        let p = Position::new(1, 1);

        assert_eq!(p.move_towards(&Direction::Up), Position::new(1, 0));
        assert_eq!(p.move_towards(&Direction::Down), Position::new(1, 2));
        assert_eq!(p.move_towards(&Direction::Left), Position::new(0, 1));
        assert_eq!(p.move_towards(&Direction::Right), Position::new(2, 1));
    }

    #[test]
    fn gets_positions_around() {
        let p = Position::new(1, 1);
        let around = p.around_me().collect::<Vec<Position>>();

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

    #[test]
    fn gets_4_positions_around() {
        let p = Position::new(1, 1);
        let around = p.around_me_4().collect::<Vec<Position>>();

        assert_eq!(around.len(), 4);
        assert_eq!(around[0], Position::new(1, 1 + 1));
        assert_eq!(around[1], Position::new(1 - 1, 1));
        assert_eq!(around[2], Position::new(1 + 1, 1));
        assert_eq!(around[3], Position::new(1, 1 - 1));
    }

    #[test]
    fn gets_up_corner() {
        let p = Position::new(0, 0);
        let (corner_1, corner_2, corner_3) = p.corner(&Direction::Up);

        // c2 c3
        // c1 p
        assert_eq!(corner_1, Position::new(-1, 0));
        assert_eq!(corner_2, Position::new(-1, -1));
        assert_eq!(corner_3, Position::new(0, -1));
    }

    #[test]
    fn gets_left_corner() {
        let p = Position::new(0, 0);
        let (corner_1, corner_2, corner_3) = p.corner(&Direction::Left);

        // c3 p
        // c2 c1
        assert_eq!(corner_1, Position::new(0, 1));
        assert_eq!(corner_2, Position::new(-1, 1));
        assert_eq!(corner_3, Position::new(-1, 0));
    }

    #[test]
    fn calculates_manhattan_distance() {
        let p1 = Position::new(10, 1);
        let p2 = Position::new(20, 2);

        assert_eq!(p1.manhattan_distance(&p2), 11);
        assert_eq!(p2.manhattan_distance(&p1), 11);
    }
}
