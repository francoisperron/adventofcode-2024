mod calibrations;

#[cfg(test)]
mod tests {
    use crate::day07::calibrations::{Calibrations, Operation};
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        let calibrations = Calibrations::from(EXAMPLE);

        assert_eq!(calibrations.solve(PART1_OPERATIONS), 3749)
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(7);
        let calibrations = Calibrations::from(&input);

        assert_eq!(calibrations.solve(PART1_OPERATIONS), 850435817339)
    }

    #[test]
    fn solves_part2_example() {
        let calibrations = Calibrations::from(EXAMPLE);

        assert_eq!(calibrations.solve(PART2_OPERATIONS), 11387)
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(7);
        let calibrations = Calibrations::from(&input);

        assert_eq!(calibrations.solve(PART2_OPERATIONS), 104824810233437)
    }

    const PART1_OPERATIONS: &[Operation; 2] = &[Operation::Addition, Operation::Multiplication];
    const PART2_OPERATIONS: &[Operation; 3] = &[Operation::Addition, Operation::Multiplication, Operation::Combination];

    const EXAMPLE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
}
