use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

pub fn solution(input: &str) -> u64 {
    let mut board = Board::new(input);
    let (sr, sc) = board.take_start();
    let (er, ec) = board.take_end();

    let mut dp = HashMap::<(usize, usize), u64>::from_iter([((sr, sc), 0)]);

    search(sr, sc, Direction::E, &board, &mut dp);

    dp[&(er, ec)]
}

fn search(r: usize, c: usize, d: Direction, board: &Board, dp: &mut HashMap<(usize, usize), u64>) {
    let s = dp[&(r, c)];

    let get_score = |d, dp: &mut HashMap<(usize, usize), u64>| {
        let (nr, nc) = match d {
            Direction::N => (r - 1, c),
            Direction::E => (r, c + 1),
            Direction::S => (r + 1, c),
            Direction::W => (r, c - 1),
        };

        if board.get(nr, nc) == '.' {
            let s = dp.entry((nr, nc)).or_insert(u64::MAX);
            Some((nr, nc, *s))
        } else {
            None
        }
    };

    let search_further = |nd: Direction, dp: &mut HashMap<(usize, usize), u64>| {
        let is_turn = d != nd;
        let diff = if is_turn { 1001 } else { 1 };

        if let Some((nr, nc, ns)) = get_score(nd, &mut *dp) {
            if ns > s + diff {
                dp.insert((nr, nc), s + diff);
                search(nr, nc, nd, board, &mut *dp);
            }
        }
    };

    match d {
        Direction::N => {
            search_further(Direction::E, &mut *dp);
            search_further(Direction::W, &mut *dp);
            search_further(Direction::N, &mut *dp);
        }
        Direction::E => {
            search_further(Direction::S, &mut *dp);
            search_further(Direction::E, &mut *dp);
            search_further(Direction::N, &mut *dp);
        }
        Direction::S => {
            search_further(Direction::W, &mut *dp);
            search_further(Direction::S, &mut *dp);
            search_further(Direction::E, &mut *dp);
        }
        Direction::W => {
            search_further(Direction::S, &mut *dp);
            search_further(Direction::W, &mut *dp);
            search_further(Direction::N, &mut *dp);
        }
    }
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

    fn get(&self, r: usize, c: usize) -> char {
        self.buf[self.w * r + c]
    }
}
