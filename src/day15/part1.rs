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
        // board.print();
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
        let raw = raw.trim_end();

        let w = raw.find('\n').unwrap();
        let h = (raw.len() + 1) / (w + 1);

        let mut buf = raw
            .lines()
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>();

        let idx = buf.iter().position(|c| *c == '@').unwrap();
        buf[idx] = '.';

        let c = (idx + 1) % w - 1;
        let r = (idx + 1) / w;

        Self {
            buf,
            pos: (r, c),
            w,
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

        let adj = self.get(adjr, adjc);
        if adj == '.' {
            self.pos = (adjr, adjc);
        } else if adj == 'O' {
            if let Some((dstr, dstc)) = self.find_vacant_spot(adjr, adjc, direction) {
                self.buf.swap(self.w * adjr + adjc, self.w * dstr + dstc);
                self.pos = (adjr, adjc);
            }
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
                if self.get(r, c) == 'O' {
                    score += 100 * r + c;
                }
            }
        }

        score
    }

    fn print(&self) {
        for (i, c) in self.buf.iter().enumerate() {
            if i == self.w * self.pos.0 + self.pos.1 {
                print!("@");
            } else {
                print!("{c}");
            }
            if (i + 1) % self.w == 0 {
                println!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day15::{EXAMPLE1, EXAMPLE2};

    use super::*;

    #[test]
    fn example1() {
        assert_eq!(solution(EXAMPLE1), 2028);
    }

    #[test]
    fn example2() {
        assert_eq!(solution(EXAMPLE2), 10092);
    }
}
