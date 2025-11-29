use itertools::{Itertools, iterate};
use std::collections::HashMap;

pub struct Secrets {
    secrets: Vec<Secret>,
}

impl Secrets {
    pub fn from(input: &str) -> Secrets {
        Secrets { secrets: input.lines().map(Secret::from).collect() }
    }

    pub fn secrets_sum(&self, n: usize) -> usize {
        self.secrets.iter().map(|secret| secret.nth(n).value).sum()
    }

    pub fn bananas_max(&self, n: usize) -> isize {
        self.secrets
            .iter()
            .map(|secret| secret.next_changes(n + 1))
            .fold(HashMap::new(), Self::combine)
            .into_values()
            .max()
            .unwrap()
    }

    fn combine(mut combined_changes: HashMap<(isize, isize, isize, isize), isize>, changes: HashMap<(isize, isize, isize, isize), isize>) -> HashMap<(isize, isize, isize, isize), isize> {
        changes.into_iter().for_each(|(k, v)| *combined_changes.entry(k).or_insert(0) += v);
        combined_changes
    }
}

#[derive(Clone)]
pub struct Secret {
    value: usize,
}

impl Secret {
    pub fn new(value: usize) -> Secret {
        Secret { value }
    }

    pub fn from(input: &str) -> Secret {
        Secret::new(input.parse().unwrap())
    }

    pub fn nth(&self, n: usize) -> Secret {
        self.next_n(n).into_iter().last().unwrap()
    }

    pub fn next_changes(&self, n: usize) -> HashMap<(isize, isize, isize, isize), isize> {
        self.next_n(n)
            .into_iter()
            .map(|s| s.digit())
            .rev()
            .tuple_windows::<(_, _, _, _, _)>()
            .map(|(v1, v2, v3, v4, v5)| ((v4 - v5, v3 - v4, v2 - v3, v1 - v2), v1))
            .collect::<HashMap<_, _>>()
    }

    fn next_n(&self, n: usize) -> Vec<Secret> {
        iterate(self.clone(), |s| s.next()).take(n + 1).collect()
    }

    fn next(&self) -> Secret {
        let mut secret = (self.value ^ (self.value * 64)) % 16777216;
        secret = (secret ^ (secret / 32)) % 16777216;
        secret = (secret ^ (secret * 2048)) % 16777216;
        Secret::new(secret)
    }

    fn digit(&self) -> isize {
        (self.value % 10) as isize
    }
}

#[cfg(test)]
mod tests {
    use crate::day22::secrets::Secret;
    use std::collections::HashMap;

    #[test]
    fn generates_secret_next_value() {
        assert_eq!(Secret::new(123).next().value, 15887950);
        assert_eq!(Secret::new(15887950).next().value, 16495136);
        assert_eq!(Secret::new(16495136).next().value, 527345);
        assert_eq!(Secret::new(527345).next().value, 704524);
        assert_eq!(Secret::new(704524).next().value, 1553684);
        assert_eq!(Secret::new(1553684).next().value, 12683156);
        assert_eq!(Secret::new(12683156).next().value, 11100544);
        assert_eq!(Secret::new(11100544).next().value, 12249484);
        assert_eq!(Secret::new(12249484).next().value, 7753432);
        assert_eq!(Secret::new(7753432).next().value, 5908254);
    }

    #[test]
    fn generates_secret_nth_value() {
        assert_eq!(Secret::new(123).nth(10).value, 5908254);
    }

    #[test]
    fn generates_secret_changes() {
        let secret = Secret::new(123);

        #[rustfmt::skip]
        let sequences = HashMap::from([
            ((2, -2, 0, -2), 2), 
            ((-3, 6, -1, -1), 4), 
            ((6, -1, -1, 0), 4), 
            ((-1, -1, 0, 2), 6),
            ((-2, 0, -2, 2), 4), 
            ((0, 2, -2, 0), 4), 
            ((-1, 0, 2, -2), 4)
        ]);

        assert_eq!(secret.next_changes(10), sequences);
    }
}
