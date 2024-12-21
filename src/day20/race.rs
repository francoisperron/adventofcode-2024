use crate::toolbox::{Direction, Grid, Position};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct Race {
    pub grid: Grid,
    pub start: Position,
    pub end: Position,
}

impl Race {
    pub fn from(input: &str) -> Race {
        let grid = Grid::from(input);
        let start = grid.position_of('S').unwrap();
        let end = grid.position_of('E').unwrap();
        Race { grid, start, end }
    }

    pub fn cheat(&mut self, position: Position) {
        self.grid.set_element_at(&position, '.')
    }

    pub fn remove_cheat(&mut self, position: Position) {
        self.grid.set_element_at(&position, '#')
    }

    pub fn cheats_over(&mut self, picoseconds: usize) -> usize {
        let time_to_beat = self.reach_exit();
        let mut cheats = Vec::new();

        let walls = self.grid.positions_of('#').collect::<Vec<Position>>();
        for wall in walls {
            self.cheat(wall);
            let cheat_time = self.reach_exit();
            if cheat_time < time_to_beat {
                cheats.push(time_to_beat - cheat_time);
            }
            self.remove_cheat(wall);
        }

        cheats.iter().filter(|c| **c >= picoseconds).count()
    }

    pub fn reach_exit(&self) -> usize {
        let mut to_visit = BinaryHeap::new();
        to_visit.push(Reverse((0, self.start)));

        let mut costs = HashMap::new();
        costs.insert(self.start, 0);

        while let Some(Reverse((cost, position))) = to_visit.pop() {
            if position == self.end {
                return cost;
            }

            for new_dir in Direction::all() {
                let new_pos = position.move_towards(&new_dir);
                if !self.grid.is_inbound(&new_pos) || self.grid.element_at(&new_pos) == &'#' {
                    continue;
                }

                let new_cost = cost + 1;

                if new_cost < *costs.get(&new_pos).unwrap_or(&usize::MAX) {
                    costs.insert(new_pos, new_cost);
                    to_visit.push(Reverse((new_cost, new_pos)));
                }
            }
        }

        usize::MAX
    }
}

#[cfg(test)]
mod tests {
    use crate::day20::race::Race;
    use crate::toolbox::Position;

    #[test]
    fn runs_race() {
        let race = Race::from(EXAMPLE);
        let picoseconds = race.reach_exit();

        assert_eq!(picoseconds, 84);
    }

    #[test]
    fn runs_race_1_cheat() {
        let mut race = Race::from(EXAMPLE);
        race.cheat(Position::new(8, 1));
        let picoseconds = race.reach_exit();

        assert_eq!(picoseconds, 84 - 12);
    }

    #[test]
    fn runs_race_2_cheat() {
        let mut race = Race::from(EXAMPLE);
        race.cheat(Position::new(10, 7));
        let picoseconds = race.reach_exit();

        assert_eq!(picoseconds, 84 - 20);
    }

    #[test]
    fn runs_race_best_cheat() {
        let mut race = Race::from(EXAMPLE);
        race.cheat(Position::new(6, 7));
        let picoseconds = race.reach_exit();

        assert_eq!(picoseconds, 84 - 64);
    }

    const EXAMPLE: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
}
