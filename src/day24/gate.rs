use crate::day24::operation::Operation;
use crate::day24::wires::Wires;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Gate {
    pub input_a: String,
    pub input_b: String,
    pub output: String,
    pub operation: Operation,
}

impl Gate {
    pub fn from(input: &str) -> Gate {
        let (input_a, operation, input_b, _arrow, output) = input.split(" ").map(|i| i.to_string()).collect_tuple().unwrap();
        Gate { input_a, input_b, output, operation: Operation::from(operation.as_str()) }
    }

    pub fn is_input(&self, input: &str) -> bool {
        self.input_a == input || self.input_b == input
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
