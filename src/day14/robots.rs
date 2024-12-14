use crate::toolbox::XY;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Robots {
    pub robots: Vec<Robot>,
    pub size: XY,
}

impl Robots {
    pub fn from(input: &str, size: XY) -> Robots {
        let robots = input.lines().map(Robot::from).collect();
        Robots { robots, size }
    }

    pub fn move_(&mut self, moves: usize) {
        for _ in 0..moves {
            for robot in &mut self.robots {
                robot.move_(&self.size);
            }
        }
    }

    pub fn safety_factor(&self) -> usize {
        let split_x = self.size.x / 2;
        let split_y = self.size.y / 2;

        let quadrant1 = self.robots.iter().filter(|r| r.pos.x < split_x && r.pos.y < split_y).count();
        let quadrant2 = self.robots.iter().filter(|r| r.pos.x > split_x && r.pos.y < split_y).count();
        let quadrant3 = self.robots.iter().filter(|r| r.pos.x < split_x && r.pos.y > split_y).count();
        let quadrant4 = self.robots.iter().filter(|r| r.pos.x > split_x && r.pos.y > split_y).count();

        quadrant1 * quadrant2 * quadrant3 * quadrant4
    }

    pub fn moves_for_easter_egg(&mut self) -> usize {
        for moves in 1.. {
            for robot in &mut self.robots {
                robot.move_(&self.size);
            }

            let mut uniq = HashSet::new();
            if self.robots.iter().all(|r| uniq.insert(r.pos)) {
                return moves;
            }
        }
        unreachable!();
    }
}

#[derive(Debug)]
pub struct Robot {
    pos: XY,
    vel: XY,
}

impl Robot {
    pub fn from(input: &str) -> Robot {
        let regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        let captures = regex.captures(input).unwrap();
        let mut caps = captures.iter().skip(1).map(|c| c.unwrap().as_str()).map(|s| s.parse::<isize>().unwrap());
        Robot { pos: XY::new((caps.next().unwrap(), caps.next().unwrap())), vel: XY::new((caps.next().unwrap(), caps.next().unwrap())) }
    }

    pub fn move_(&mut self, max: &XY) {
        self.pos = (self.pos + self.vel) % *max;
    }
}

#[cfg(test)]
mod tests {
    use crate::day14::robots::Robot;
    use crate::toolbox::XY;

    #[test]
    fn moves_inside_grid() {
        let mut robot = Robot::from("p=2,4 v=2,-3");
        robot.move_(&XY::new((11, 7)));

        assert_eq!(robot.pos, XY::new((4, 1)));
    }

    #[test]
    fn teleports_min_y() {
        let mut robot = Robot::from("p=2,4 v=4,-6");
        robot.move_(&XY::new((11, 7)));

        assert_eq!(robot.pos, XY::new((6, 5)));
    }

    #[test]
    fn teleports_max_y() {
        let mut robot = Robot::from("p=2,4 v=2,4");
        robot.move_(&XY::new((11, 7)));

        assert_eq!(robot.pos, XY::new((4, 1)));
    }

    #[test]
    fn teleports_min_x() {
        let mut robot = Robot::from("p=2,4 v=-3,2");
        robot.move_(&XY::new((11, 7)));

        assert_eq!(robot.pos, XY::new((10, 6)));
    }

    #[test]
    fn teleports_max_x() {
        let mut robot = Robot::from("p=2,4 v=10,2");
        robot.move_(&XY::new((11, 7)));

        assert_eq!(robot.pos, XY::new((1, 6)));
    }
}
