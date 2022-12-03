use std::{collections::HashSet, fs};

fn main() -> anyhow::Result<()> {
    let part_one = fs::read_to_string("./input/day3.txt")?
        .lines()
        .map(|l| {
            [
                HashSet::from_iter(l[..l.len() / 2].chars()),
                HashSet::from_iter(l[l.len() / 2..].chars()),
            ]
        })
        .map(|ch| priority_for_chunk(&ch))
        .sum::<Result<usize, _>>()?;

    println!("part 1: {part_one}");

    let part_two = fs::read_to_string("./input/day3.txt")?
        .lines()
        .map(|l| HashSet::from_iter(l.chars()))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(priority_for_chunk)
        .sum::<Result<usize, _>>()?;

    println!("part 2: {part_two}");

    Ok(())
}

fn priority_for_chunk(ch: &[HashSet<char>]) -> anyhow::Result<usize> {
    let mut res = ch
        .get(0)
        .ok_or(anyhow::anyhow!("expected at least one thing in chunk"))?
        .clone();

    for r in ch.iter().skip(1) {
        res.retain(|c| r.contains(c));
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
