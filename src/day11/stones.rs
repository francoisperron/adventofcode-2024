use std::collections::HashMap;

pub struct Stones {
    stones: HashMap<Stone, usize>,
}

impl Stones {
    pub fn from(input: &str) -> Stones {
        Stones { stones: input.split_whitespace().map(|i| (Stone::from(i), 1)).collect::<HashMap<Stone, usize>>() }
    }

    pub fn blink(&self, times: usize) -> Stones {
        let mut stones = self.stones.clone();

        for _ in 0..times {
            let mut new_stones: HashMap<Stone, usize> = HashMap::new();
            stones
                .iter()
                .for_each(|(stone, count)| stone.blink().for_each(|s| *new_stones.entry(s).or_default() += count));
            stones = new_stones;
        }

        Stones { stones }
    }

    pub fn count(&self) -> usize {
        self.stones.values().sum()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Stone {
    number: usize,
}

impl Stone {
    pub fn from(input: &str) -> Stone {
        Stone::new(input.parse().unwrap())
    }

    pub fn new(number: usize) -> Stone {
        Stone { number }
    }

    pub fn blink(&self) -> impl Iterator<Item = Stone> + '_ {
        let number_as_string = self.number.to_string();

        match self.number {
            0 => vec![Stone::new(1)],
            _n if number_as_string.len() % 2 == 0 => {
                let (high, low) = number_as_string.split_at(number_as_string.len() / 2);
                vec![Stone::new(high.parse().unwrap()), Stone::new(low.parse().unwrap())]
            }
            _ => vec![Stone::new(self.number * 2024)],
        }
        .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::stones::Stone;

    #[test]
    fn zero_replaced_by_one() {
        let stone = Stone::new(0);

        assert_eq!(stone.blink().collect::<Vec<Stone>>(), vec![Stone::new(1)]);
    }

    #[test]
    fn even_number_of_digits_splits() {
        let stone = Stone::new(10);

        assert_eq!(stone.blink().collect::<Vec<Stone>>(), vec![Stone::new(1), Stone::new(0)]);
    }

    #[test]
    fn otherwise_multiply_by_2024() {
        let stone = Stone::new(1);

        assert_eq!(stone.blink().collect::<Vec<Stone>>(), vec![Stone::new(2024)]);
    }
}
