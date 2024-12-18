use std::collections::{HashMap, HashSet};

const SIZE: isize = 71;

// const SIZE: isize = 7;

pub fn solution(input: &str) -> String {
    let bytes = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
        })
        .collect::<Vec<_>>();

    let (mut bot, mut top) = (0, bytes.len());
    loop {
        if top - bot <= 1 {
            let (x, y) = bytes[top];
            return format!("{x},{y}");
        }

        let idx = (bot + top) / 2;

        let bytes = HashSet::from_iter(bytes[0..=idx].iter().copied());
        let mut dp = HashMap::<(isize, isize), u64>::new();

        search(0, 0, 0, &bytes, &mut dp);
        search(0, 0, 0, &bytes, &mut dp);

        if dp.get(&(SIZE - 1, SIZE - 1)).is_some() {
            bot = idx;
        } else {
            top = idx;
        }
    }
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
