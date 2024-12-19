use std::collections::HashSet;

pub fn solution(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();

    let towels = towels
        .split(", ")
        .map(|t| t.as_bytes())
        .collect::<HashSet<_>>();

    patterns
        .lines()
        .filter_map(|p| arrangements(p.as_bytes(), &towels))
        .sum()
}

fn arrangements(p: &[u8], ts: &HashSet<&[u8]>) -> Option<usize> {
    if p.is_empty() {
        return Some(1);
    }

    let mut result = 0;
    if ts.contains(p) {
        result += 1;
    }

    for i in 1..p.len() {
        let head = &p[0..i];
        let tail = &p[i..];

        if ts.contains(head) {
            if let Some(n) = arrangements(tail, ts) {
                result += n
            }
        }
    }

    (result > 0).then_some(result)
}
