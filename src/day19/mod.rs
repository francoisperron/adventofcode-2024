mod towels;

#[cfg(test)]
mod tests {
    use crate::day19::towels::Towels;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        let towels = Towels::from(EXAMPLE);

        assert_eq!(towels.possible_designs(), 6);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(19);
        let towels = Towels::from(&input);

        assert_eq!(towels.possible_designs(), 313);
    }

    #[test]
    fn solves_part2_example() {
        let towels = Towels::from(EXAMPLE);

        assert_eq!(towels.all_possible_designs(), 16);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(19);
        let towels = Towels::from(&input);

        assert_eq!(towels.all_possible_designs(), 666491493769758);
    }

    const EXAMPLE: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
}
