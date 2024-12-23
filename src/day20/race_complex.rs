use num_complex::Complex;
use std::collections::HashMap;

pub struct RaceComplex {
    grid: HashMap<Complex<i32>, char>,
    start: Complex<i32>,
}

impl RaceComplex {
    pub fn from(input: &str) -> RaceComplex {
        let mut grid: HashMap<Complex<i32>, char> = HashMap::new();
        for (real, line) in input.lines().enumerate() {
            for (img, value) in line.chars().enumerate() {
                if value != '#' {
                    grid.insert(Complex::new(img as i32, real as i32), value);
                }
            }
        }

        let start = grid.iter().find(|(_, value)| **value == 'S').map(|(position, _)| *position).unwrap();

        RaceComplex { grid, start }
    }

    pub fn cheat(&self, nb_cheats: impl Fn(i32) -> bool) -> i32 {
        let distances = self.calculate_distances();
        self.count_cheats(&distances, nb_cheats)
    }

    fn calculate_distances(&self) -> HashMap<Complex<i32>, i32> {
        let mut distances: HashMap<Complex<i32>, i32> = HashMap::from([(self.start, 0)]);
        let mut to_visit: Vec<Complex<i32>> = vec![self.start];

        while let Some(pos) = to_visit.pop() {
            for around in [Complex::new(1, 0), Complex::new(-1, 0), Complex::new(0, 1), Complex::new(0, -1)] {
                let new_pos = pos + around;
                if self.grid.contains_key(&new_pos) && !distances.contains_key(&new_pos) {
                    distances.insert(new_pos, distances[&pos] + 1);
                    to_visit.push(new_pos);
                }
            }
        }
        distances
    }

    fn count_cheats(&self, distances: &HashMap<Complex<i32>, i32>, nb_cheats: impl Fn(i32) -> bool) -> i32 {
        let mut cheats = 0;
        for (p1, i) in distances {
            for (p2, j) in distances {
                let manhattan_distance = (p1 - p2).re.abs() + (p1 - p2).im.abs();
                if nb_cheats(manhattan_distance) && j - i - manhattan_distance >= 100 {
                    cheats += 1;
                }
            }
        }
        cheats
    }
}
