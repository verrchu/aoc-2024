pub fn solution(input: &str) -> u64 {
    let mut result = 0;
    for line in input.lines() {
        let (target, parts) = line.split_once(": ").unwrap();
        let target = target.parse::<u64>().unwrap();
        let parts = parts.split_whitespace().map(|p| p.parse::<u64>().unwrap());

        if check_line(target, parts) {
            result += target;
        }
    }

    result
}

pub fn check_line(target: u64, mut parts: impl Iterator<Item = u64>) -> bool {
    let first = parts.next().unwrap();

    let mut progress = vec![first];
    let mut parts = parts.peekable();
    while let Some(p) = parts.next() {
        let last = parts.peek().is_none();

        let mut next_progress = Vec::with_capacity(progress.len() * 3);
        for r in progress {
            let concat = {
                let digits = match (p as f64).log10() {
                    n if n.fract() == 0.0 => n + 1.0,
                    n => n.ceil(),
                };
                r * 10u64.pow(digits as u32) + p
            };

            let candidates = [r + p, r * p, concat];
            if last && candidates.contains(&target) {
                return true;
            } else {
                next_progress.extend(candidates);
            }
        }

        progress = next_progress;
    }

    false
}
