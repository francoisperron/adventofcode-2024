use itertools::Itertools;

pub struct Locations {
    pub left: Vec<usize>,
    pub right: Vec<usize>,
}

impl Locations {
    pub fn from(input: &str) -> Locations {
        let left = Locations::parse_column(input, 0);
        let right = Locations::parse_column(input, 1);

        Locations { left, right }
    }

    fn parse_column(input: &str, index: usize) -> Vec<usize> {
        input
            .lines()
            .map(|line| line.split_whitespace().nth(index).unwrap().parse().unwrap())
            .sorted()
            .collect()
    }
}

impl Locations {
    pub fn total_distance(&self) -> usize {
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum()
    }

    pub fn similarity_score(&self) -> usize {
        self.left
            .iter()
            .map(|l| self.right.iter().filter(|r| *r == l).count() * l)
            .sum()
    }
}
