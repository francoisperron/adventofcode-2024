mod maze;

#[cfg(test)]
mod tests {
    use crate::day16::maze::Maze;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example1() {
        let maze = Maze::from(EXAMPLE_1);

        assert_eq!(maze.shortest_path_score(), 7036);
    }

    #[test]
    fn solves_part1_example2() {
        let maze = Maze::from(EXAMPLE_2);

        assert_eq!(maze.shortest_path_score(), 11048);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(16);
        let maze = Maze::from(&input);

        assert_eq!(maze.shortest_path_score(), 99488);
    }

    #[test]
    fn solves_part2_example1() {
        let mut maze = Maze::from(EXAMPLE_1);

        assert_eq!(maze.places_to_sit(), 45);
    }

    #[test]
    fn solves_part2_example2() {
        let mut maze = Maze::from(EXAMPLE_2);

        assert_eq!(maze.places_to_sit(), 64);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(16);
        let mut maze = Maze::from(&input);

        assert_eq!(maze.places_to_sit(), 531);
    }

    const EXAMPLE_1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const EXAMPLE_2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
}
