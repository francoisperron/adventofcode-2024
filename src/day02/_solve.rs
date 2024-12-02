#[cfg(test)]
mod tests {
    use crate::daily::daily_input;
    use crate::day02::reports::Reports;

    #[test]
    fn solves_part1_example() {
        let reports = Reports::from(EXAMPLE);

        assert_eq!(reports.safe_count(), 2);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(2);
        let reports = Reports::from(&input);

        assert_eq!(reports.safe_count(), 502)
    }
    
    #[test]
    fn solves_part2_example() {
        let reports = Reports::from(EXAMPLE);

        assert_eq!(reports.safe_count_with_problem_dampener(), 4);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(2);
        let reports = Reports::from(&input);

        assert_eq!(reports.safe_count_with_problem_dampener(), 544)
    }

    const EXAMPLE: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
}
