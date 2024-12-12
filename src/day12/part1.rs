use std::collections::HashSet;

#[derive(PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

pub fn solution(input: &str) -> u64 {
    let board = Board::new(input);

    let mut seen = HashSet::<(isize, isize)>::with_capacity((board.h * board.w) as usize);

    let mut result = 0;
    for r in 0..board.h {
        for c in 0..board.w {
            let target = board.get(r, c).unwrap();
            let (s, p) = board.walk(r, c, target, &mut seen);
            result += s * p;
        }
    }

    result
}

struct Board<'a> {
    buf: &'a [u8],
    w: isize,
    h: isize,
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
            w: w as isize,
            h: h as isize,
        }
    }

    fn get(&self, r: isize, c: isize) -> Option<u8> {
        ((0..self.h).contains(&r) && (0..self.w).contains(&c))
            .then(|| self.buf[((self.w + 1) * r + c) as usize])
    }

    fn walk(
        &self,
        r: isize,
        c: isize,
        target: u8,
        seen: &mut HashSet<(isize, isize)>,
    ) -> (u64, u64) {
        let Some(ch) = self.get(r, c) else {
            return (0, 0);
        };

        if target != ch || !seen.insert((r, c)) {
            return (0, 0);
        }

        let (mut s, mut p) = (1, 0);

        if self.should_put_fence(r, c, target, Direction::N) {
            p += 1;
        } else {
            let (ns, np) = self.walk(r - 1, c, target, &mut *seen);
            p += np;
            s += ns;
        }

        if self.should_put_fence(r, c, target, Direction::E) {
            p += 1;
        } else {
            let (ns, np) = self.walk(r, c + 1, target, &mut *seen);
            p += np;
            s += ns;
        }

        if self.should_put_fence(r, c, target, Direction::S) {
            p += 1;
        } else {
            let (ns, np) = self.walk(r + 1, c, target, &mut *seen);
            p += np;
            s += ns;
        }

        if self.should_put_fence(r, c, target, Direction::W) {
            p += 1;
        } else {
            let (ns, np) = self.walk(r, c - 1, target, &mut *seen);
            p += np;
            s += ns;
        }

        (s, p)
    }

    fn should_put_fence(&self, r: isize, c: isize, target: u8, direction: Direction) -> bool {
        match direction {
            Direction::N => self.get(r - 1, c) != Some(target),
            Direction::E => self.get(r, c + 1) != Some(target),
            Direction::S => self.get(r + 1, c) != Some(target),
            Direction::W => self.get(r, c - 1) != Some(target),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::day12::{EXAMPLE11, EXAMPLE12, EXAMPLE13};

    #[test]
    fn examplp11() {
        assert_eq!(solution(EXAMPLE11), 140);
    }

    #[test]
    fn examplp12() {
        assert_eq!(solution(EXAMPLE12), 772);
    }

    #[test]
    fn examplp13() {
        assert_eq!(solution(EXAMPLE13), 1930);
    }
}
