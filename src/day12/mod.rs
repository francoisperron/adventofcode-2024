mod garden;
mod price;
mod region;

#[cfg(test)]
mod tests {
    use crate::day12::garden::Garden;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        let garden = Garden::from(EXAMPLE);
        let price = garden.fences_price();

        assert_eq!(price.total, 1930);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(12);
        let garden = Garden::from(&input);
        let price = garden.fences_price();

        assert_eq!(price.total, 1363682);
    }

    #[test]
    fn solves_part2_example1() {
        let garden = Garden::from(EXAMPLE_1);
        let price = garden.fences_price();

        assert_eq!(price.total_with_discount, 80);
    }

    #[test]
    fn solves_part2_example() {
        let garden = Garden::from(EXAMPLE);
        let price = garden.fences_price();

        assert_eq!(price.total_with_discount, 1206);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(12);
        let garden = Garden::from(&input);
        let price = garden.fences_price();

        assert_eq!(price.total_with_discount, 787680);
    }

    const EXAMPLE_1: &str = "\
AAAA
BBCD
BBCC
EEEC";

    const EXAMPLE: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
}
