pub fn solution(input: &str) -> i64 {
    let parse_button = |s: &str| -> (i64, i64) {
        let (_button, rest) = s.split_once(": ").unwrap();
        let (rawx, rawy) = rest.split_once(", ").unwrap();

        let x = rawx.strip_prefix("X+").unwrap().parse::<i64>().unwrap();
        let y = rawy.strip_prefix("Y+").unwrap().parse::<i64>().unwrap();

        (x, y)
    };

    let parse_target = |s: &str| -> (i64, i64) {
        let (_target, rest) = s.split_once(": ").unwrap();
        let (rawx, rawy) = rest.split_once(", ").unwrap();

        let x = rawx.strip_prefix("X=").unwrap().parse::<i64>().unwrap() + 10000000000000;
        let y = rawy.strip_prefix("Y=").unwrap().parse::<i64>().unwrap() + 10000000000000;

        (x, y)
    };

    input
        .split("\n\n")
        .filter_map(|machine_input| {
            let mut input = machine_input.lines();

            solve_machine(
                parse_button(input.next().unwrap()),
                parse_button(input.next().unwrap()),
                parse_target(input.next().unwrap()),
            )
        })
        .sum()
}

fn solve_machine(
    (ax, ay): (i64, i64),
    (bx, by): (i64, i64),
    (dstx, dsty): (i64, i64),
) -> Option<i64> {
    // a * ax + b * bx = dstx
    // a * ay + b * by = dsty
    let n = ax * dsty - ay * dstx;
    let d = ax * by - ay * bx;

    if n % d == 0 {
        let b = n / d;

        if (dstx - bx * b) % ax == 0 {
            let a = (dstx - bx * b) / ax;

            return Some(a * 3 + b);
        }
    }

    None
}
