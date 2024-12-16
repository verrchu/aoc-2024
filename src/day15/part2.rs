use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    N,
    W,
    S,
    E,
}

pub fn solution(input: &str) -> usize {
    let (board, moves) = input.split_once("\n\n").unwrap();

    let mut board = Board::new(board);
    board.print();

    for c in moves.lines().flat_map(|line| line.chars()) {
        match c {
            '>' => board.advance(Direction::E),
            '<' => board.advance(Direction::W),
            '^' => board.advance(Direction::N),
            'v' => board.advance(Direction::S),
            _ => unreachable!(),
        }

        // let mut s = String::new();
        // std::io::stdin().read_line(&mut s).unwrap();
        // println!("\n{c}");
        // println!("{}", board.print());
    }

    board.score()
}

struct Board {
    buf: Vec<char>,
    pos: (usize, usize),
    w: usize,
    h: usize,
}

impl Board {
    fn new(raw: &str) -> Board {
        let raw = raw.trim();

        let w = raw.find('\n').unwrap();
        let h = (raw.len() + 1) / (w + 1);

        let mut buf = raw
            .lines()
            .flat_map(|line| {
                line.chars().flat_map(|c| match c {
                    '.' | '#' => [c; 2],
                    '@' => ['@', '.'],
                    'O' => ['[', ']'],
                    c => unreachable!("DOUND THIS: {c}"),
                })
            })
            .collect::<Vec<_>>();

        let idx = buf.iter().position(|c| *c == '@').unwrap();
        buf[idx] = '.';

        let c = (idx + 1) % (w * 2) - 1;
        let r = (idx + 1) / (w * 2);

        Self {
            buf,
            pos: (r, c),
            w: w * 2,
            h,
        }
    }

    fn get(&self, r: usize, c: usize) -> char {
        self.buf[self.w * r + c]
    }

    fn advance(&mut self, direction: Direction) {
        let (r, c) = self.pos;

        let (adjr, adjc) = match direction {
            Direction::E => (r, c + 1),
            Direction::S => (r + 1, c),
            Direction::W => (r, c - 1),
            Direction::N => (r - 1, c),
        };

        match self.get(adjr, adjc) {
            '#' => (),
            '.' => {
                self.pos = (adjr, adjc);
            }
            adj @ ('[' | ']') => match direction {
                Direction::E => {
                    if let Some((dstr, dstc)) = self.find_vacant_spot(adjr, adjc, direction) {
                        let start = self.w * adjr + adjc;
                        let end = self.w * dstr + dstc;
                        for idx in ((start + 1)..=end).rev() {
                            self.buf.swap(idx, idx - 1);
                        }

                        self.pos = (adjr, adjc);
                    }
                }
                Direction::W => {
                    if let Some((dstr, dstc)) = self.find_vacant_spot(adjr, adjc, direction) {
                        let end = self.w * adjr + adjc;
                        let start = self.w * dstr + dstc;
                        for idx in start..end {
                            self.buf.swap(idx, idx + 1);
                        }

                        self.pos = (adjr, adjc);
                    }
                }
                Direction::N => {
                    let mut stack = match adj {
                        '[' => vec![HashSet::from_iter([(adjr, adjc), (adjr, adjc + 1)])],
                        ']' => vec![HashSet::from_iter([(adjr, adjc), (adjr, adjc - 1)])],
                        _ => unreachable!(),
                    };

                    loop {
                        let mut next_level = HashSet::new();

                        for (r, c) in stack.last().unwrap().iter().copied() {
                            let adj = self.get(r - 1, c);
                            if adj == '#' {
                                return;
                            } else if adj == '[' {
                                next_level.insert((r - 1, c));
                                next_level.insert((r - 1, c + 1));
                            } else if adj == ']' {
                                next_level.insert((r - 1, c));
                                next_level.insert((r - 1, c - 1));
                            }
                        }

                        if !next_level.is_empty() {
                            stack.push(next_level);
                        } else {
                            break;
                        }
                    }

                    for level in stack.into_iter().rev() {
                        for (r, c) in level {
                            let idx = self.w * r + c;
                            self.buf.swap(idx, idx - self.w);
                        }
                    }

                    self.pos = (adjr, adjc);
                }
                Direction::S => {
                    let mut stack = match adj {
                        '[' => vec![HashSet::from_iter([(adjr, adjc), (adjr, adjc + 1)])],
                        ']' => vec![HashSet::from_iter([(adjr, adjc), (adjr, adjc - 1)])],
                        _ => unreachable!(),
                    };

                    loop {
                        let mut next_level = HashSet::new();

                        for (r, c) in stack.last().unwrap().iter().copied() {
                            let adj = self.get(r + 1, c);
                            if adj == '#' {
                                return;
                            } else if adj == '[' {
                                next_level.insert((r + 1, c));
                                next_level.insert((r + 1, c + 1));
                            } else if adj == ']' {
                                next_level.insert((r + 1, c));
                                next_level.insert((r + 1, c - 1));
                            }
                        }

                        if !next_level.is_empty() {
                            stack.push(next_level);
                        } else {
                            break;
                        }
                    }

                    for level in stack.into_iter().rev() {
                        for (r, c) in level {
                            let idx = self.w * r + c;
                            self.buf.swap(idx, idx + self.w);
                        }
                    }

                    self.pos = (adjr, adjc);
                }
            },
            _ => unreachable!(),
        }
    }

    fn find_vacant_spot(&self, r: usize, c: usize, direction: Direction) -> Option<(usize, usize)> {
        match direction {
            Direction::E => {
                for c in (c + 1)..(self.w - 1) {
                    let ch = self.get(r, c);
                    if ch == '.' {
                        return Some((r, c));
                    } else if ch == '#' {
                        return None;
                    }
                }
            }
            Direction::S => {
                for r in (r + 1)..(self.h - 1) {
                    let ch = self.get(r, c);
                    if ch == '.' {
                        return Some((r, c));
                    } else if ch == '#' {
                        return None;
                    }
                }
            }
            Direction::W => {
                for c in (1..=(c - 1)).rev() {
                    let ch = self.get(r, c);
                    if ch == '.' {
                        return Some((r, c));
                    } else if ch == '#' {
                        return None;
                    }
                }
            }
            Direction::N => {
                for r in (1..=(r - 1)).rev() {
                    let ch = self.get(r, c);
                    if ch == '.' {
                        return Some((r, c));
                    } else if ch == '#' {
                        return None;
                    }
                }
            }
        }

        None
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for r in 0..self.h {
            for c in 0..self.w {
                if self.get(r, c) == '[' {
                    score += 100 * r + c;
                }
            }
        }

        score
    }

    fn print(&self) -> String {
        let mut s = String::new();

        for (i, c) in self.buf.iter().enumerate() {
            if i == self.w * self.pos.0 + self.pos.1 {
                s.push('@');
            } else {
                s.push(*c);
            }
            if (i + 1) % self.w == 0 {
                s.push('\n');
            }
        }

        s
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn move_right() {
        let input = r#"
##########
#..@OO.#.#
##########
"#;

        let mut board = Board::new(input);

        assert_eq!(
            board.print().trim(),
            r#"
####################
##....@.[][]..##..##
####################
"#
            .trim()
            .to_string()
        );

        board.advance(Direction::E);

        assert_eq!(
            board.print().trim(),
            r#"
####################
##.....@[][]..##..##
####################
"#
            .trim()
            .to_string()
        );

        board.advance(Direction::E);

        assert_eq!(
            board.print().trim(),
            r#"
####################
##......@[][].##..##
####################
"#
            .trim()
            .to_string()
        );

        board.advance(Direction::E);

        assert_eq!(
            board.print().trim(),
            r#"
####################
##.......@[][]##..##
####################
"#
            .trim()
            .to_string()
        );

        board.advance(Direction::E);

        assert_eq!(
            board.print().trim(),
            r#"
####################
##.......@[][]##..##
####################
"#
            .trim()
            .to_string()
        );
    }

    #[test]
    fn move_left() {
        let input = r#"
##########
#.#.OO.@.#
##########
"#;

        let mut board = Board::new(input);

        assert_eq!(
            board.print().trim(),
            r#"
####################
##..##..[][]..@...##
####################
"#
            .trim()
            .to_string()
        );

        board.advance(Direction::W);

        assert_eq!(
            board.print().trim(),
            r#"
####################
##..##..[][].@....##
####################
"#
            .trim()
            .to_string()
        );

        board.advance(Direction::W);

        assert_eq!(
            board.print().trim(),
            r#"
####################
##..##..[][]@.....##
####################
"#
            .trim()
            .to_string()
        );

        board.advance(Direction::W);

        assert_eq!(
            board.print().trim(),
            r#"
####################
##..##.[][]@......##
####################
"#
            .trim()
            .to_string()
        );

        board.advance(Direction::W);

        assert_eq!(
            board.print().trim(),
            r#"
####################
##..##[][]@.......##
####################
"#
            .trim()
            .to_string()
        );

        board.advance(Direction::W);

        assert_eq!(
            board.print().trim(),
            r#"
####################
##..##[][]@.......##
####################
"#
            .trim()
            .to_string()
        );
    }

    #[test]
    fn move_up_simple() {
        let input = r#"
#####
#...#
#...#
#.O.#
#...#
#.@.#
#####
"#;

        let mut board = Board::new(input);

        assert_eq!(
            board.print().trim(),
            r#"
##########
##......##
##......##
##..[]..##
##......##
##..@...##
##########
"#
            .trim()
            .to_string()
        );

        board.advance(Direction::N);

        assert_eq!(
            board.print().trim(),
            r#"
##########
##......##
##......##
##..[]..##
##..@...##
##......##
##########
"#
            .trim()
            .to_string()
        );

        board.advance(Direction::N);

        assert_eq!(
            board.print().trim(),
            r#"
##########
##......##
##..[]..##
##..@...##
##......##
##......##
##########
"#
            .trim()
            .to_string()
        );

        board.advance(Direction::N);

        assert_eq!(
            board.print().trim(),
            r#"
##########
##..[]..##
##..@...##
##......##
##......##
##......##
##########
"#
            .trim()
            .to_string()
        );

        board.advance(Direction::N);

        assert_eq!(
            board.print().trim(),
            r#"
##########
##..[]..##
##..@...##
##......##
##......##
##......##
##########
"#
            .trim()
            .to_string()
        );
    }

    #[test]
    fn move_up_stack() {
        let input = r#"
#####
#...#
#.O.#
#.O.#
#...#
#.@.#
#####
"#;

        let mut board = Board::new(input);

        assert_eq!(
            board.print().trim(),
            r#"
##########
##......##
##..[]..##
##..[]..##
##......##
##..@...##
##########
"#
            .trim()
            .to_string()
        );

        board.advance(Direction::N);

        assert_eq!(
            board.print().trim(),
            r#"
##########
##......##
##..[]..##
##..[]..##
##..@...##
##......##
##########
"#
            .trim()
            .to_string()
        );

        board.advance(Direction::N);

        assert_eq!(
            board.print().trim(),
            r#"
##########
##..[]..##
##..[]..##
##..@...##
##......##
##......##
##########
"#
            .trim()
            .to_string()
        );

        board.advance(Direction::N);

        assert_eq!(
            board.print().trim(),
            r#"
##########
##..[]..##
##..[]..##
##..@...##
##......##
##......##
##########
"#
            .trim()
            .to_string()
        );
    }
}
