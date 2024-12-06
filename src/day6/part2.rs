use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn is_south(&self) -> bool {
        matches!(self, Self::S)
    }

    fn is_north(&self) -> bool {
        matches!(self, Self::N)
    }

    fn is_east(&self) -> bool {
        matches!(self, Self::E)
    }

    fn is_west(&self) -> bool {
        matches!(self, Self::W)
    }

    fn turn_right(self) -> Self {
        match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }
}

pub fn solution(input: &str) -> usize {
    let mut board = Board::new(input);

    let mut dir = Direction::N;
    let mut regular_path = HashSet::<(usize, usize)>::new();

    let start = board.starting_position();

    let mut pos = start;
    loop {
        regular_path.insert(pos);

        if board.is_leaving(pos.0, pos.1, dir) {
            break;
        }

        (pos, dir) = board.next(pos.0, pos.1, dir);
    }

    let mut loops = 0;
    for pos in regular_path.iter().copied() {
        if pos == start {
            continue;
        }

        board.set_extra_obstacle(pos.0, pos.1);

        let mut augmented_path =
            HashSet::<((usize, usize), Direction)>::with_capacity(regular_path.len());

        let mut dir = Direction::N;
        let mut pos = start;

        loop {
            if !augmented_path.insert((pos, dir)) {
                loops += 1;
                break;
            }

            if board.is_leaving(pos.0, pos.1, dir) {
                break;
            }

            (pos, dir) = board.next(pos.0, pos.1, dir);
        }
    }

    loops
}

struct Board<'a> {
    buf: &'a [u8],
    extra_obstacle: Option<(usize, usize)>,
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

        Self {
            buf,
            w,
            h,
            extra_obstacle: None,
        }
    }

    fn get(&self, r: usize, c: usize) -> char {
        debug_assert!((0..self.h).contains(&r));
        debug_assert!((0..self.w).contains(&c));

        self.buf[(self.w + 1) * r + c] as char
    }

    fn starting_position(&self) -> (usize, usize) {
        let idx = self.buf.iter().position(|ch| *ch == b'^').unwrap();
        let r = idx / (self.w + 1);
        let c = idx % (self.w + 1);

        (r, c)
    }

    fn next(&self, r: usize, c: usize, dir: Direction) -> ((usize, usize), Direction) {
        let (next_r, next_c) = match dir {
            Direction::N => (r - 1, c),
            Direction::E => (r, c + 1),
            Direction::S => (r + 1, c),
            Direction::W => (r, c - 1),
        };

        let mut candidate = self.get(next_r, next_c);
        if Some((next_r, next_c)) == self.extra_obstacle {
            candidate = '#';
        }

        if candidate == '#' {
            ((r, c), dir.turn_right())
        } else {
            ((next_r, next_c), dir)
        }
    }

    fn is_leaving(&self, r: usize, c: usize, dir: Direction) -> bool {
        (dir.is_north() && r == 0)
            || (dir.is_south() && r == self.h - 1)
            || (dir.is_west() && c == 0)
            || (dir.is_east() && c == self.w - 1)
    }

    fn set_extra_obstacle(&mut self, r: usize, c: usize) {
        self.extra_obstacle = Some((r, c));
    }
}
