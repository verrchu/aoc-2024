use std::collections::{HashMap, HashSet};

// type Memo = HashMap<(usize, usize), Rc<RefCell<HashSet<(usize, usize)>>>>;
type Memo = HashMap<(usize, usize), HashSet<(usize, usize)>>;

pub fn solution(input: &str) -> usize {
    let board = Board::new(input);

    let mut memo = Memo::new();

    let mut result = 0;
    for r in 0..board.h {
        for c in 0..board.w {
            let ch = board.get(r, c);
            if ch == 0 {
                result += board.search(r, c, ch, &mut memo).len();
            }
        }
    }

    result
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

    fn get(&self, r: usize, c: usize) -> u32 {
        debug_assert!((0..self.h).contains(&r));
        debug_assert!((0..self.w).contains(&c));

        (self.buf[(self.w + 1) * r + c] as char)
            .to_digit(10)
            .unwrap()
    }

    fn search(&self, r: usize, c: usize, ch: u32, memo: &mut Memo) -> HashSet<(usize, usize)> {
        if self.get(r, c) != ch {
            return HashSet::new();
        }

        if let Some(reachable) = memo.get(&(r, c)).cloned() {
            return reachable;
        }

        if ch == 9 {
            HashSet::from([(r, c)])
        } else {
            let next = ch + 1;

            let mut reachable = HashSet::new();

            if c < (self.w - 1) {
                reachable.extend(self.search(r, c + 1, next, &mut *memo));
            }

            if c > 0 {
                reachable.extend(self.search(r, c - 1, next, &mut *memo));
            }

            if r < (self.h - 1) {
                reachable.extend(self.search(r + 1, c, next, &mut *memo));
            }

            if r > 0 {
                reachable.extend(self.search(r - 1, c, next, &mut *memo));
            }

            if ch != 0 {
                memo.insert((r, c), reachable.clone());
            }

            reachable
        }
    }
}
