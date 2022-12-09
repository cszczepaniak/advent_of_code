use std::str::FromStr;

use common::runner_main;

const GROUP_SEPARATOR: &'static str = "\n\n";

runner_main!(2022, day 1, part1: part_one, part2: part_two);

fn part_one(input: &str) -> anyhow::Result<usize> {
    let group_sums = parse_and_sort(input);
    Ok(group_sums[0])
}

fn part_two(input: &str) -> anyhow::Result<usize> {
    let group_sums = parse_and_sort(input);
    Ok(group_sums.iter().take(3).sum())
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
