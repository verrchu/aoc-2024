static SIDES: [(char, char); 2] = [('M', 'S'), ('S', 'M')];

pub fn solution(input: &str) -> u64 {
    let board = Board::new(input);

    let mut found = 0;
    for r in 1..(board.h - 1) {
        for c in 1..(board.w - 1) {
            let ch = board.get(r, c);

            if ch == 'A' {
                let ne = board.get(r - 1, c + 1);
                let nw = board.get(r - 1, c - 1);
                let se = board.get(r + 1, c + 1);
                let sw = board.get(r + 1, c - 1);

                if SIDES.contains(&(ne, sw)) && SIDES.contains(&(se, nw)) {
                    found += 1;
                }
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
}
