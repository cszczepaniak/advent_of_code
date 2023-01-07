pub fn part_one(input: &str) -> Option<usize> {
    find_marker(input.as_bytes(), 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    find_marker(input.as_bytes(), 14)
}

fn find_marker(input: &[u8], length: usize) -> Option<usize> {
    let mut idx = 0;
    while let Some(slice) = input.get(idx..idx + length) {
        let mut state = 0u32;

        if let Some(pos) = slice.iter().rposition(|byte| {
            let bit_idx = byte % 32;
            let ret = state & (1 << bit_idx) != 0;
            state |= 1 << bit_idx;
            ret
        }) {
            idx += pos + 1;
        } else {
            return Some(idx + length);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = common::network::get_input(2022, 6).unwrap();

        assert_eq!(1766, part_one(&input).unwrap());
    }

    #[test]
    fn part_two_test() {
        let input = common::network::get_input(2022, 6).unwrap();

        assert_eq!(2383, part_two(&input).unwrap());
    }

    #[test]
    fn test_part_one_examples() {
        assert_eq!(
            5,
            find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(), 4).unwrap()
        );
        assert_eq!(
            6,
            find_marker("nppdvjthqldpwncqszvftbrmjlhg".as_bytes(), 4).unwrap()
        );
        assert_eq!(
            10,
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes(), 4).unwrap()
        );
        assert_eq!(
            11,
            find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes(), 4).unwrap()
        );
    }

    #[test]
    fn test_part_two_examples() {
        assert_eq!(
            19,
            find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes(), 14).unwrap()
        );
        assert_eq!(
            23,
            find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(), 14).unwrap()
        );
        assert_eq!(
            23,
            find_marker("nppdvjthqldpwncqszvftbrmjlhg".as_bytes(), 14).unwrap()
        );
    }
}
