pub struct Program {
    a: usize,
    b: usize,
    c: usize,
    instructions: Vec<usize>,
}

impl Program {
    pub fn from(input: &str) -> Program {
        let mut lines = input.lines();

        let a = lines.next().unwrap()[12..].parse::<usize>().unwrap();
        let b = lines.next().unwrap()[12..].parse::<usize>().unwrap();
        let c = lines.next().unwrap()[12..].parse::<usize>().unwrap();
        lines.next();
        let instructions = lines.next().unwrap()[9..].split(',').map(|i| i.parse::<usize>().unwrap()).collect();

        Program { a, b, c, instructions }
    }

    pub fn run(&mut self) -> Vec<usize> {
        let mut pointer = 0;
        let mut output = Vec::new();

        while pointer < self.instructions.len() {
            let opcode = self.instructions[pointer];
            let operand = self.instructions[pointer + 1];

            let combo = match operand {
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => operand,
            };

            match opcode {
                0 => self.a >>= combo,
                1 => self.b ^= operand,
                2 => self.b = combo % 8,
                3 => {
                    if self.a != 0 {
                        pointer = operand;
                        continue;
                    }
                }
                4 => self.b ^= self.c,
                5 => output.push(combo % 8),
                6 => self.b = self.a >> combo,
                7 => self.c = self.a >> combo,
                _ => panic!(),
            };

            pointer += 2;
        }
        output
    }

    pub fn copy(&mut self) -> usize {
        let mut factors = vec![0; self.instructions.len()];

        loop {
            let a = factors.iter().enumerate().map(|(i, &f)| 8usize.pow(i as u32) * f).sum();
            self.a = a;
            let output = self.run();

            if output == self.instructions {
                return a;
            }

            for i in (0..self.instructions.len()).rev() {
                if output.len() < i || output[i] != self.instructions[i] {
                    factors[i] += 1;
                    factors[..i].fill(0);
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day17::program::Program;

    #[test]
    fn adv_0() {
        let mut program = Program { a: 5, b: 0, c: 0, instructions: vec![0, 1] };
        program.run();

        assert_eq!(program.a, 5 >> 1);
    }

    #[test]
    fn bxl_1() {
        let mut program = Program { a: 0, b: 5, c: 0, instructions: vec![1, 1] };
        program.run();

        assert_eq!(program.b, 5 ^ 1);
    }

    #[test]
    fn bst_2() {
        let mut program = Program { a: 0, b: 0, c: 0, instructions: vec![2, 17] };
        program.run();

        assert_eq!(program.b, 17 % 8);
    }

    #[test]
    fn jnz_3_with_a_0() {
        let mut program = Program { a: 0, b: 0, c: 0, instructions: vec![3, 0] };
        program.run();

        assert_eq!(program.a, 0);
        assert_eq!(program.b, 0);
        assert_eq!(program.c, 0);
    }

    #[test]
    fn jnz_3_with_a_not_0() {
        let mut program = Program { a: 5, b: 0, c: 0, instructions: vec![3, 4, 7, 7, 0, 1] };
        program.run();

        assert_eq!(program.a, 5 >> 1);
    }

    #[test]
    fn bxc_4() {
        let mut program = Program { a: 0, b: 8, c: 9, instructions: vec![4, 7] };
        program.run();

        assert_eq!(program.b, 8 ^ 9);
    }

    #[test]
    fn out_5() {
        let mut program = Program { a: 0, b: 0, c: 0, instructions: vec![5, 17] };
        let output = program.run();

        assert_eq!(output, vec![17 % 8]);
    }

    #[test]
    fn bdv_6() {
        let mut program = Program { a: 5, b: 0, c: 0, instructions: vec![6, 1] };
        program.run();

        assert_eq!(program.b, 5 >> 1);
    }

    #[test]
    fn cdv_7() {
        let mut program = Program { a: 5, b: 0, c: 0, instructions: vec![7, 1] };
        program.run();

        assert_eq!(program.c, 5 >> 1);
    }

    #[test]
    fn outputs_copy_of_input_when_a_is_ok() {
        let mut program = Program { a: 117440, b: 0, c: 0, instructions: vec![0, 3, 5, 4, 3, 0] };

        assert_eq!(program.run(), vec![0, 3, 5, 4, 3, 0]);
    }
}
