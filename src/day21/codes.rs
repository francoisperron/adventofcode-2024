use crate::day21::pads::{DIRECTIONAL_PAD, NUMERIC_PAD, Pad};
use crate::day21::robot_command::RobotCommand;
use crate::toolbox::{Direction, Position};
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct Codes {
    codes: Vec<Code>,
    nb_pads: usize,
}

impl Codes {
    pub fn from(input: &str, nb_pads: usize) -> Codes {
        Codes { codes: input.lines().map(Code::from).collect(), nb_pads }
    }

    pub fn complexities(&self) -> usize {
        let mut cache = Cache::default();
        self.codes
            .iter()
            .map(|code| code.numeric_part() * code.numeric_pad_cost(&mut cache, self.nb_pads))
            .sum()
    }
}

pub struct Code {
    code: Vec<char>,
}

impl Code {
    pub fn from(input: &str) -> Code {
        Code { code: input.chars().collect() }
    }

    pub fn numeric_part(&self) -> usize {
        self.code.iter().take(3).join("").parse().unwrap()
    }

    fn numeric_pad_cost(&self, cache: &mut Cache, nb_pads: usize) -> usize {
        let mut costs = HashMap::new();
        let mut queue = BinaryHeap::from([Reverse((0, Position::new(2, 3), RobotCommand::Press, 0))]);
        while let Some(Reverse((cost, position, command, index))) = queue.pop() {
            if index == self.code.len() {
                return cost;
            }

            let key = (position, command, index);
            if costs.contains_key(&key) {
                continue;
            }
            costs.insert(key, cost);

            for new_command in RobotCommand::all() {
                let (new_position, new_value) = Code::execute(position, new_command, &NUMERIC_PAD);

                let new_position = match new_position {
                    Some(p) => p,
                    None => continue,
                };

                let new_index = match new_value {
                    Some(v) if v != self.code[index] => continue,
                    Some(_) => index + 1,
                    None => index,
                };

                let new_cost = cost + Code::directional_pad_cost(cache, new_command, command, nb_pads);
                queue.push(Reverse((new_cost, new_position, new_command, new_index)));
            }
        }
        unreachable!()
    }

    fn directional_pad_cost(cache: &mut Cache, goal: RobotCommand, last_command: RobotCommand, nb_pads: usize) -> usize {
        if nb_pads == 0 {
            return 1;
        }

        if let Some(&cached_cost) = cache.get(&(goal, last_command, nb_pads)) {
            return cached_cost;
        }

        let position = match last_command {
            RobotCommand::Press => Position::new(2, 0),
            RobotCommand::Move(Direction::Up) => Position::new(1, 0),
            RobotCommand::Move(Direction::Left) => Position::new(0, 1),
            RobotCommand::Move(Direction::Down) => Position::new(1, 1),
            RobotCommand::Move(Direction::Right) => Position::new(2, 1),
        };

        let mut queue = BinaryHeap::from([Reverse((0, position, RobotCommand::Press, ' '))]);
        while let Some(Reverse((cost, position, command, current_value))) = queue.pop() {
            if current_value == goal.value() {
                cache.put((goal, last_command, nb_pads), cost);
                return cost;
            }

            for new_command in RobotCommand::all() {
                let (new_position, new_value) = Code::execute(position, new_command, &DIRECTIONAL_PAD);

                let new_position = match new_position {
                    Some(p) => p,
                    None => continue,
                };

                let new_value = match new_value {
                    Some(v) if v != goal.value() => continue,
                    Some(v) => v,
                    None => ' ',
                };

                let new_cost = cost + Code::directional_pad_cost(cache, new_command, command, nb_pads - 1);
                queue.push(Reverse((new_cost, new_position, new_command, new_value)));
            }
        }
        unreachable!()
    }

    fn execute(position: Position, command: RobotCommand, pad: &Pad) -> (Option<Position>, Option<char>) {
        match command {
            RobotCommand::Move(direction) => {
                let new_position = position.move_towards(&direction);
                if pad.is_a_button(&new_position) { (Some(new_position), None) } else { (None, None) }
            }
            RobotCommand::Press => (Some(position), Some(pad.value_at(&position))),
        }
    }
}

#[derive(Default)]
struct Cache(HashMap<(RobotCommand, RobotCommand, usize), usize>);

impl Cache {
    fn put(&mut self, key: (RobotCommand, RobotCommand, usize), value: usize) {
        self.0.insert(key, value);
    }

    fn get(&self, key: &(RobotCommand, RobotCommand, usize)) -> Option<&usize> {
        self.0.get(key)
    }
}
