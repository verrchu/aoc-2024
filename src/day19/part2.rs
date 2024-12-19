use std::collections::HashMap;

#[derive(Default, Debug)]
struct Node {
    next: HashMap<u8, Node>,
    end: bool,
}

impl Node {
    fn new<'a>(items: impl Iterator<Item = &'a [u8]>) -> Self {
        let mut root = Self::default();

        for item in items {
            root.insert(item);
        }

        root
    }

    fn insert(&mut self, item: &[u8]) {
        if let Some((head, tail)) = item.split_first() {
            let next = self.next.entry(*head).or_default();
            next.insert(tail);
        } else {
            self.end = true;
        }
    }

    fn prefixes(&self, pattern: &[u8]) -> Vec<usize> {
        let mut idx = 0;
        let mut idxs = vec![];

        let mut node = self;
        for c in pattern {
            if let Some(next) = node.next.get(c) {
                node = next;
                idx += 1;
            } else {
                break;
            }

            if node.end {
                idxs.push(idx);
            }
        }

        idxs
    }
}

pub fn solution(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();

    let towels = Node::new(towels.split(", ").map(|t| t.as_bytes()));

    patterns
        .lines()
        .filter_map(|p| {
            let mut memo = HashMap::new();
            arrangements(p.as_bytes(), &towels, &mut memo)
        })
        .sum()
}

fn arrangements(
    pattern: &[u8],
    towels: &Node,
    memo: &mut HashMap<usize, Option<usize>>,
) -> Option<usize> {
    if pattern.is_empty() {
        return Some(1);
    }

    if let Some(result) = memo.get(&pattern.len()).copied() {
        return result;
    }

    let prefixes = towels.prefixes(pattern);

    let result = prefixes
        .into_iter()
        .filter_map(|prefix| {
            let (_, pattern) = pattern.split_at(prefix);
            let result = arrangements(pattern, towels, &mut *memo);

            memo.insert(pattern.len(), result);

            result
        })
        .sum();

    (result > 0).then_some(result)
}
