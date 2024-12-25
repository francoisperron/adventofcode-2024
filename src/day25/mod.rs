mod schematics;

#[cfg(test)]
mod tests {
    use crate::day25::schematics::Schematics;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        let schematics = Schematics::from(EXAMPLE);

        assert_eq!(schematics.overlapping(), 3);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(25);
        let schematics = Schematics::from(&input);

        assert_eq!(schematics.overlapping(), 3495);
    }

    const EXAMPLE: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
}
