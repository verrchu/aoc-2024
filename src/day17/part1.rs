pub fn solution(input: &str) -> String {
    let (regs, tape) = input.split_once("\n\n").unwrap();
    let mut regs = regs.lines();

    let parse_reg = |raw: &str| {
        let (_, val) = raw.split_once(": ").unwrap();
        val.parse::<u32>().unwrap()
    };

    let mut ra = parse_reg(regs.next().unwrap());
    let mut rb = parse_reg(regs.next().unwrap());
    let mut rc = parse_reg(regs.next().unwrap());

    let tape = tape
        .strip_prefix("Program: ")
        .unwrap()
        .trim()
        .split(',')
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut out = Vec::new();

    let combo = |arg, ra, rb, rc| match arg {
        arg if arg < 4 => arg,
        4 => ra,
        5 => rb,
        6 => rc,
        _ => unreachable!(),
    };

    let mut idx = 0;
    loop {
        let (Some(op), Some(arg)) = (tape.get(idx), tape.get(idx + 1)) else {
            break;
        };

        match op {
            0 => ra /= 2u32.pow(combo(*arg, ra, rb, rc)),
            1 => rb ^= arg,
            2 => rb = combo(*arg, ra, rb, rc) % 8,
            3 => {
                if ra != 0 {
                    idx = *arg as usize;
                    continue;
                }
            }
            4 => rb ^= rc,
            5 => out.push(combo(*arg, ra, rb, rc) % 8),
            6 => rb = ra / 2u32.pow(combo(*arg, ra, rb, rc)),
            7 => rc = ra / 2u32.pow(combo(*arg, ra, rb, rc)),
            _ => unreachable!(),
        }

        idx += 2;
    }

    let mut buf = String::new();
    for (i, c) in out.iter().copied().enumerate() {
        buf.push(char::from_digit(c, 10).unwrap());
        if i < out.len() - 1 {
            buf.push(',');
        }
    }

    buf
}
