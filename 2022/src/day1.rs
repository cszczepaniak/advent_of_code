const GROUP_SEPARATOR: &'static str = "\n\n";

pub fn part_one(input: &str) -> usize {
    input.split(GROUP_SEPARATOR).map(sum_group).max().unwrap()
}

pub fn part_two(input: &str) -> usize {
    let mut top_three = [0usize; 3];
    for group_sum in input.split(GROUP_SEPARATOR).map(sum_group) {
        if group_sum > top_three[0] {
            top_three[2] = top_three[1];
            top_three[1] = top_three[0];
            top_three[0] = group_sum;
        } else if group_sum > top_three[1] {
            top_three[2] = top_three[1];
            top_three[1] = group_sum;
        } else if group_sum > top_three[2] {
            top_three[2] = group_sum;
        }
    }

    top_three.iter().sum()
}

fn sum_group(g: &str) -> usize {
    let mut sum: usize = 0;
    for l in g.lines() {
        sum += l.parse().unwrap_or(0);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = common::network::get_input(2022, 1).unwrap();

        assert_eq!(70116, part_one(&input));
    }

    #[test]
    fn test_part_two() {
        let input = common::network::get_input(2022, 1).unwrap();

        assert_eq!(206582, part_two(&input));
    }
}
