#[cfg(test)]
mod tests {
    use crate::day14::robots::Robots;
    use crate::toolbox::daily::daily_input;
    use crate::toolbox::XY;

    #[test]
    fn solves_part1_example() {
        let mut robots = Robots::from(EXAMPLE, XY::new((11, 7)));
        robots.move_(100);

        assert_eq!(robots.safety_factor(), 12);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(14);
        let mut robots = Robots::from(&input, XY::new((101, 103)));
        robots.move_(100);

        assert_eq!(robots.safety_factor(), 229069152);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(14);
        let mut robots = Robots::from(&input, XY::new((101, 103)));

        let moves = robots.moves_for_easter_egg();
        // robots.print_xmas_tree();
        assert_eq!(moves, 7383);
    }

    const EXAMPLE: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
}
