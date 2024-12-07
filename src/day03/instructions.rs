use regex::Regex;

pub struct Instructions {
    multiplications: Vec<(usize, usize, usize)>,
    dos: Vec<usize>,
    donts: Vec<usize>,
}

impl Instructions {
    pub fn from(input: &str) -> Instructions {
        let multiplications = Self::parse_multiplications(input);
        let dos = Self::parse_dos(input);
        let donts = Self::parse_donts(input);

        Instructions { multiplications, dos, donts }
    }

    fn parse_multiplications(input: &str) -> Vec<(usize, usize, usize)> {
        Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)")
            .unwrap()
            .captures_iter(input)
            .map(|caps| {
                let index = caps.iter().next().unwrap().unwrap().start();
                let a = caps["a"].parse().unwrap();
                let b = caps["b"].parse().unwrap();
                (index, a, b)
            })
            .collect()
    }

    fn parse_dos(input: &str) -> Vec<usize> {
        Regex::new(r"do\(\)")
            .unwrap()
            .captures_iter(input)
            .map(|caps| caps.iter().next().unwrap().unwrap().start())
            .collect()
    }

    fn parse_donts(input: &str) -> Vec<usize> {
        Regex::new(r"don't\(\)")
            .unwrap()
            .captures_iter(input)
            .map(|caps| caps.iter().next().unwrap().unwrap().start())
            .collect()
    }
}

impl Instructions {
    pub fn sum(&self) -> usize {
        self.multiplications.iter().map(|(_, a, b)| a * b).sum()
    }

    pub fn enabled_sum(&self) -> usize {
        self.multiplications
            .iter()
            .filter(|(index, _, _)| {
                let enabled = self.dos.iter().rev().find(|i| *i < index).unwrap_or(&0);
                let disabled = self.donts.iter().rev().find(|i| *i < index).unwrap_or(&0);
                enabled >= disabled
            })
            .map(|(_, a, b)| a * b)
            .sum()
    }
}
