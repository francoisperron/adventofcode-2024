use std::collections::HashMap;

pub struct Towels {
    towels: Vec<String>,
    designs: Vec<String>,
}

impl Towels {
    pub fn from(input: &str) -> Towels {
        let (towels, patterns) = input.split_once("\n\n").unwrap();
        let towels = towels.split(", ").map(|t| t.to_string()).collect();
        let designs = patterns.lines().map(|line| line.to_string()).collect();
        Towels { towels, designs }
    }

    pub fn possible_designs(&self) -> usize {
        self.designs.iter().filter(|design| self.possible_ways(design, &mut Cache::default()) > 0).count()
    }

    pub fn all_possible_designs(&self) -> usize {
        self.designs.iter().map(|design| self.possible_ways(design, &mut Cache::default())).sum()
    }

    fn possible_ways(&self, design: &str, cache: &mut Cache) -> usize {
        if design.is_empty() {
            return 1;
        }

        if let Some(&ways) = cache.get(design) {
            return ways;
        }

        let mut ways = 0;
        for towel in self.towels.iter() {
            if design.starts_with(towel) {
                let remaining_design = &design[towel.len()..];
                ways += self.possible_ways(remaining_design, cache);
            }
        }

        cache.put(design, ways);
        ways
    }
}

#[derive(Default)]
struct Cache(HashMap<String, usize>);

impl Cache {
    pub fn put(&mut self, pattern: &str, ways: usize) {
        self.0.insert(pattern.to_string(), ways);
    }

    pub fn get(&self, pattern: &str) -> Option<&usize> {
        self.0.get(pattern)
    }
}
