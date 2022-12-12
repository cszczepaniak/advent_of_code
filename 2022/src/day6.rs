use std::collections::HashSet;

pub fn part_one(input: &str) -> anyhow::Result<usize> {
    find_marker_optimized(input, 4).ok_or(anyhow::anyhow!("didn't find a marker"))
}

pub fn part_two(input: &str) -> anyhow::Result<usize> {
    find_marker_optimized(input, 14).ok_or(anyhow::anyhow!("didn't find a marker"))
}

#[allow(dead_code)]
fn find_marker(input: &str, n: usize) -> Option<usize> {
    let cs: Vec<_> = input.chars().collect();
    for (i, w) in cs.windows(n).enumerate() {
        if HashSet::<&char>::from_iter(w.iter()).len() == n {
            return Some(i + n);
        }
    }
    None
}

fn find_marker_optimized(input: &str, n: usize) -> Option<usize> {
    for (i, _) in input.char_indices() {
        if i < n {
            continue;
        }

        let window = &input[i.saturating_sub(n - 1)..=i];
        if all_unique(window) {
            return Some(i + 1);
        }
    }

    None
}

fn all_unique(s: &str) -> bool {
    let mut bits = 0u32;
    for c in s.chars() {
        let idx = c as u32 - 'a' as u32;
        let bit = 1 << idx;
        if bits & bit > 0 {
            return false;
        }
        bits |= 1 << idx;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{find_marker, find_marker_optimized};

    #[test]
    fn test_part_one_examples() {
        assert_eq!(5, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap());
        assert_eq!(6, find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap());
        assert_eq!(
            10,
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap()
        );
        assert_eq!(
            11,
            find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap()
        );
    }

    #[test]
    fn test_part_one_examples_opt() {
        assert_eq!(
            5,
            find_marker_optimized("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap()
        );
        assert_eq!(
            6,
            find_marker_optimized("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap()
        );
        assert_eq!(
            10,
            find_marker_optimized("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap()
        );
        assert_eq!(
            11,
            find_marker_optimized("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap()
        );
    }

    #[test]
    fn test_part_two_examples() {
        assert_eq!(
            19,
            find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap()
        );
        assert_eq!(23, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap());
        assert_eq!(23, find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14).unwrap());
    }
}
