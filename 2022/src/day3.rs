use std::collections::HashSet;

pub fn part_one(input: &str) -> anyhow::Result<usize> {
    input
        .lines()
        .map(|l| {
            [
                HashSet::from_iter(l[..l.len() / 2].chars()),
                HashSet::from_iter(l[l.len() / 2..].chars()),
            ]
        })
        .map(|ch| priority_for_chunk(&ch))
        .sum()
}

pub fn part_two(input: &str) -> anyhow::Result<usize> {
    input
        .lines()
        .map(|l| HashSet::from_iter(l.chars()))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(priority_for_chunk)
        .sum()
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
