use std::str::FromStr;

const GROUP_SEPARATOR: &'static str = "\n\n";

pub fn part_one(input: &str) -> usize {
    parse_and_sort(input)[0]
}

pub fn part_two(input: &str) -> usize {
    parse_and_sort(input).iter().take(3).sum()
}

fn parse_and_sort(input: &str) -> Vec<usize> {
    let mut group_sums: Vec<usize> = input
        .split(GROUP_SEPARATOR)
        .map(|l| l.parse::<Group>().expect("bad puzzle input"))
        .map(|g| g.0)
        .collect();

    group_sums.sort_by(|a, b| b.cmp(a));

    group_sums
}

struct Group(usize);

impl FromStr for Group {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s
            .trim()
            .lines()
            .map(|l| {
                l.parse::<usize>()
                    .map_err(|_| anyhow::anyhow!("failed to parse int"))
            })
            .sum::<Result<usize, _>>()?;

        Ok(Self(res))
    }
}
