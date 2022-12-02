use std::str::FromStr;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn main() -> anyhow::Result<()> {
    let two_new_lines = LINE_ENDING.to_string() + LINE_ENDING;
    let mut group_sums: Vec<usize> =
        advent::parse_input_delim::<Group, anyhow::Error>("./input/day1.txt", &two_new_lines)?
            .iter()
            .map(|g| g.0)
            .collect();

    group_sums.sort_by(|a, b| b.cmp(a));

    println!("part 1: {}", group_sums[0]);
    println!("part 2: {}", group_sums.iter().take(3).sum::<usize>());

    Ok(())
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
