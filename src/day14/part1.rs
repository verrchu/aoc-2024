const SECS: isize = 100;
const W: isize = 101;
const H: isize = 103;

pub fn solution(input: &str) -> u64 {
    let (mut nw, mut ne, mut sw, mut se) = (0, 0, 0, 0);
    for line in input.lines() {
        let (pos, vel) = line.split_once(' ').unwrap();

        let pos = pos.strip_prefix("p=").unwrap();
        let (px, py) = pos.split_once(',').unwrap();

        let px = px.parse::<isize>().unwrap();
        let py = py.parse::<isize>().unwrap();

        let vel = vel.strip_prefix("v=").unwrap();
        let (vx, vy) = vel.split_once(',').unwrap();

        let vx = vx.parse::<isize>().unwrap();
        let vy = vy.parse::<isize>().unwrap();

        let mut rx = (px + vx * SECS) % W;
        if rx < 0 {
            rx += W
        };

        let mut ry = (py + vy * SECS) % H;
        if ry < 0 {
            ry += H;
        }

        if rx < W / 2 {
            if ry < H / 2 {
                nw += 1;
            } else if ry > H / 2 {
                ne += 1;
            }
        } else if rx > W / 2 {
            if ry < H / 2 {
                sw += 1;
            } else if ry > H / 2 {
                se += 1;
            }
        }
    }

    nw * ne * sw * se
}
