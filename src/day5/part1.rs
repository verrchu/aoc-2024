use std::collections::{HashMap, HashSet};

pub fn solution(input: &str) -> u64 {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = Rules::parse(rules);

    updates
        .lines()
        .filter_map(|line| Update::parse(line).check(&rules))
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

    fn check(&self, rules: &Rules) -> Option<u64> {
        let mid = self.0[self.0.len() / 2];
        let mut prev = HashSet::<u64>::new();

        for n in self.0.iter().copied() {
            if !rules.check(n, &prev) {
                return None;
            }

            prev.insert(n);
        }

        Some(mid)
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

    fn check(&self, curr: u64, prev: &HashSet<u64>) -> bool {
        let Some(next) = self.0.get(&curr) else {
            return true;
        };

        next.intersection(prev).count() == 0
    }
}
