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

    let (mut ls, mut rs): (Vec<_>, Vec<_>) = input.lines().map(split).unzip();
    ls.sort_unstable();
    rs.sort_unstable();

    ls.into_iter().zip(rs).map(|(l, r)| l.abs_diff(r)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("./example");

    #[test]
    fn test_example() {
        assert_eq!(solution(INPUT), 11);
    }
}
