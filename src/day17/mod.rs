mod program;

#[cfg(test)]
mod tests {
    use crate::day17::program::Program;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        let mut program = Program::from(EXAMPLE_PART1);

        assert_eq!(program.run(), vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(17);
        let mut program = Program::from(&input);

        assert_eq!(program.run(), vec![4, 1, 7, 6, 4, 1, 0, 2, 7]);
    }

    #[test]
    fn solves_part2_example() {
        let mut program = Program::from(EXAMPLE_PART2);

        assert_eq!(program.copy(), 117440);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(17);
        let mut program = Program::from(&input);

        assert_eq!(program.copy(), 164279024971453);
    }

    const EXAMPLE_PART1: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const EXAMPLE_PART2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
}
