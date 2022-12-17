pub fn part_one(input: &str) -> anyhow::Result<usize> {
    let mut res = 0;
    for l in input.lines() {
        let w1 = word_to_bits(&l[..l.len() / 2]);
        let w2 = word_to_bits(&l[l.len() / 2..]);
        res += priority_for_bits(w1 & w2);
    }
    Ok(res)
}

pub fn part_two(input: &str) -> anyhow::Result<usize> {
    let mut ws = [0u64; 3];
    let mut res = 0;
    for (i, l) in input.lines().enumerate() {
        ws[i % 3] = word_to_bits(l);
        if (i + 1) % 3 == 0 {
            res += priority_for_bits(ws[0] & ws[1] & ws[2]);
        }
    }
    Ok(res)
}

fn word_to_bits(w: &str) -> u64 {
    let mut res = 0;
    for c in w.chars() {
        res |= 1 << char_to_usize(c);
    }

    res
}

fn char_to_usize(c: char) -> usize {
    if c as usize >= 'a' as usize && c as usize <= 'a' as usize {
        c as usize - 'a' as usize
    } else {
        c as usize - 'A' as usize
    }
}

fn priority_for_bits(bits: u64) -> usize {
    bits.trailing_zeros() as usize + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parity() {
        let first_half = "vJrwpWtwJgWr";
        let second_half = "hcsFMMfFFhFp";

        let priority = priority_for_bits(word_to_bits(first_half) & word_to_bits(second_half));

        assert_eq!('p' as usize - 'a' as usize + 1, priority);
    }

    #[test]
    fn test_part_one() {
        let input = common::network::get_input(2022, 3).unwrap();

        assert_eq!(7826, part_one(&input).unwrap());
    }

    #[test]
    fn test_part_two() {
        let input = common::network::get_input(2022, 3).unwrap();

        assert_eq!(2577, part_two(&input).unwrap());
    }
}
