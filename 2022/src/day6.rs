pub fn part_one(input: &str) -> anyhow::Result<usize> {
    find_marker(input, 4).ok_or(anyhow::anyhow!("didn't find a marker"))
}

pub fn part_two(input: &str) -> anyhow::Result<usize> {
    find_marker(input, 14).ok_or(anyhow::anyhow!("didn't find a marker"))
}

fn find_marker(input: &str, n: usize) -> Option<usize> {
    let mut u = Uniquer::new(n);

    for (i, c) in input.chars().enumerate() {
        if i >= n {
            let leaving = input.as_bytes()[i - n] as char;
            u.remove(leaving);
        }
        u.add(c);

        if u.unique() {
            return Some(i + 1);
        }
    }

    None
}

struct Uniquer {
    state: [usize; 26],
    num_greater_than_one: usize,
    n: usize,
}

impl Uniquer {
    fn new(n: usize) -> Self {
        Self {
            state: [0; 26],
            num_greater_than_one: 0,
            n,
        }
    }

    fn add(&mut self, c: char) {
        let idx = c as usize - 'a' as usize;
        if self.state[idx] == 1 {
            self.num_greater_than_one += 1;
        }
        self.state[idx] += 1;
    }

    fn remove(&mut self, c: char) {
        let idx = c as usize - 'a' as usize;
        if self.state[idx] == 2 {
            self.num_greater_than_one -= 1;
        }
        self.state[idx] -= 1;
    }

    fn unique(&self) -> bool {
        if self.num_greater_than_one > 0 {
            return false;
        }
        let mut sum = 0;
        for count in self.state.iter() {
            if count > &1 {
                return false;
            }
            sum += count;
        }
        sum == self.n
    }
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
    fn test_part_two_examples() {
        assert_eq!(
            19,
            find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap()
        );
        assert_eq!(23, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap());
        assert_eq!(23, find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14).unwrap());
    }
}
