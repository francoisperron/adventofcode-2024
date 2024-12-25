use std::ops::{BitAnd, BitOr, BitXor};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Bit(pub bool);

impl Bit {
    pub fn from(input: &str) -> Option<Bit> {
        match input {
            "1" => Some(Bit(true)),
            "0" => Some(Bit(false)),
            _ => None,
        }
    }

    pub fn value(&self) -> char {
        match self {
            Bit(true) => '1',
            Bit(false) => '0',
        }
    }
}

impl BitAnd for Bit {
    type Output = Bit;

    fn bitand(self, other: Self) -> Self::Output {
        Bit(self.0 && other.0)
    }
}

impl BitOr for Bit {
    type Output = Bit;

    fn bitor(self, other: Self) -> Self::Output {
        Bit(self.0 || other.0)
    }
}

impl BitXor for Bit {
    type Output = Bit;

    fn bitxor(self, other: Self) -> Self::Output {
        Bit(self.0 ^ other.0)
    }
}
