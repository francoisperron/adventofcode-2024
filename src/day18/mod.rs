mod memory;

#[cfg(test)]
mod tests {
    use crate::day18::memory::Memory;
    use crate::toolbox::daily::daily_input;
    use crate::toolbox::Position;

    #[test]
    fn solves_part1_example() {
        let mut memory = Memory::from(7, 7, EXAMPLE);
        (0..12).for_each(|byte| memory.corrupt(byte));

        assert_eq!(memory.reach_exit(), 22);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(18);
        let mut memory = Memory::from(71, 71, &input);
        (0..1024).for_each(|byte| memory.corrupt(byte));

        assert_eq!(memory.reach_exit(), 272);
    }

    #[test]
    fn solves_part2_example() {
        let mut memory = Memory::from(7, 7, EXAMPLE);

        assert_eq!(memory.last_position(), Position::new(6, 1));
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(18);
        let mut memory = Memory::from(71, 71, &input);

        assert_eq!(memory.last_position(), Position::new(16, 44));
    }

    const EXAMPLE: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
}
