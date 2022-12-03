use std::{collections::HashSet, str::FromStr};

fn main() -> anyhow::Result<()> {
    let part_one = common::parse_input_lines::<RucksackHalves, anyhow::Error>("./input/day3.txt")?
        .iter()
        .map(|r| r.priority_of_shared())
        .sum::<Result<usize, _>>()?;

    println!("part 1: {part_one}");

    let part_two = common::parse_input_lines::<Rucksack, anyhow::Error>("./input/day3.txt")?
        .chunks(3)
        .map(|ch| priority_for_chunk(ch))
        .sum::<Result<usize, _>>()?;

    println!("part 2: {part_two}");

    Ok(())
}

fn priority_for_chunk(ch: &[Rucksack]) -> anyhow::Result<usize> {
    let mut res = ch
        .get(0)
        .ok_or(anyhow::anyhow!("expected three things in chunk"))?
        .contents
        .clone();

    for r in ch.iter().skip(1) {
        res.retain(|c| r.contents.contains(c));
    }

    if res.len() != 1 {
        anyhow::bail!("expected only one shared item");
    }

    let shared = res.iter().next().unwrap();
    Ok(priority(*shared))
}

fn priority(ch: char) -> usize {
    if ch.is_ascii_lowercase() {
        ch as usize - 'a' as usize + 1
    } else {
        ch as usize - 'A' as usize + 1 + 26
    }
}

struct RucksackHalves {
    first_half: HashSet<char>,
    second_half: HashSet<char>,
}

impl RucksackHalves {
    fn priority_of_shared(&self) -> anyhow::Result<usize> {
        let shared: Vec<&char> = self.first_half.intersection(&self.second_half).collect();
        if shared.len() != 1 {
            anyhow::bail!("expected only one shared item");
        }

        let shared = shared.get(0).unwrap();
        Ok(priority(**shared))
    }
}

impl FromStr for RucksackHalves {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 != 0 {
            anyhow::bail!("invalid input length");
        }

        let first_half: HashSet<_> = s[..s.len() / 2].chars().collect();
        let second_half: HashSet<_> = s[s.len() / 2..].chars().collect();

        Ok(RucksackHalves {
            first_half,
            second_half,
        })
    }
}

#[derive(Debug)]
struct Rucksack {
    contents: HashSet<char>,
}

impl FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rucksack {
            contents: s.chars().collect(),
        })
    }
}
