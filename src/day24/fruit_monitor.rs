use itertools::Itertools;
use std::collections::HashMap;

pub struct FruitMonitor {
    wires: Wires,
    gates: Vec<Gate>,
}

impl FruitMonitor {
    pub fn from(input: &str) -> FruitMonitor {
        let (wires_input, gates_input) = input.split_once("\n\n").unwrap();
        let mut wires = Wires::new();

        let gates: Vec<Gate> = gates_input.lines().map(Gate::from).collect();
        for gate in gates.iter() {
            wires.set(&gate.input_a, None);
            wires.set(&gate.input_b, None);
            wires.set(&gate.output, None);
        }

        wires.from(wires_input);

        FruitMonitor { wires, gates }
    }

    pub fn wires_z_output(&mut self) -> usize {
        while self.wires.value_of("z").is_none() {
            for gate in self.gates.iter() {
                gate.output(&mut self.wires);
            }
        }
        self.wires.value_of("z").unwrap()
    }
}

pub struct Wires {
    wires: HashMap<String, Option<Bit>>,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Bit(bool);

impl Bit {
    pub fn from(input: &str) -> Option<Bit> {
        match input {
            "1" => Some(Bit(true)),
            "0" => Some(Bit(false)),
            _ => None,
        }
    }

    pub fn value(&self) -> char {
        match self {
            Bit(true) => '1',
            Bit(false) => '0',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Gate {
    pub input_a: String,
    input_b: String,
    output: String,
    operation: Operation,
}

impl Gate {
    pub fn from(input: &str) -> Gate {
        let (input_a, operation, input_b, _arrow, output) = input.split(" ").map(|i| i.to_string()).collect_tuple().unwrap();
        Gate { input_a, input_b, output, operation: Operation::from(operation.as_str()) }
    }

    pub fn output(&self, wires: &mut Wires) {
        let input_a = wires.get(&self.input_a);
        let input_b = wires.get(&self.input_b);
        if let (Some(a), Some(b)) = (input_a, input_b) {
            let output = self.operation.evaluate(a, b);
            wires.set(&self.output, Some(output));
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    pub fn from(input: &str) -> Operation {
        match input {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => unreachable!(),
        }
    }
    
    pub fn evaluate(&self, a: Bit, b: Bit) -> Bit {
        match self {
            Operation::And => Bit(a.0 & b.0),
            Operation::Or => Bit(a.0 | b.0),
            Operation::Xor => Bit(a.0 ^ b.0),
        }
    }
}

    

#[cfg(test)]
mod tests {
    use crate::day24::fruit_monitor::{Bit, FruitMonitor, Gate, Operation};
    use crate::day24::tests::EXAMPLE;
    use std::collections::HashMap;

    #[test]
    fn parses_wires() {
        let monitor = FruitMonitor::from(EXAMPLE);

        #[rustfmt::skip]
        assert_eq!(monitor.wires.wires, HashMap::from([
            ("x00".to_string(), Some(Bit(true))),
            ("x01".to_string(), Some(Bit(true))),
            ("x02".to_string(), Some(Bit(true))),
            
            ("y00".to_string(), Some(Bit(false))),
            ("y01".to_string(), Some(Bit(true))),
            ("y02".to_string(), Some(Bit(false))),

            ("z00".to_string(), None),
            ("z01".to_string(), None),
            ("z02".to_string(), None),
        ]));
    }

    #[test]
    fn parses_gates() {
        let monitor = FruitMonitor::from(EXAMPLE);

        #[rustfmt::skip]
        assert_eq!(monitor.gates, vec![
            Gate { input_a: "x00".to_string(), input_b: "y00".to_string(), output: "z00".to_string(), operation: Operation::And },
            Gate { input_a: "x01".to_string(), input_b: "y01".to_string(), output: "z01".to_string(), operation: Operation::Xor },
            Gate { input_a: "x02".to_string(), input_b: "y02".to_string(), output: "z02".to_string(), operation: Operation::Or },
        ]);
    }

    #[test]
    fn gives_value_of_a_wire() {
        let monitor = FruitMonitor::from(EXAMPLE);

        let x_wire = monitor.wires.value_of("x");
        assert_eq!(x_wire, Some(7));

        let y_wire = monitor.wires.value_of("y");
        assert_eq!(y_wire, Some(2));

        let z_wire = monitor.wires.value_of("z");
        assert_eq!(z_wire, None);
    }
}
