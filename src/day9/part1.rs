use std::cmp::Ordering;
use std::collections::{BTreeMap, VecDeque};

pub fn solution(input: &str) -> usize {
    let disk = input.trim().chars().map(|ch| ch.to_digit(10).unwrap());

    let mut files = BTreeMap::<usize, (usize, u32)>::new();
    let mut holes = VecDeque::<(usize, u32)>::new();

    let mut idx = 0;
    for (i, n) in disk.enumerate() {
        if i % 2 == 0 {
            let file_id = files.len();
            files.insert(idx, (file_id, n));
        } else if n > 0 {
            holes.push_back((idx, n));
        }

        idx += n as usize;
    }

    loop {
        let Some(mut rightmost_file) = files.last_entry() else {
            break;
        };

        let Some((hole_idx, hole_size)) = holes.front_mut() else {
            break;
        };

        let file_idx = *rightmost_file.key();
        let (file_id, file_size) = rightmost_file.get_mut();

        if *hole_idx >= file_idx {
            break;
        }

        match hole_size.cmp(&file_size) {
            Ordering::Less => {
                *file_size -= *hole_size;

                let (file_id, _file_size) = (*file_id, *file_size);
                files.insert(*hole_idx, (file_id, *hole_size));

                holes.pop_front().unwrap();
            }
            Ordering::Equal => {
                let (file_id, file_size) = (*file_id, *file_size);

                files.pop_last().unwrap();
                files.insert(*hole_idx, (file_id, file_size));

                holes.pop_front().unwrap();
            }
            Ordering::Greater => {
                let (file_id, file_size) = (*file_id, *file_size);

                files.pop_last().unwrap();
                files.insert(*hole_idx, (file_id, file_size));

                *hole_idx += file_size as usize;
                *hole_size -= file_size;
            }
        }
    }

    files
        .into_iter()
        .map(|(file_idx, (file_id, file_size))| {
            let avg_idx = (file_idx as f64 + (file_idx + file_size as usize - 1) as f64) / 2.0;
            (avg_idx * file_size as f64).round() as usize * file_id
        })
        .sum()
}
