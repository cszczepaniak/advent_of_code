pub fn part_one(input: &str) -> usize {
    todo!()
}

pub fn part_two(input: &str) -> usize {
    todo!()
}

struct Pair {
    left: Item,
    right: Item,
}

enum Item {
    Number(usize),
    List(Vec<usize>),
}

fn compare_int(l: usize, r: usize) -> Option<bool> {
    if l < r {
        Some(true)
    } else if l > r {
        Some(false)
    } else {
        None
    }
}

fn compare_list(
    l: impl Iterator<Item = usize>,
    mut r: impl Iterator<Item = usize>,
) -> Option<bool> {
    for l_n in l {
        if let Some(r_n) = r.next() {
            if let Some(res) = compare_int(l_n, r_n) {
                return Some(res);
            }
        } else {
            // The right iterator ran out of items first; the items are not in the right order
            return Some(false);
        }
    }

    if let Some(_) = r.next() {
        // The left iterator ran out of items first; the items are in the right order
        return Some(true);
    }

    // The lists were identical. No conclusion.
    None
}
