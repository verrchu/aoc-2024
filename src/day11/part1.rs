use std::collections::HashMap;

pub fn solution(input: &str) -> usize {
    let stones = input
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut memo = HashMap::<(u64, usize), usize>::new();

    stones.into_iter().map(|s| search(s, 25, &mut memo)).sum()
}

fn search(n: u64, steps: usize, memo: &mut HashMap<(u64, usize), usize>) -> usize {
    if steps == 0 {
        return 1;
    }

    if let Some(size) = memo.get(&(n, steps)) {
        return *size;
    }

    let size = if n == 0 {
        search(1, steps - 1, &mut *memo)
    } else if let Some((l, r)) = split_even(n) {
        search(l, steps - 1, &mut *memo) + search(r, steps - 1, &mut *memo)
    } else {
        search(n * 2024, steps - 1, &mut *memo)
    };

    memo.insert((n, steps), size);
    size
}

fn split_even(n: u64) -> Option<(u64, u64)> {
    let ndigits = match (n as f64).log10() {
        n if n.fract() == 0.0 => n + 1.0,
        n => n.ceil(),
    } as u64;

    (ndigits % 2 == 0).then(|| {
        let factor = 10u64.pow((ndigits / 2) as u32);
        (n / factor, n % factor)
    })
}
