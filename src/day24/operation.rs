use crate::day24::bit::Bit;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    pub fn from(input: &str) -> Operation {
        match input {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => unreachable!(),
        }
    }

    pub fn evaluate(&self, a: Bit, b: Bit) -> Bit {
        match self {
            Operation::And => a & b,
            Operation::Or => a | b,
            Operation::Xor => a ^ b,
        }
    }
}
