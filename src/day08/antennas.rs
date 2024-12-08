use crate::toolbox::{Grid, Position};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Antennas {
    grid: Grid,
    antennas: HashMap<char, HashSet<Position>>,
    antinodes: HashSet<Position>,
}

impl Antennas {
    pub fn from(input: &str) -> Antennas {
        let grid = Grid::from(input);
        let mut antennas = grid.group_by_value();
        antennas.remove(&'.');

        Antennas { grid, antennas, antinodes: HashSet::new() }
    }

    pub fn antinodes_count(&mut self) -> usize {
        self.find_antinodes(false)
    }

    pub fn resonant_antinodes_count(&mut self) -> usize {
        self.find_antinodes(true)
    }

    fn find_antinodes(&mut self, resonant: bool) -> usize {
        let antennas_positions: Vec<_> = self.antennas.values().cloned().collect();
        for positions in antennas_positions {
            for (&p1, &p2) in positions.iter().tuple_combinations() {
                let diff = p1 - p2;
                self.add_antinodes(p1, diff, resonant);
                self.add_antinodes(p2, -diff, resonant);
            }
        }

        self.count_antinodes(resonant)
    }

    fn add_antinodes(&mut self, position: Position, diff: Position, resonant: bool) {
        let mut multiplier = 1;
        loop {
            let antinode = position + diff * multiplier;
            if self.grid.is_inbound(&antinode) {
                self.antinodes.insert(antinode);
                if !resonant {
                    break;
                }
            } else {
                break;
            }
            multiplier += 1;
        }
    }

    fn count_antinodes(&mut self, resonant: bool) -> usize {
        if resonant {
            let antennas_positions = self.antennas.values().flatten().collect::<Vec<&Position>>();
            let antinodes_except_antennas = self.antinodes.iter().filter(|p| !antennas_positions.contains(p)).count();
            antinodes_except_antennas + antennas_positions.len()
        } else {
            self.antinodes.len()
        }
    }
}
