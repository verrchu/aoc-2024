pub fn solution(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        process_line(line, &mut sum);
    }

    sum
}

fn process_line(mut line: &str, sum: &mut u64) {
    loop {
        let (remaining, factors) = probe(line);
        if let Some(remaining) = remaining {
            line = remaining;
        } else {
            break;
        }

        if let Some((l, r)) = factors {
            *sum += l * r;
        }
    }
}

fn probe(line: &str) -> (Option<&str>, Option<(u64, u64)>) {
    if line.len() < 4 {
        return (None, None);
    }

    let Some(line) = line.strip_prefix("mul(") else {
        return (Some(&line[1..]), None);
    };

    #[derive(Debug, PartialEq)]
    enum Target {
        Number,
        Comma,
        Bracket,
    }

    let mut targets = vec![Target::Number];

    let mut seen_left_factor = false;
    let mut seen_right_factor = false;
    let mut seen_comma = false;

    let mut left_factor_buf = vec![];
    let mut right_factor_buf = vec![];

    let mut line = line.chars();
    loop {
        let Some(c) = line.next() else {
            return (Some(line.as_str()), None);
        };

        match c {
            c if c.is_ascii_digit() => {
                if targets.contains(&Target::Number) {
                    if !seen_left_factor {
                        seen_left_factor = true;
                        targets = vec![Target::Number, Target::Comma];
                    }

                    if seen_comma && !seen_right_factor {
                        seen_right_factor = true;
                        targets = vec![Target::Number, Target::Bracket];
                    }

                    if seen_comma {
                        right_factor_buf.push(c);
                    } else {
                        left_factor_buf.push(c);
                    }
                } else {
                    return (Some(line.as_str()), None);
                }
            }
            ',' => {
                if targets.contains(&Target::Comma) {
                    seen_comma = true;
                    targets = vec![Target::Number];
                } else {
                    return (Some(line.as_str()), None);
                }
            }
            ')' => {
                if targets.contains(&Target::Bracket) {
                    let left_factor = left_factor_buf
                        .into_iter()
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();

                    let right_factor = right_factor_buf
                        .into_iter()
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();

                    return (Some(line.as_str()), Some((left_factor, right_factor)));
                } else {
                    return (Some(line.as_str()), None);
                }
            }
            _ => {
                return (Some(line.as_str()), None);
            }
        }
    }
}
