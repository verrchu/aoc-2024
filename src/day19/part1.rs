use std::collections::HashSet;

pub fn solution(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();

    let towels = towels
        .split(", ")
        .map(|t| t.as_bytes())
        .collect::<HashSet<_>>();

    patterns
        .lines()
        .filter(|p| is_possible(p.as_bytes(), &towels))
        .count()
}

fn is_possible(p: &[u8], ts: &HashSet<&[u8]>) -> bool {
    if p.is_empty() || ts.contains(p) {
        return true;
    }

    for i in 1..p.len() {
        let head = &p[0..i];
        let tail = &p[i..];

        if ts.contains(head) && is_possible(tail, ts) {
            return true;
        }
    }

    false
}
