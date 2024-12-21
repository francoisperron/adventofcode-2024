mod race;

#[cfg(test)]
mod tests {
    use crate::day20::race::Race;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        let mut race = Race::from(EXAMPLE);

        assert_eq!(race.cheats_over(1), 44);
        assert_eq!(race.cheats_over(50), 1);
    }

    #[test]
    #[ignore] // 20 secs on my mbp
    fn solves_part1() {
        let input = daily_input(20);
        let mut race = Race::from(&input);

        assert_eq!(race.cheats_over(100), 1343);
    }

    const EXAMPLE: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
}
