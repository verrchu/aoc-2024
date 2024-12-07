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
    for p in parts {
        let mut next_progress = Vec::with_capacity(progress.len() * 2);
        for r in progress {
            let candidates = [r + p, r * p];
            next_progress.extend(candidates);
        }

        progress = next_progress;
    }

    progress.contains(&target)
}
