static INPUT: &str = include_str!("./input");
// static INPUT: &str = include_str!("./example");

fn main() {
    println!("SOLUTION: {}", solution(INPUT));
}

fn solution(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let report = line.split_whitespace().map(|level| level.parse().unwrap());

            is_safe_report(report)
        })
        .filter(|x| *x)
        .count()
}

#[derive(Debug, PartialEq, Eq)]
enum Direaction {
    Up,
    Down,
}

fn is_safe_report<I>(report: I) -> bool
where
    I: Iterator<Item = i64>,
{
    let mut report = report.peekable();

    let Some(a) = report.next() else {
        return true;
    };

    let Some(b) = report.peek() else {
        return true;
    };

    let direction = match b - a >= 1 {
        true => Direaction::Up,
        false => Direaction::Down,
    };

    let check_adjacent = |l: i64, r: i64| {
        let diff = r - l;

        match direction {
            Direaction::Up => (1..=3).contains(&diff),
            Direaction::Down => (-3..=-1).contains(&diff),
        }
    };

    let mut prev = a;
    for next in report {
        if !check_adjacent(prev, next) {
            return false;
        }

        prev = next;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("./example");

    #[test]
    fn test_example() {
        assert_eq!(solution(INPUT), 2);
    }
}
