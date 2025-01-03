mod warehouse;

#[cfg(test)]
mod tests {
    use crate::day15::warehouse::Warehouse;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_simple() {
        let mut warehouse = Warehouse::from(SIMPLE);
        warehouse.move_robot();

        assert_eq!(warehouse.boxes_sum(), 2028);
    }

    #[test]
    fn solves_part1_example() {
        let mut warehouse = Warehouse::from(EXAMPLE);
        warehouse.move_robot();

        assert_eq!(warehouse.boxes_sum(), 10092);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(15);
        let mut warehouse = Warehouse::from(&input);
        warehouse.move_robot();

        assert_eq!(warehouse.boxes_sum(), 1527563);
    }

    #[test]
    fn solves_part2_simple() {
        let mut warehouse = Warehouse::wider(SIMPLE_2);
        warehouse.move_robot();

        assert_eq!(warehouse.boxes_sum(), 105 + 207 + 306);
    }

    #[test]
    fn solves_part2_example() {
        let mut warehouse = Warehouse::wider(EXAMPLE);
        warehouse.move_robot();

        assert_eq!(warehouse.boxes_sum(), 9021);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(15);
        let mut warehouse = Warehouse::wider(&input);
        warehouse.move_robot();

        assert_eq!(warehouse.boxes_sum(), 1521635);
    }

    const SIMPLE: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    pub const SIMPLE_2: &str = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    const EXAMPLE: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
}
