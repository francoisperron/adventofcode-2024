use std::ops::{Add, Mul, Rem};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Ord, PartialOrd)]
pub struct XY {
    pub x: isize,
    pub y: isize,
}

impl XY {
    pub fn new(xy: (isize, isize)) -> XY {
        XY { x: xy.0, y: xy.1 }
    }

    pub fn determinant(&self, other: &XY) -> isize {
        self.y * other.x - self.x * other.y
    }
}

impl Add for XY {
    type Output = XY;
    fn add(self, other: Self) -> Self::Output {
        XY { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Mul<isize> for XY {
    type Output = XY;
    fn mul(self, factor: isize) -> Self::Output {
        XY { x: self.x * factor, y: self.y * factor }
    }
}

impl Rem for XY {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        Self { x: ((self.x % other.x) + other.x) % other.x, y: ((self.y % other.y) + other.y) % other.y }
    }
}

#[cfg(test)]
mod tests {
    use crate::toolbox::xy::XY;

    #[test]
    fn calculates_determinant() {
        let xy = XY::new((10, 20));
        let xy2 = XY::new((30, 40));

        assert_eq!(xy.determinant(&xy2), 30 * 20 - 10 * 40);
    }

    #[test]
    fn adds_xy() {
        let xy = XY::new((1, 2));
        let xy2 = XY::new((3, 4));

        assert_eq!(xy + xy2, XY::new((4, 6)));
    }

    #[test]
    fn multiplies_xy() {
        let xy = XY::new((1, 2));

        assert_eq!(xy * 2, XY::new((2, 4)));
    }

    #[test]
    fn modulo_xy() {
        let xy = XY::new((11, 23));
        let xy2 = XY::new((2, 5));

        assert_eq!(xy % xy2, XY::new((1, 3)));
    }
}
