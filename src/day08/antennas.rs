use crate::toolbox::{Grid, Position};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Antennas {
    grid: Grid,
}

impl Antennas {
    pub fn from(input: &str) -> Antennas {
        let grid = Grid::from(input);
        Antennas { grid }
    }

    pub fn antinodes_count(&self) -> usize {
        self.find_antinodes(false)
    }

    pub fn resonant_antinodes_count(&self) -> usize {
        self.find_antinodes(true)
    }

    fn find_antinodes(&self, resonant: bool) -> usize {
        let mut antennas: HashMap<char, HashSet<Position>> = self.grid.group_by_value();
        antennas.remove(&'.');

        let mut antinodes: HashSet<Position> = HashSet::new();
        for positions in antennas.values() {
            for (&p1, &p2) in positions.iter().tuple_combinations() {
                let diff = p1 - p2;
                antinodes.extend(&self.create_antinodes(p1, diff, resonant));
                antinodes.extend(&self.create_antinodes(p2, -diff, resonant));
            }
        }

        antinodes.len()
    }

    fn create_antinodes(&self, position: Position, diff: Position, resonant: bool) -> HashSet<Position> {
        let mut antinodes = HashSet::new();
        let mut multiplier = if resonant { 0 } else { 1 };
        loop {
            let antinode = position + diff * multiplier;
            if self.grid.is_inbound(&antinode) {
                antinodes.insert(antinode);
                if !resonant {
                    return antinodes;
                }
            } else {
                return antinodes;
            }
            multiplier += 1;
        }
    }
}
