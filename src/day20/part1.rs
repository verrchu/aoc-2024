use std::collections::HashMap;

pub fn solution(input: &str) -> u64 {
    let mut board = Board::new(input);
    let (sx, sy) = board.take_start();
    // let (ex, ey) = board.take_end();

    let mut dp = HashMap::<(usize, usize), u64>::new();
    flood(sx, sy, 0, &board, &mut dp);

    // println!("{}", dp[&(ex, ey)]);

    let cheats = find_cheats(&board, &dp);
    // for (a, b) in cheats {
    //     println!("{a} -> {b}");
    // }

    cheats
        .into_iter()
        .filter_map(|(save, n)| (save >= 100).then_some(n))
        .sum()
}

fn find_cheats(board: &Board, dp: &HashMap<(usize, usize), u64>) -> HashMap<u64, u64> {
    #[derive(PartialEq)]
    enum Direction {
        N,
        E,
        S,
        W,
    }

    let get_cheat_score = |x, y, d| {
        let ((n1x, n1y), (n2x, n2y)) = match d {
            Direction::N => ((x - 1, y), (x as isize - 2, y as isize)),
            Direction::E => ((x, y + 1), (x as isize, y as isize + 2)),
            Direction::S => ((x + 1, y), (x as isize + 2, y as isize)),
            Direction::W => ((x, y - 1), (x as isize, y as isize - 2)),
        };

        if !(0..(board.w as isize)).contains(&n2y) || !(0..(board.h as isize)).contains(&n2x) {
            return None;
        }

        let (n2x, n2y) = (n2x as usize, n2y as usize);
        if !((board.get(n1x, n1y), board.get(n2x, n2y)) == ('#', '.')) {
            return None;
        }

        Some(dp[&(n2x, n2y)])
    };

    let mut cheats = HashMap::new();
    for ((x, y), score) in dp {
        use Direction::*;
        for d in [N, E, S, W] {
            if let Some(cheat_score) = get_cheat_score(*x, *y, d) {
                if cheat_score > score + 2 {
                    *(cheats.entry(cheat_score - (score + 2)).or_default()) += 1;
                }
            }
        }
    }

    cheats
}

fn flood(x: usize, y: usize, score: u64, board: &Board, dp: &mut HashMap<(usize, usize), u64>) {
    if board.get(x, y) == '#' {
        return;
    }

    let prev_score = dp.entry((x, y)).or_insert(u64::MAX);
    if score < *prev_score {
        *prev_score = score;
    } else {
        return;
    }

    flood(x + 1, y, score + 1, board, &mut *dp);
    flood(x, y + 1, score + 1, board, &mut *dp);
    flood(x - 1, y, score + 1, board, &mut *dp);
    flood(x, y - 1, score + 1, board, &mut *dp);
}

struct Board {
    buf: Vec<char>,
    w: usize,
    h: usize,
}

impl Board {
    fn new(raw: &str) -> Board {
        let raw = raw.trim_end();

        let w = raw.find('\n').unwrap();
        let h = (raw.len() + 1) / (w + 1);

        let buf = raw
            .lines()
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>();

        Self { buf, w, h }
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
