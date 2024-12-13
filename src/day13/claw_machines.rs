use crate::toolbox::XY;
use std::str::Lines;

#[derive(Debug)]
pub struct ClawMachines {
    machines: Vec<ClawMachine>,
}

impl ClawMachines {
    pub fn from(input: &str) -> ClawMachines {
        ClawMachines { machines: input.split("\n\n").map(ClawMachine::from).collect() }
    }

    pub fn add_conversion_error(&mut self, error: isize) {
        for machine in &mut self.machines {
            machine.prize.x += error;
            machine.prize.y += error;
        }
    }

    pub fn tokens(&self) -> isize {
        self.machines.iter().flat_map(|machine| machine.tokens()).sum()
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct ClawMachine {
    button_a: XY,
    button_b: XY,
    prize: XY,
}

impl ClawMachine {
    pub fn from(input: &str) -> ClawMachine {
        let mut lines = input.lines();
        let button_a = Self::parse_line(&mut lines, "+");
        let button_b = Self::parse_line(&mut lines, "+");
        let prize = Self::parse_line(&mut lines, "=");
        ClawMachine { button_a, button_b, prize }
    }

    fn parse_line(lines: &mut Lines, delimiter: &str) -> XY {
        lines
            .next()
            .unwrap()
            .split_once(delimiter)
            .unwrap()
            .1
            .split_once(format!(", Y{}", delimiter).as_str())
            .map(|(x, y)| XY::new((x.parse().unwrap(), y.parse().unwrap())))
            .unwrap()
    }

    pub fn new(button_a: (isize, isize), button_b: (isize, isize), prize: (isize, isize)) -> ClawMachine {
        ClawMachine { button_a: XY::new(button_a), button_b: XY::new(button_b), prize: XY::new(prize) }
    }

    pub fn tokens(&self) -> Option<isize> {
        self.pushes().map(|p| p.x * 3 + p.y)
    }

    pub fn pushes(&self) -> Option<XY> {
        let b_push = self.button_a.determinant(&self.prize) / self.button_a.determinant(&self.button_b);
        let a_push = (self.prize.x - b_push * self.button_b.x) / self.button_a.x;

        if self.button_a * a_push + self.button_b * b_push == self.prize {
            Some(XY::new((a_push, b_push)))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_pushes_needed() {
        let machine = ClawMachine::new((94, 34), (22, 67), (8400, 5400));

        assert_eq!(machine.pushes(), Some(XY::new((80, 40))));
    }

    #[test]
    fn none_when_no_pushes_combination() {
        let machine = ClawMachine::new((26, 66), (67, 21), (12748, 12176));

        assert_eq!(machine.pushes(), None);
    }

    #[test]
    fn calculates_minimal_pushes_needed() {
        let machine = ClawMachine::new((17, 86), (84, 37), (7870, 6450));

        assert_eq!(machine.pushes(), Some(XY::new((38, 86))));
    }
}
