use std::collections::HashMap;

pub fn solution(input: &str) -> u64 {
    let split = |line: &str| {
        let mut parts = line.split_whitespace();
        let l = parts.next().unwrap().parse::<u64>().unwrap();
        let r = parts.next().unwrap().parse::<u64>().unwrap();

        (l, r)
    };

    let (ls, rs): (Vec<_>, Vec<_>) = input.lines().map(split).unzip();

    let rcounts = rs
        .into_iter()
        .fold(HashMap::<u64, u64>::new(), |mut acc, x| {
            (*acc.entry(x).or_default()) += 1;

            acc
        });

    ls.into_iter()
        .map(|l| l * rcounts.get(&l).copied().unwrap_or_default())
        .sum()
}
