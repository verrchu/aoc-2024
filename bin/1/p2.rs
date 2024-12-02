use std::collections::HashMap;

static INPUT: &str = include_str!("./input");
// static INPUT: &str = include_str!("./example");

fn main() {
    println!("SOLUTION: {}", solution(INPUT));
}

fn solution(input: &str) -> u64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("./example");

    #[test]
    fn test_example() {
        assert_eq!(solution(INPUT), 31);
    }
}
