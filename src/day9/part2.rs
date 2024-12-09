use std::collections::{BTreeSet, HashMap};

pub fn solution(input: &str) -> usize {
    let disk = input.trim().chars().map(|ch| ch.to_digit(10).unwrap());

    let mut files = Vec::<(usize, usize, u32)>::new();
    let mut holes = HashMap::<u32, BTreeSet<usize>>::new();

    let mut idx = 0;
    for (i, n) in disk.enumerate() {
        if i % 2 == 0 {
            let file_id = files.len();
            files.push((idx, file_id, n));
        } else if n > 0 {
            holes.entry(n).or_default().insert(idx);
        }

        idx += n as usize;
    }

    for (file_idx, _file_id, file_size) in files.iter_mut().rev() {
        let hole = holes
            .iter()
            .filter(|(hole_size, _hole_idxs)| *hole_size >= file_size)
            .filter_map(|(hole_size, hole_idxs)| {
                hole_idxs.first().map(|hole_idx| (hole_size, hole_idx))
            })
            .filter(|(_hole_size, hole_idx)| *hole_idx < file_idx)
            .min_by_key(|(_hole_size, hole_idx)| *hole_idx)
            .map(|(hole_size, hole_idx)| (*hole_size, *hole_idx));

        if let Some((hole_size, hole_idx)) = hole {
            holes.get_mut(&hole_size).unwrap().pop_first().unwrap();

            *file_idx = hole_idx;
            if hole_size > *file_size {
                let size_diff = hole_size - *file_size;
                holes
                    .entry(size_diff)
                    .or_default()
                    .insert(hole_idx + *file_size as usize);
            }
        }
    }

    files.sort_by_key(|(file_idx, _file_id, _file_size)| *file_idx);

    files
        .into_iter()
        .map(|(file_idx, file_id, file_size)| {
            let avg_idx = (file_idx as f64 + (file_idx + file_size as usize - 1) as f64) / 2.0;
            (avg_idx * file_size as f64).round() as usize * file_id
        })
        .sum()
}