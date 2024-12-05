use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn solution(input: &str) -> u64 {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = Rules::parse(rules);

    updates
        .lines()
        .filter_map(|line| {
            let mut update = Update::parse(line);

            if !update.check(&rules) {
                return Some(update.fix(&rules));
            }

            None
        })
        .sum()
}

#[derive(Debug)]
struct Update(Vec<u64>);

impl Update {
    fn parse(s: &str) -> Self {
        let inner = s
            .split(',')
            .map(|page| page.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        Self(inner)
    }

    fn check(&self, rules: &Rules) -> bool {
        let mut prev = HashSet::<u64>::new();

        for n in self.0.iter().copied() {
            if !rules.check(n, &prev) {
                return false;
            }

            prev.insert(n);
        }

        true
    }

    fn fix(&mut self, rules: &Rules) -> u64 {
        debug_assert!(!self.check(rules));

        self.0.sort_by(|a, b| {
            if let Some(next) = rules.get(a) {
                if next.contains(b) {
                    return Ordering::Less;
                }
            }

            Ordering::Equal
        });

        debug_assert!(self.check(rules));

        self.0[self.0.len() / 2]
    }
}

#[derive(Debug)]
struct Rules(HashMap<u64, HashSet<u64>>);

impl Rules {
    fn parse(s: &str) -> Self {
        let mut inner = HashMap::<u64, HashSet<u64>>::new();

        for line in s.trim().lines() {
            let (l, r) = line.split_once('|').unwrap();
            let l = l.parse::<u64>().unwrap();
            let r = r.parse::<u64>().unwrap();

            inner.entry(l).or_default().insert(r);
        }

        Self(inner)
    }

    fn get(&self, key: &u64) -> Option<&HashSet<u64>> {
        self.0.get(key)
    }

    fn check(&self, curr: u64, prev: &HashSet<u64>) -> bool {
        let Some(next) = self.0.get(&curr) else {
            return true;
        };

        next.intersection(prev).count() == 0
    }
}
