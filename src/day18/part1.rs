use std::collections::{HashMap, HashSet};

const SIZE: isize = 71;
const TAKE: usize = 1024;

// const SIZE: isize = 7;
// const TAKE: usize = 12;

pub fn solution(input: &str) -> u64 {
    let bytes = input
        .lines()
        .take(TAKE)
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
        })
        .collect::<HashSet<_>>();

    let mut dp = HashMap::<(isize, isize), u64>::new();

    search(0, 0, 0, &bytes, &mut dp);
    search(0, 0, 0, &bytes, &mut dp);

    dp[&(SIZE - 1, SIZE - 1)]
}

fn search(
    x: isize,
    y: isize,
    s: u64,
    bytes: &HashSet<(isize, isize)>,
    dp: &mut HashMap<(isize, isize), u64>,
) {
    if !(0..SIZE).contains(&x) || !(0..SIZE).contains(&y) {
        return;
    }

    if bytes.contains(&(x, y)) {
        return;
    }

    let prev_score = dp.entry((x, y)).or_insert(u64::MAX);
    if s < *prev_score {
        *prev_score = s;
    } else {
        return;
    }

    search(x + 1, y, s + 1, bytes, &mut *dp);
    search(x, y + 1, s + 1, bytes, &mut *dp);
    search(x - 1, y, s + 1, bytes, &mut *dp);
    search(x, y - 1, s + 1, bytes, &mut *dp);
}
