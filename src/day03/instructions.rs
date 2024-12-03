use regex::Regex;

pub struct Instructions {
    multiplications: Vec<(usize, usize)>,
}

impl Instructions {
    pub fn from(input: &str) -> Instructions {
        let regex = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();
        
        let multiplications = regex
            .captures_iter(input)
            .map(|caps| (caps["a"].parse().unwrap(), caps["b"].parse().unwrap()))
            .collect();

        Instructions { multiplications }
    }
}

impl Instructions {
    pub fn sum(&self) -> usize {
        self.multiplications.iter().map(|(a, b)| a * b).sum()
    }
}
