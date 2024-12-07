#[cfg(test)]
mod tests {
    use crate::toolbox::daily::daily_input;
    use crate::day04::word_search::WordSearch;

    #[test]
    fn solves_part1_example() {
        let word_search = WordSearch::from(EXAMPLE);

        assert_eq!(word_search.search_xmas(), 18);
    }
    
    #[test]
    fn solves_part1() {
        let input = daily_input(4);
        let word_search = WordSearch::from(&input);

        assert_eq!(word_search.search_xmas(), 2390);
    }
    
    #[test]
    fn solves_part2_example() {
        let word_search = WordSearch::from(EXAMPLE);
    
        assert_eq!(word_search.search_x_mas(), 9);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(4);
        let word_search = WordSearch::from(&input);

        assert_eq!(word_search.search_x_mas(), 1809);
    }

    const EXAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
}