use crate::day24::bit::Bit;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Wires {
    pub wires: HashMap<String, Option<Bit>>,
}

impl Wires {
    pub fn new() -> Wires {
        Wires { wires: HashMap::new() }
    }

    pub fn from(&mut self, input: &str) {
        for line in input.lines() {
            let (wire, value) = line.split_once(": ").unwrap();
            let value = Bit::from(value);
            self.wires.insert(wire.to_string(), value);
        }
    }

    pub fn set(&mut self, wire: &str, value: Option<Bit>) {
        self.wires.insert(wire.to_string(), value);
    }

    pub fn get(&self, wire: &str) -> Option<Bit> {
        self.wires.get(wire).copied().unwrap_or(None)
    }

    pub fn value_of(&self, wire: &str) -> Option<usize> {
        let values: Vec<Option<Bit>> = self
            .wires
            .iter()
            .filter(|(w, _)| w.starts_with(wire))
            .sorted_by_key(|(w, _)| *w)
            .rev()
            .map(|(_, v)| *v)
            .collect();

        if values.iter().any(|v| v.is_none()) {
            None
        } else {
            let bits: String = values.iter().map(|v| v.unwrap().value()).join("");
            let value = usize::from_str_radix(&bits, 2).unwrap();
            Some(value)
        }
    }
}
