use crate::day12::price::Price;
use crate::day12::region::Region;
use crate::toolbox::{Grid, Position};
use std::collections::HashSet;

pub struct Garden {
    pub grid: Grid,
}

impl Garden {
    pub fn from(input: &str) -> Garden {
        Garden { grid: Grid::from(input) }
    }

    pub fn fences_price(&self) -> Price {
        let mut visited = HashSet::new();

        self.grid
            .elements
            .iter()
            .map(|(position, plant)| {
                let mut region = Region::default();
                self.visit(position, plant, &mut visited, &mut region);
                region.price()
            })
            .sum()
    }

    fn visit(&self, current_position: &Position, current_plant: &char, visited: &mut HashSet<Position>, region: &mut Region) {
        if visited.contains(current_position) || self.grid.element_at(current_position) != current_plant {
            return;
        }

        visited.insert(*current_position);
        region.add(current_position, current_plant, &self.grid);

        current_position
            .around_me_4()
            .filter(|p| self.grid.is_inbound(p))
            .for_each(|a| self.visit(&a, current_plant, visited, region));
    }
}
