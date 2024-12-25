mod bit;
mod fruit_monitor;
mod gate;
mod operation;
mod wires;

#[cfg(test)]
mod tests {
    use crate::day24::fruit_monitor::FruitMonitor;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        let mut monitor = FruitMonitor::from(EXAMPLE);

        assert_eq!(monitor.wires_z_output(), 4);
    }
    #[test]
    fn solves_part1() {
        let input = daily_input(24);
        let mut monitor = FruitMonitor::from(&input);

        assert_eq!(monitor.wires_z_output(), 56620966442854);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(24);
        let monitor = FruitMonitor::from(&input);

        assert_eq!(monitor.swapped_wires(), "chv,jpj,kgj,rts,vvw,z07,z12,z26");
    }

    pub const EXAMPLE: &str = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
}
