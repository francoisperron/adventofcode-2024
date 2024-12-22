mod codes;
mod pads;
mod robot_command;

#[cfg(test)]
mod tests {
    use crate::day21::codes::Codes;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        let codes = Codes::from(EXAMPLE, 2);

        assert_eq!(codes.complexities(), 126384);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(21);
        let codes = Codes::from(&input, 2);

        assert_eq!(codes.complexities(), 217662);
    }

    #[test]
    fn solves_part2_example() {
        let codes = Codes::from(EXAMPLE, 25);

        assert_eq!(codes.complexities(), 154115708116294);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(21);
        let codes = Codes::from(&input, 25);

        assert_eq!(codes.complexities(), 263617786809000);
    }

    const EXAMPLE: &str = "\
029A
980A
179A
456A
379A";
}
