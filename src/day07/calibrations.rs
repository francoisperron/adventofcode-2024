pub struct Calibrations {
    calibrations: Vec<Calibration>
}

impl Calibrations {
    pub fn from(input: &str) -> Calibrations {
        let calibrations = input.lines().map(Calibration::from).collect();
        Calibrations { calibrations }
    }
    
    pub fn solve(&self, operations: &[Operation]) -> usize {
        self.calibrations.iter()
            .filter(|calibration| calibration.can_solve(operations))
            .map(|calibration| calibration.result)
            .sum()
    }
}

pub struct Calibration {
    result: usize,
    numbers: Vec<usize>
}

impl Calibration {
    pub fn from(input: &str) -> Calibration {
        let (result, numbers) = input.split_once(": ").unwrap();
        Calibration {
            result: result.parse().unwrap(),
            numbers: numbers.split_whitespace().map(|n| n.parse().unwrap()).collect()
        }
    }
    
    pub fn can_solve(&self, operations: &[Operation]) -> bool {
        self.try_to_solve(0, 0, operations)
    }

    fn try_to_solve(&self, index: usize, result: usize, operations: &[Operation]) -> bool {
        if index == self.numbers.len() {
            return result == self.result;
        }

        operations
            .iter()
            .any(|operation| {
                let new_result = operation.process(result, self.numbers[index]);
                self.try_to_solve(index + 1, new_result, operations)
            })
    }
}

pub enum Operation {
    Addition,
    Multiplication,
    Combination
}

impl Operation {
    fn process(&self, a: usize, b: usize) -> usize {
        match self {
            Operation::Addition => a + b,
            Operation::Multiplication => if a == 0 { b } else { a * b },
            Operation::Combination => format!("{}{}", a, b).parse().unwrap()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_by_additions() {
        let c = Calibration { result: 6, numbers: vec![1, 2, 3] };

        assert!(c.can_solve(&[Operation::Addition]));
    }

    #[test]
    fn cant_solves_by_additions() {
        let c = Calibration { result: 100, numbers: vec![1, 2, 3] };

        assert!(!c.can_solve(&[Operation::Addition]));
    }

    #[test]
    fn solves_by_multiplications() {
        let c = Calibration { result: 12, numbers: vec![2, 2, 3] };

        assert!(c.can_solve(&[Operation::Multiplication]));
    }

    #[test]
    fn cant_solve_by_multiplications() {
        let c = Calibration { result: 100, numbers: vec![2, 2, 3] };

        assert!(!c.can_solve(&[Operation::Multiplication]));
    }

    #[test]
    fn solves_by_both() {
        // 1 + 2 * 3
        let c = Calibration { result: 9, numbers: vec![1, 2, 3] };

        assert!(c.can_solve(&[Operation::Addition, Operation::Multiplication]));
    }

    #[test]
    fn cant_solve_by_both() {
        let c = Calibration { result: 100, numbers: vec![1, 2, 3] };

        assert!(!c.can_solve(&[Operation::Addition, Operation::Multiplication]));
    }

    #[test]
    fn solves_by_combinations() {
        let c = Calibration { result: 156, numbers: vec![15, 6] };

        assert!(c.can_solve(&[Operation::Combination]));
    }
}