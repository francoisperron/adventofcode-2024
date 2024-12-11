use crate::toolbox::{Grid, Position};
use std::collections::HashSet;

pub struct TopographicMap {
    grid: Grid,
}

impl TopographicMap {
    pub fn from(input: &str) -> Self {
        Self { grid: Grid::from(input) }
    }

    pub fn trailheads_score(&self) -> usize {
        let trailheads = self.grid.elements.iter().filter(|(_, &v)| v == '0').map(|(&p, _)| p).collect::<Vec<Position>>();

        trailheads
            .iter()
            .map(|p| {
                let mut hiked: HashSet<Position> = HashSet::new();
                self.hike(p, &mut hiked, false)
            })
            .sum()
    }

    pub fn hike(&self, current: &Position, hiked: &mut HashSet<Position>, rating: bool) -> usize {
        let current_value = self.grid.element_at(current).to_digit(10).unwrap();
        if current_value == 9 {
            return if hiked.insert(*current) { 1 } else if rating {1} else {0};
        }

        current
            .around_me_4()
            .iter()
            .filter(|p| self.grid.is_inbound(p))
            .filter(|p| self.grid.element_at(p).to_digit(10).unwrap() == current_value + 1)
            .map(|p| self.hike(p, hiked, rating))
            .sum()
    }
    
    pub fn trailheads_ratings(&self) -> usize {
        let trailheads = self.grid.elements.iter().filter(|(_, &v)| v == '0').map(|(&p, _)| p).collect::<Vec<Position>>();

        trailheads
            .iter()
            .map(|p| {
                let mut hiked: HashSet<Position> = HashSet::new();
                self.hike(p, &mut hiked, true)
            })
            .sum()
    }
}
