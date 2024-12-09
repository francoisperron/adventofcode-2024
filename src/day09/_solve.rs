#[cfg(test)]
mod tests {
    use crate::day09::disk::Disk;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        let disk = Disk::from_blocks(EXAMPLE);

        assert_eq!(disk.defrag().checksum(), 1928);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(9);
        let disk = Disk::from_blocks(&input);

        assert_eq!(disk.defrag().checksum(), 6301895872542);
    }
    
    #[test]
    fn solves_part2_example() {
        let disk = Disk::from_files(EXAMPLE);
    
        assert_eq!(disk.defrag().checksum(), 2858);
    }
    
    #[test]
    fn solves_part2() {
        let input = daily_input(9);
        let disk = Disk::from_files(&input);
    
        assert_eq!(disk.defrag().checksum(), 6323761685944);
    }
    
    const EXAMPLE: &str = "2333133121414131402";
}