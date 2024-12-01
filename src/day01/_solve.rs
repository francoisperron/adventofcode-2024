#[cfg(test)]
mod tests {
    use crate::daily::daily_input;
    use crate::day01::_solve::EXAMPLE;
    use crate::day01::location::Locations;

    #[test]
    fn solves_part1_example() {
        let locations = Locations::from(EXAMPLE);

        assert_eq!(locations.total_distance(), 11)
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(1);
        let locations = Locations::from(&input);

        assert_eq!(locations.total_distance(), 1834060)
    }
}

const EXAMPLE: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";