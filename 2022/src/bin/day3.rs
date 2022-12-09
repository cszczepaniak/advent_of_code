use std::collections::HashSet;

use common::network;

fn main() -> anyhow::Result<()> {
    let input = network::get_input(2022, 3)?;

    let part_one = input
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

    let part_two = input
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
    if shared.is_ascii_lowercase() {
        Ok(*shared as usize - 'a' as usize + 1)
    } else {
        Ok(*shared as usize - 'A' as usize + 1 + 26)
    }
}
