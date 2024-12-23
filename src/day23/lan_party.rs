use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

pub struct LanParty {
    connections: HashMap<Computer, HashSet<Computer>>,
}

impl LanParty {
    pub fn from(input: &str) -> LanParty {
        let mut connections: HashMap<Computer, HashSet<Computer>> = HashMap::new();

        for line in input.lines() {
            for (c1_a, c1_b, _dash, c2_a, c2_b) in line.bytes().tuples() {
                connections.entry(Computer::new(c1_a, c1_b)).or_default().insert(Computer::new(c2_a, c2_b));
                connections.entry(Computer::new(c2_a, c2_b)).or_default().insert(Computer::new(c1_a, c1_b));
            }
        }

        Self { connections }
    }

    pub fn connected(&self) -> usize {
        let mut connected: HashSet<(&Computer, &Computer, &Computer)> = HashSet::new();
        for (c1, c1_connections) in self.connections.iter().filter(|(c, _)| c.maybe_chief()) {
            for c2 in c1_connections.iter() {
                for c3 in c1_connections.intersection(&self.connections[c2]) {
                    connected.insert([c1, c2, c3].into_iter().sorted().next_tuple().unwrap());
                }
            }
        }
        connected.len()
    }

    pub fn password(&self) -> String {
        self.connect_all()
            .iter()
            .map(|c| c.iter().sorted().join(","))
            .sorted_by(|a, b| a.len().cmp(&b.len()))
            .last()
            .unwrap()
    }

    fn connect_all(&self) -> Vec<HashSet<Computer>> {
        let mut all: Vec<HashSet<Computer>> = self.connections.keys().map(|&c| HashSet::from([c])).collect();
        for c1 in self.connections.keys() {
            for connections in all.iter_mut() {
                if connections.iter().all(|c2| self.connections[c1].contains(c2)) {
                    connections.insert(*c1);
                }
            }
        }
        all
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
pub struct Computer(u8, u8);

impl Computer {
    pub fn new(a: u8, b: u8) -> Computer {
        Computer(a, b)
    }

    pub fn maybe_chief(&self) -> bool {
        self.0 == b't'
    }
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0 as char, self.1 as char)
    }
}

#[cfg(test)]
mod tests {
    use crate::day23::lan_party::{Computer, LanParty};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn creates_bidirectional_connections() {
        let lan_party = LanParty::from("11-22");

        #[rustfmt::skip]
        assert_eq!(lan_party.connections, HashMap::from([
            (Computer::new(b'1', b'1'), HashSet::from([Computer::new(b'2', b'2')])),
            (Computer::new(b'2', b'2'), HashSet::from([Computer::new(b'1', b'1')])),
        ]));
    }

    #[test]
    fn creates_multiple_connections() {
        let lan_party = LanParty::from("11-22\n11-33");

        #[rustfmt::skip]
        assert_eq!(lan_party.connections, HashMap::from([
            (Computer::new(b'1', b'1'), HashSet::from([Computer::new(b'2', b'2'), Computer::new(b'3', b'3')])),
            (Computer::new(b'2', b'2'), HashSet::from([Computer::new(b'1', b'1')])),
            (Computer::new(b'3', b'3'), HashSet::from([Computer::new(b'1', b'1')])),
        ]));
    }

    #[test]
    fn connects_three() {
        let lan_party = LanParty::from("t1-t2\nt2-t3\nt1-t3");

        assert_eq!(lan_party.connected(), 1);
    }

    #[test]
    fn connects_only_chief_computer() {
        let lan_party = LanParty::from("11-22\n22-33\n11-33");

        assert_eq!(lan_party.connected(), 0);
    }

    #[test]
    fn connects_all_computers() {
        let lan_party = LanParty::from("11-22\n11-33\n22-33");

        #[rustfmt::skip]
        assert_eq!(lan_party.connect_all(), vec![
            HashSet::from([Computer::new(b'1', b'1'), Computer::new(b'2', b'2'), Computer::new(b'3', b'3')]),
            HashSet::from([Computer::new(b'2', b'2'), Computer::new(b'3', b'3'), Computer::new(b'1', b'1')]),
            HashSet::from([Computer::new(b'3', b'3'), Computer::new(b'2', b'2'), Computer::new(b'1', b'1')]),
        ]);
    }
}
