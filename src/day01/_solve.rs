#[cfg(test)]
mod tests {
    use crate::toolbox::daily::daily_input;
    use crate::day01::locations::Locations;

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

    #[test]
    fn solves_part2_example() {
        let locations = Locations::from(EXAMPLE);

        assert_eq!(locations.similarity_score(), 31)
    }
    
    #[test]
    fn solves_part2() {
        let input = daily_input(1);
        let locations = Locations::from(&input);
        
        assert_eq!(locations.similarity_score(), 21607792)
    }

    const EXAMPLE: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";
}