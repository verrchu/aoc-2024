use std::collections::HashMap;

#[derive(PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn flip(self) -> Self {
        match self {
            Self::N => Self::S,
            Self::E => Self::W,
            Self::S => Self::N,
            Self::W => Self::E,
        }
    }
}

pub fn solution(input: &str) -> usize {
    let mut board = Board::new(input);
    let (sx, sy) = board.take_start();
    let (_ex, _ey) = board.take_end();

    let mut path = HashMap::<(usize, usize), u64>::new();
    let starting_direction = find_starting_direction(sx, sy, &board);
    find_path(sx, sy, 0, starting_direction, &board, &mut path);

    let cheats = find_cheats(&path);

    cheats
        .into_iter()
        .filter_map(|(save, n)| (save >= 100).then_some(n))
        .sum()
}

fn find_cheats(path: &HashMap<(usize, usize), u64>) -> HashMap<u64, usize> {
    let mut cheats = HashMap::new();

    let get_dst_score = |nx, ny| {
        if let (Ok(nx), Ok(ny)) = (usize::try_from(nx), usize::try_from(ny)) {
            return path.get(&(nx, ny)).copied();
        }

        None
    };

    for ((x, y), src_score) in path {
        let (x, y) = (*x as isize, *y as isize);

        for xd in -20isize..=20isize {
            let ylimit = 20 - xd.abs();

            for yd in -ylimit..=ylimit {
                if let Some(dst_score) = get_dst_score(x + xd, y + yd) {
                    let steps = (xd.abs() + yd.abs()) as u64;
                    if dst_score > src_score + steps {
                        let diff = dst_score - (src_score + steps);
                        *(cheats.entry(diff).or_default()) += 1;
                    }
                }
            }
        }
    }

    cheats
}

fn find_path(
    x: usize,
    y: usize,
    s: u64,
    d: Direction,
    board: &Board,
    path: &mut HashMap<(usize, usize), u64>,
) {
    let get_next = || {
        use Direction::*;

        let from = d.flip();

        if from != N {
            let (nx, ny) = (x - 1, y);
            if board.get(nx, ny) == '.' {
                return Some((nx, ny, N));
            }
        }

        if from != E {
            let (nx, ny) = (x, y + 1);
            if board.get(nx, ny) == '.' {
                return Some((nx, ny, E));
            }
        }

        if from != S {
            let (nx, ny) = (x + 1, y);
            if board.get(nx, ny) == '.' {
                return Some((nx, ny, S));
            }
        }

        if from != W {
            let (nx, ny) = (x, y - 1);
            if board.get(nx, ny) == '.' {
                return Some((nx, ny, W));
            }
        }

        None
    };

    path.insert((x, y), s);
    if let Some((nx, ny, nd)) = get_next() {
        find_path(nx, ny, s + 1, nd, board, path);
    }
}

fn find_starting_direction(x: usize, y: usize, board: &Board) -> Direction {
    if board.get(x - 1, y) == '.' {
        return Direction::N;
    } else if board.get(x, y + 1) == '.' {
        return Direction::E;
    } else if board.get(x + 1, y) == '.' {
        return Direction::S;
    } else if board.get(x, y - 1) == '.' {
        return Direction::W;
    }

    unreachable!()
}

struct Board {
    buf: Vec<char>,
    w: usize,
}

impl Board {
    fn new(raw: &str) -> Board {
        let raw = raw.trim_end();

        let w = raw.find('\n').unwrap();

        let buf = raw
            .lines()
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>();

        Self { buf, w }
    }

    fn take_start(&mut self) -> (usize, usize) {
        let idx = self.buf.iter().position(|c| *c == 'S').unwrap();
        self.buf[idx] = '.';

        ((idx + 1) % self.w - 1, (idx + 1) / self.w)
    }

    fn take_end(&mut self) -> (usize, usize) {
        let idx = self.buf.iter().position(|c| *c == 'E').unwrap();
        self.buf[idx] = '.';

        ((idx + 1) % self.w - 1, (idx + 1) / self.w)
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.buf[self.w * y + x]
    }
}
