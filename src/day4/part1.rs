static TO_SEARCH: [char; 3] = ['M', 'A', 'S'];

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn is_south(&self) -> bool {
        matches!(self, Self::S | Self::SE | Self::SW)
    }

    fn is_north(&self) -> bool {
        matches!(self, Self::N | Self::NE | Self::NW)
    }

    fn is_east(&self) -> bool {
        matches!(self, Self::E | Self::NE | Self::SE)
    }

    fn is_west(&self) -> bool {
        matches!(self, Self::W | Self::NW | Self::SW)
    }
}

pub fn solution(input: &str) -> u64 {
    let board = Board::new(input);

    let mut found = 0;
    for r in 0..board.h {
        for c in 0..board.w {
            let ch = board.get(r, c);

            if ch == 'X' {
                found += board.search(r, c, Direction::N) as u64;
                found += board.search(r, c, Direction::NE) as u64;
                found += board.search(r, c, Direction::E) as u64;
                found += board.search(r, c, Direction::SE) as u64;
                found += board.search(r, c, Direction::S) as u64;
                found += board.search(r, c, Direction::SW) as u64;
                found += board.search(r, c, Direction::W) as u64;
                found += board.search(r, c, Direction::NW) as u64;
            }
        }
    }

    found
}

struct Board<'a> {
    buf: &'a [u8],
    w: usize,
    h: usize,
}

impl<'buf> Board<'buf> {
    fn new(buf: &'buf str) -> Board<'buf> {
        let buf = buf.trim_end();

        let w = buf.find('\n').unwrap();
        let h = (buf.len() + 1) / (w + 1);

        debug_assert_eq!((buf.len() + 1) % (w + 1), 0);

        let buf = buf.as_bytes();
        Self { buf, w, h }
    }

    fn get(&self, r: usize, c: usize) -> char {
        debug_assert!((0..self.h).contains(&r));
        debug_assert!((0..self.w).contains(&c));

        self.buf[(self.w + 1) * r + c] as char
    }

    fn search(&self, mut r: usize, mut c: usize, dir: Direction) -> bool {
        for ch in TO_SEARCH.iter() {
            let Some((next_r, next_c)) = self.get_next(r, c, dir) else {
                return false;
            };

            r = next_r;
            c = next_c;

            if self.get(r, c) != *ch {
                return false;
            }
        }

        true
    }

    fn get_next(&self, r: usize, c: usize, dir: Direction) -> Option<(usize, usize)> {
        let (mut r, mut c) = (r as isize, c as isize);

        if dir.is_south() {
            r += 1
        };

        if dir.is_north() {
            r -= 1;
        }

        if dir.is_east() {
            c += 1;
        }

        if dir.is_west() {
            c -= 1;
        }

        if !((0..(self.w as isize)).contains(&c)) {
            return None;
        }

        if !((0..(self.h as isize)).contains(&r)) {
            return None;
        }

        Some((r as usize, c as usize))
    }
}
