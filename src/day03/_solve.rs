#[cfg(test)]
mod tests {
    use crate::daily::daily_input;
    use crate::day03::instructions::Instructions;

    #[test]
    fn solves_part1_example() {
        let instructions = Instructions::from(EXAMPLE);

        assert_eq!(instructions.sum(), 161);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(3);
        let instructions = Instructions::from(&input);

        assert_eq!(instructions.sum(), 173731097);
    }

    #[test]
    fn solves_part2_example() {
        let instructions = Instructions::from(EXAMPLE);

        assert_eq!(instructions.enabled_sum(), 48);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(3);
        let instructions = Instructions::from(&input);

        assert_eq!(instructions.enabled_sum(), 93729253);
    }

    const EXAMPLE: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5)";
}
