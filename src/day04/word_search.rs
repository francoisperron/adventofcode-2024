use crate::day04::grid::Grid;
use crate::day04::position::Position;

pub struct WordSearch {
    grid: Grid,
}

impl WordSearch {
    pub fn from(input: &str) -> WordSearch {
        let grid = Grid::from(input);
        WordSearch { grid }
    }
}

impl WordSearch {
    pub fn search_xmas(&self) -> usize {
        self.grid.elements
            .keys()
            .map(|&position| {
                Position::around()
                    .iter()
                    .filter(|&&delta| {
                        "XMAS".chars()
                            .enumerate()
                            .all(|(step, value)| self.grid.element_at(&(position + (delta * step))) == &value)
                    })
                    .count()
            })
            .sum()
    }

    pub fn search_x_mas(&self) -> usize {
        self.grid.elements
            .iter()
            .filter(|(&position, element)| {
                let ul = self.grid.element_at(&(position + Position::new(-1, -1)));
                let lr = self.grid.element_at(&(position + Position::new(1, 1)));
                let middle = *element;
                let ur = self.grid.element_at(&(position + Position::new(-1, 1)));
                let ll = self.grid.element_at(&(position + Position::new(1, -1)));
                
                matches!(&[ul, ur, middle, ll, lr], &['M', 'M', 'A', 'S', 'S'] | &['M', 'S', 'A', 'M', 'S'] | &['S', 'M', 'A', 'S', 'M'] | &['S', 'S', 'A', 'M', 'M'])
            })
            .count()
    }
}