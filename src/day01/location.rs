use itertools::Itertools;

pub struct Locations {
    pub left: Vec<usize>,
    pub right: Vec<usize>,
}

impl Locations {
    pub fn from(input: &str) -> Locations {
        let left: Vec<usize> = input
            .lines()
            .map(|line| line.split_whitespace().next().unwrap().parse().unwrap())
            .sorted()
            .collect();

        let right: Vec<usize> = input
            .lines()
            .map(|line| line.split_whitespace().nth(1).unwrap().parse().unwrap())
            .sorted()
            .collect();

        Locations { left, right }
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
}
