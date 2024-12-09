use std::cell::RefCell;

pub fn solution(input: &str) -> usize {
    let disk = input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .enumerate()
        .flat_map(|(i, n)| {
            let elem = (i % 2 == 0)
                .then(|| RefCell::new(Some(i / 2)))
                .unwrap_or(RefCell::new(None));

            (0..n).map(move |_| elem.clone())
        })
        .collect::<Vec<_>>();

    'outer: for (ri, rn) in disk.iter().enumerate().rev() {
        if rn.borrow().is_none() {
            continue 'outer;
        }

        'inner: for (li, ln) in disk.iter().enumerate() {
            if li >= ri {
                break 'outer;
            }

            if ln.borrow().is_some() {
                continue 'inner;
            }

            ln.swap(rn);
            break 'inner;
        }
    }

    disk.into_iter()
        .take_while(|n| n.borrow().is_some())
        .map(|n| n.take().unwrap())
        .enumerate()
        .map(|(i, n)| i * n)
        .sum()
}
