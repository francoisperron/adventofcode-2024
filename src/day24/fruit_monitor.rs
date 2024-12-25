use crate::day24::gate::Gate;
use crate::day24::operation::Operation;
use crate::day24::wires::Wires;
use itertools::Itertools;
use std::collections::BTreeSet;

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

    // https://en.wikipedia.org/wiki/Adder_(electronics)#Ripple-carry_adder
    pub fn swapped_wires(&self) -> String {
        let mut swapped = Swapped::default();

        let mut carry_output = self.gate_output("x00", "y00", Operation::And);

        let nb_bits = self.wires.wires.iter().filter(|(_, v)| v.is_some()).count() as u8 / 2;
        for bit in 1..nb_bits {
            let x = format!("x{bit:02}");
            let y = format!("y{bit:02}");
            let z = format!("z{bit:02}");

            let basic_add_output = self.gate_output(&x, &y, Operation::Xor);
            let add_gate = self.gate(carry_output, basic_add_output, Operation::Xor);
            swapped.try_insert_end(&add_gate.output, &z);
            swapped.try_insert(add_gate, basic_add_output);
            swapped.try_insert(add_gate, carry_output);

            let cascade_carry_gate = self.gate(basic_add_output, carry_output, Operation::And);
            swapped.try_insert(cascade_carry_gate, basic_add_output);
            swapped.try_insert(cascade_carry_gate, carry_output);

            let basic_carry_output = self.gate_output(&x, &y, Operation::And);
            let carry_gate = self.gate(&cascade_carry_gate.output, basic_carry_output, Operation::Or);
            swapped.try_insert(carry_gate, &cascade_carry_gate.output);
            swapped.try_insert(carry_gate, basic_carry_output);

            carry_output = &carry_gate.output;
        }

        swapped.print()
    }

    fn gate_output(&self, input_a: &str, input_b: &str, operation: Operation) -> &str {
        self.gates
            .iter()
            .find(|e| e.is_input(input_a) && e.is_input(input_b) && e.operation == operation)
            .unwrap()
            .output
            .as_str()
    }

    fn gate(&self, input_a: &str, input_b: &str, operation: Operation) -> &Gate {
        self.gates
            .iter()
            .find(|e| (e.is_input(input_a) || e.is_input(input_b)) && e.operation == operation)
            .unwrap()
    }
}

#[derive(Default)]
pub struct Swapped(BTreeSet<String>);

impl Swapped {
    pub fn try_insert(&mut self, gate: &Gate, output: &str) {
        if !gate.is_input(output) {
            self.0.insert(output.to_string());
        }
    }

    pub fn try_insert_end(&mut self, output: &str, end: &str) {
        if output != end {
            self.0.insert(end.to_string());
            self.0.insert(output.to_string());
        }
    }

    pub fn print(&self) -> String {
        self.0.iter().join(",")
    }
}

#[cfg(test)]
mod tests {
    use crate::day24::bit::Bit;
    use crate::day24::fruit_monitor::FruitMonitor;
    use crate::day24::gate::Gate;
    use crate::day24::operation::Operation;
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
