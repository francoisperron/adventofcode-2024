mod map;

#[cfg(test)]
mod tests {
    use crate::day06::map::Map;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        let mut map = Map::from(EXAMPLE);
        map.predict();

        assert_eq!(map.patrolled_areas_count(), 41);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(6);
        let mut map = Map::from(&input);
        map.predict();

        assert_eq!(map.patrolled_areas_count(), 5101);
    }

    #[test]
    fn solves_part2_example() {
        let mut map = Map::from(EXAMPLE);

        assert_eq!(map.obstructions_count(), 6)
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(6);
        let mut map = Map::from(&input);

        assert_eq!(map.obstructions_count(), 1951);
    }

    const EXAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
}
