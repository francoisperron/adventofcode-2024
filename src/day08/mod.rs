mod antennas;

#[cfg(test)]
mod tests {
    use crate::day08::antennas::Antennas;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        let antennas = Antennas::from(EXAMPLE);

        assert_eq!(antennas.antinodes_count(), 14);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(8);
        let antennas = Antennas::from(&input);

        assert_eq!(antennas.antinodes_count(), 228);
    }

    #[test]
    fn solves_part2_example() {
        let antennas = Antennas::from(EXAMPLE);

        assert_eq!(antennas.resonant_antinodes_count(), 34);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(8);
        let antennas = Antennas::from(&input);

        assert_eq!(antennas.resonant_antinodes_count(), 766);
    }

    const EXAMPLE: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
}
