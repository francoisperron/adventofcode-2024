pub struct Schematics {
    pub locks: Vec<[usize; 5]>,
    pub keys: Vec<[usize; 5]>,
}

impl Schematics {
    pub fn from(input: &str) -> Schematics {
        let (locks, keys) = input.split("\n\n").partition::<Vec<_>, _>(|i| i.lines().next().unwrap() == "#####");
        let locks = locks.iter().map(|i| Self::parse(i.lines().skip(1))).collect();
        let keys = keys.iter().map(|i| Self::parse(i.lines().rev().skip(1))).collect();

        Schematics { locks, keys }
    }

    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> [usize; 5] {
        let mut pin_heights = [0; 5];
        for line in lines {
            for (i, v) in line.chars().enumerate() {
                if v == '#' {
                    pin_heights[i] += 1;
                }
            }
        }
        pin_heights
    }

    pub fn overlapping(&self) -> usize {
        let mut overlaps = 0;
        for lock in self.locks.iter() {
            for key in self.keys.iter() {
                if (0..5).all(|i| lock[i] + key[i] < 6) {
                    overlaps += 1;
                }
            }
        }
        overlaps
    }
}
