mod race_complex;

#[cfg(test)]
mod tests {
    use crate::day20::race_complex::RaceComplex;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1() {
        let input = daily_input(20);
        let race = RaceComplex::from(&input);

        let cheats = race.cheat(|cheats: i32| cheats == 2);

        assert_eq!(cheats, 1343);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(20);
        let race = RaceComplex::from(&input);

        let cheats = race.cheat(|cheats: i32| cheats < 21);

        assert_eq!(cheats, 982891);
    }
}
