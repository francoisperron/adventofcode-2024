use crate::toolbox::{Grid, Position};
use std::collections::HashMap;

pub struct TopographicMap {
    grid: Grid,
    trailheads: Vec<Position>,
}

impl TopographicMap {
    pub fn from(input: &str) -> Self {
        let grid = Grid::from(input);
        let trailheads = grid.elements.iter().filter(|&(_, &v)| v == '0').map(|(&p, _)| p).collect::<Vec<Position>>();
        Self { grid, trailheads }
    }

    pub fn trailheads_score(&self) -> usize {
        self.trailheads().count()
    }

    pub fn trailheads_ratings(&self) -> usize {
        self.trailheads().map(|(_, v)| v).sum()
    }

    pub fn trailheads(&self) -> impl Iterator<Item = (Position, usize)> + '_ {
        self.trailheads.iter().flat_map(|p| {
            let mut hiked: HashMap<Position, usize> = HashMap::new();
            self.hike(p, &mut hiked);
            hiked.into_iter()
        })
    }

    pub fn hike(&self, current: &Position, hiked: &mut HashMap<Position, usize>) {
        let current_value = self.grid.element_at(current).to_digit(10).unwrap();
        if current_value == 9 {
            hiked.insert(*current, hiked.get(current).unwrap_or(&0) + 1);
            return;
        }

        current
            .around_me_4()
            .filter(|p| self.grid.is_inbound(p))
            .filter(|p| self.grid.element_at(p).to_digit(10).unwrap() == current_value + 1)
            .for_each(|p| self.hike(&p, hiked));
    }
}
