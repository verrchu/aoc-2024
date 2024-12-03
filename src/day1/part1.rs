pub fn solution(input: &str) -> u64 {
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
