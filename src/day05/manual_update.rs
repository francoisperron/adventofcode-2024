use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub struct ManualUpdate {
    rules: Rules,
    updates: Vec<Update>,
}

impl ManualUpdate {
    pub fn from(input: &str) -> ManualUpdate {
        let (rules, updates) = input.split_once("\n\n").unwrap();
        ManualUpdate { rules: Rules::from(rules), updates: updates.lines().map(Update::from).collect() }
    }

    pub fn valid_updates_middle_page_sum(&self) -> usize {
        self.updates.iter().filter(|u| u.is_in_order(&self.rules)).map(|u| u.middle_page()).sum()
    }

    pub fn fixed_invalid_updates_middle_page_sum(&self) -> usize {
        self.updates
            .iter()
            .filter(|u| !u.is_in_order(&self.rules))
            .map(|u| u.reorder(&self.rules))
            .map(|u| u.middle_page())
            .sum()
    }
}

#[derive(Debug, PartialEq)]
pub struct Update {
    updates: Vec<usize>,
}

impl Update {
    pub fn from(input: &str) -> Update {
        let updates = input.split(',').map(|u| u.parse::<usize>().unwrap()).collect();
        Update { updates }
    }

    pub fn is_in_order(&self, rules: &Rules) -> bool {
        self.updates.is_sorted_by(|page1, page2| rules.are_followed(page1, page2))
    }

    pub fn reorder(&self, rules: &Rules) -> Update {
        let mut updates = self.updates.clone();
        updates.sort_by(|page1, page2| if rules.are_followed(page1, page2) { Ordering::Less } else { Ordering::Greater });
        Update { updates }
    }

    pub fn middle_page(&self) -> usize {
        *self.updates.get(self.updates.len() / 2).unwrap()
    }
}

pub struct Rules {
    rules: HashMap<usize, HashSet<usize>>,
}

impl Rules {
    pub fn from(input: &str) -> Rules {
        let mut rules = HashMap::<usize, HashSet<usize>>::new();
        for rule in input.lines() {
            let (page1, page2) = rule.split_once('|').unwrap();
            rules.entry(page2.parse().unwrap()).or_default().insert(page1.parse().unwrap());
        }
        Rules { rules }
    }

    pub fn are_followed(&self, page1: &usize, page2: &usize) -> bool {
        self.rules.contains_key(page2) && self.rules[page2].contains(page1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_if_an_update_is_valid() {
        let update = Update::from("75,47");
        let rules = Rules::from("75|47");

        assert!(update.is_in_order(&rules));
    }

    #[test]
    fn determines_if_an_update_is_invalid() {
        let update = Update::from("75,47");
        let rules = Rules::from("47|75");

        assert!(!update.is_in_order(&rules))
    }

    #[test]
    fn reorders_update() {
        let update = Update::from("1,2,3");
        let rules = Rules::from("3|1\n2|1");

        assert_eq!(update.reorder(&rules), Update { updates: vec![2, 3, 1] });
    }
}
