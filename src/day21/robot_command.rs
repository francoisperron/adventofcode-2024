use crate::toolbox::Direction;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Ord, PartialOrd)]
pub enum RobotCommand {
    Move(Direction),
    Press,
}

impl RobotCommand {
    pub fn all() -> Vec<RobotCommand> {
        vec![RobotCommand::Move(Direction::Up), RobotCommand::Move(Direction::Down), RobotCommand::Move(Direction::Left), RobotCommand::Move(Direction::Right), RobotCommand::Press]
    }

    pub fn value(&self) -> char {
        match self {
            RobotCommand::Press => 'A',
            RobotCommand::Move(direction) => match direction {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day21::robot_command::RobotCommand;
    use crate::toolbox::Direction;

    #[test]
    fn all_commands() {
        let commands = RobotCommand::all();

        assert_eq!(commands.len(), 5);
    }

    #[test]
    fn converts_to_value() {
        assert_eq!(RobotCommand::Press.value(), 'A');
        assert_eq!(RobotCommand::Move(Direction::Up).value(), '^');
        assert_eq!(RobotCommand::Move(Direction::Down).value(), 'v');
        assert_eq!(RobotCommand::Move(Direction::Left).value(), '<');
        assert_eq!(RobotCommand::Move(Direction::Right).value(), '>');
    }
}
