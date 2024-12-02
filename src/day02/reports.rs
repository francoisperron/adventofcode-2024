pub struct Reports {
    reports: Vec<Report>,
}

impl Reports {
    pub fn from(input: &str) -> Reports {
        let reports = input.split("\n").map(Report::from).collect();
        Reports { reports }
    }
}

impl Reports {
    pub fn safe_count(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| report.is_safe())
            .count()
    }

    pub fn safe_count_with_problem_dampener(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| report.is_safe() || report.is_safe_with_problem_dampener())
            .count()
    }
}

pub struct Report {
    levels: Vec<usize>,
}

impl Report {
    pub fn from(line: &str) -> Report {
        let levels = line
            .split_whitespace()
            .map(|value| value.parse().unwrap())
            .collect();
        Report { levels }
    }
}

impl Report {
    pub fn is_safe(&self) -> bool {
        let diff = self
            .levels
            .iter()
            .zip(self.levels.iter().skip(1))
            .map(|(l1, l2)| *l2 as isize - *l1 as isize)
            .collect::<Vec<isize>>();

        let increasing = diff.iter().all(|d| *d > 0);
        let decreasing = diff.iter().all(|d| *d < 0);
        let safe_level = diff.iter().all(|d| d.abs() >= 1 && d.abs() <= 3);

        (increasing || decreasing) && safe_level
    }

    fn is_safe_with_problem_dampener(&self) -> bool {
        (0..self.levels.len())
            .any(|i| {
                let mut dampened_levels = self.levels.clone();
                dampened_levels.remove(i);
                Report { levels: dampened_levels }.is_safe()
            })
    }
}
