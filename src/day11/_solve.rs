#[cfg(test)]
mod tests {
    use crate::day11::stones::Stones;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example_test() {
        let stones = Stones::from(EXAMPLE);

        assert_eq!(stones.blink(4).count(), 9);
    }

    #[test]
    fn solves_part1_example() {
        let stones = Stones::from(EXAMPLE);

        assert_eq!(stones.blink(25).count(), 55312);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(11);
        let stones = Stones::from(&input);

        assert_eq!(stones.blink(25).count(), 186203);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(11);
        let stones = Stones::from(&input);

        assert_eq!(stones.blink(75).count(), 221291560078593);
    }

    const EXAMPLE: &str = "125 17";
}
