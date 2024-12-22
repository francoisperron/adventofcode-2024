mod secrets;

#[cfg(test)]
mod tests {
    use crate::day22::secrets::Secrets;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        let secrets = Secrets::from(EXAMPLE);

        assert_eq!(secrets.secrets_sum(2000), 37327623);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(22);
        let secrets = Secrets::from(&input);

        assert_eq!(secrets.secrets_sum(2000), 20071921341);
    }

    #[test]
    fn solves_part2_example() {
        let secrets = Secrets::from(EXAMPLE);

        assert_eq!(secrets.bananas_max(2000), 24);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(22);
        let secrets = Secrets::from(&input);

        assert_eq!(secrets.bananas_max(2000), 2242);
    }

    const EXAMPLE: &str = "\
1
10
100
2024";
}
