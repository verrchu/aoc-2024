pub fn solution(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let report = line
                .split_whitespace()
                .map(|level| level.parse().unwrap())
                .collect::<Vec<_>>();

            is_safe_report(report.iter().copied()) || is_almost_safe_report(&report)
        })
        .filter(|x| *x)
        .count()
}

#[derive(Debug, PartialEq, Eq)]
enum Direaction {
    Up,
    Down,
}

fn is_safe_report(report: impl Iterator<Item = i64>) -> bool {
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

fn is_almost_safe_report(report: &[i64]) -> bool {
    for i in 0..report.len() {
        let l = &report[0..i];
        let r = &report[(i + 1)..report.len()];

        let sub_report = l.iter().chain(r.iter()).copied();
        if is_safe_report(sub_report) {
            return true;
        }
    }

    false
}
