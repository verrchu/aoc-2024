use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(PartialEq, Eq, Hash)]
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
            let mut fence = HashMap::<(isize, Direction), BTreeSet<isize>>::with_capacity(
                (board.h * board.w) as usize,
            );
            let s = board.walk(r, c, target, &mut fence, &mut seen);
            let p = count_fence_sides(&fence);
            result += s * p;
        }
    }

    result
}

fn count_fence_sides(fence: &HashMap<(isize, Direction), BTreeSet<isize>>) -> u64 {
    let mut sides = 0;
    for line in fence.values() {
        let mut line = line.iter();
        let mut line_sides = 1;
        let mut prev = line.next().unwrap();
        for next in line {
            if next - prev > 1 {
                line_sides += 1;
            }

            prev = next;
        }

        sides += line_sides;
    }

    sides
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
        fence: &mut HashMap<(isize, Direction), BTreeSet<isize>>,
        seen: &mut HashSet<(isize, isize)>,
    ) -> u64 {
        let Some(ch) = self.get(r, c) else {
            return 0;
        };

        if target != ch || !seen.insert((r, c)) {
            return 0;
        }

        let mut s = 1;

        if self.should_put_fence(r, c, target, Direction::N) {
            fence.entry((r, Direction::N)).or_default().insert(c);
        } else {
            s += self.walk(r - 1, c, target, &mut *fence, &mut *seen);
        }

        if self.should_put_fence(r, c, target, Direction::E) {
            fence.entry((c, Direction::E)).or_default().insert(r);
        } else {
            s += self.walk(r, c + 1, target, &mut *fence, &mut *seen);
        }

        if self.should_put_fence(r, c, target, Direction::S) {
            fence.entry((r, Direction::S)).or_default().insert(c);
        } else {
            s += self.walk(r + 1, c, target, &mut *fence, &mut *seen);
        }

        if self.should_put_fence(r, c, target, Direction::W) {
            fence.entry((c, Direction::W)).or_default().insert(r);
        } else {
            s += self.walk(r, c - 1, target, &mut *fence, &mut *seen);
        }

        s
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

    use crate::day12::{EXAMPLE11, EXAMPLE12, EXAMPLE21, EXAMPLE22};

    #[test]
    fn examplp11() {
        assert_eq!(solution(EXAMPLE11), 80);
    }

    #[test]
    fn examplp12() {
        assert_eq!(solution(EXAMPLE12), 436);
    }

    #[test]
    fn examplp21() {
        assert_eq!(solution(EXAMPLE21), 236);
    }

    #[test]
    fn examplp22() {
        assert_eq!(solution(EXAMPLE22), 368);
    }
}
