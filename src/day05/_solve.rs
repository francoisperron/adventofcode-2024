#[cfg(test)]
mod tests {
    use crate::day05::manual_update::ManualUpdate;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        let manual_update = ManualUpdate::from(EXAMPLE);

        assert_eq!(manual_update.valid_updates_middle_page_sum(), 143);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(5);
        let manual_update = ManualUpdate::from(&input);

        assert_eq!(manual_update.valid_updates_middle_page_sum(), 6267);
    }

    #[test]
    fn solves_part2_example() {
        let manual_update = ManualUpdate::from(EXAMPLE);

        assert_eq!(manual_update.fixed_invalid_updates_middle_page_sum(), 123);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(5);
        let manual_update = ManualUpdate::from(&input);

        assert_eq!(manual_update.fixed_invalid_updates_middle_page_sum(), 5184);
    }

    const EXAMPLE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
}
