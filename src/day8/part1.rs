use std::collections::{HashMap, HashSet};

pub fn solution(input: &str) -> usize {
    let board = Board::new(input);

    let mut antennas = HashMap::<char, Vec<(usize, usize)>>::new();
    for r in 0..board.h {
        for c in 0..board.w {
            let ch = board.get(r, c);
            if ch != '.' {
                antennas.entry(ch).or_default().push((r, c));
            }
        }
    }

    let mut antinodes = HashSet::<(usize, usize)>::new();
    for mut antennas in antennas.values().map(Vec::as_slice) {
        while let Some((head, tail)) = antennas.split_first() {
            let (ar, ac) = head;
            for (br, bc) in tail {
                let (ar, ac, br, bc) = (*ar as isize, *ac as isize, *br as isize, *bc as isize);

                let diff_r = br - ar;
                let diff_c = bc - ac;

                let (next_r, next_c) = (br + diff_r, bc + diff_c);
                let (prev_r, prev_c) = (ar - diff_r, ac - diff_c);

                if board.is_within_bounds(next_r, next_c) {
                    antinodes.insert((next_r as usize, next_c as usize));
                }

                if board.is_within_bounds(prev_r, prev_c) {
                    antinodes.insert((prev_r as usize, prev_c as usize));
                }
            }

            antennas = tail;
        }
    }

    antinodes.len()
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

    fn is_within_bounds(&self, r: isize, c: isize) -> bool {
        r >= 0 && c >= 0 && r < self.h as isize && c < self.w as isize
    }
}
