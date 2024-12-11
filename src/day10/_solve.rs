#[cfg(test)]
mod tests {
    use crate::day10::topographic_map::TopographicMap;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        let map = TopographicMap::from(EXAMPLE);

        assert_eq!(map.trailheads_score(), 36);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(10);
        let map = TopographicMap::from(&input);

        assert_eq!(map.trailheads_score(), 468);
    }

    #[test]
    fn solves_part2_example() {
        let map = TopographicMap::from(EXAMPLE);

        assert_eq!(map.trailheads_ratings(), 81);
    }
    
    #[test]
    fn solves_part2() {
        let input = daily_input(10);
        let map = TopographicMap::from(&input);

        assert_eq!(map.trailheads_ratings(), 966);
    }

    const EXAMPLE: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
}
