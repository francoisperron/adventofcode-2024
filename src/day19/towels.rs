use regex::Regex;

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
        let towels_regex = Regex::new(&format!("^({})*$", self.towels.join("|"))).unwrap();
        self.designs.iter().filter(|design| towels_regex.is_match(design)).count()
    }
}
