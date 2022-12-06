use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::take,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{self},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn main() -> anyhow::Result<()> {
    let two_new_lines = LINE_ENDING.to_string() + LINE_ENDING;

    let input = common::get_input(2022, 5)?;

    let (stacks, instructions) = input
        .split_once(&two_new_lines)
        .expect("input should have a blank line");

    // The stacks have a trailing line of indices
    // e.g. 1 2 3 4 5
    let (stacks, _) = stacks
        .rsplit_once(LINE_ENDING)
        .expect("expected splitting off the indices to succeed");

    let stacks = parse_stacks(&stacks)?;

    let part_one = process_stacks(
        stacks.clone(),
        parse_instructions(instructions),
        |to, items| {
            to.extend(items.iter().rev());
        },
    )?;
    println!("{part_one}");

    let part_two = process_stacks(stacks, parse_instructions(instructions), |to, items| {
        to.extend(items);
    })?;
    println!("{part_two}");

    Ok(())
}

fn process_stacks<F>(
    mut stacks: BTreeMap<usize, Vec<char>>,
    instructions: impl Iterator<Item = Instruction>,
    mut processor: F,
) -> anyhow::Result<String>
where
    F: FnMut(&mut Vec<char>, Vec<char>),
{
    for ins in instructions {
        let from = stacks.get_mut(&ins.dir.from).unwrap();
        let idx = from.len().saturating_sub(ins.n);
        let tail = from.split_off(idx);

        let mut to = stacks.get_mut(&ins.dir.to).unwrap();

        processor(&mut to, tail);
    }

    let mut res_str = String::with_capacity(stacks.len());
    for (_, v) in stacks.iter() {
        res_str.extend(v.last());
    }
    Ok(res_str)
}

struct Instruction {
    n: usize,
    dir: Direction,
}

struct Direction {
    from: usize,
    to: usize,
}

fn parse_instructions<'i>(input: &'i str) -> impl Iterator<Item = Instruction> + 'i {
    input
        .lines()
        .map(|l| parse_instruction(l).expect("failed to parse instruction"))
}

fn parse_instruction(input: &str) -> anyhow::Result<Instruction> {
    let (_, (n, dir)) = tuple((parse_move, parse_from_to))(input)
        .map_err(|_| anyhow::anyhow!("failed to parse instruction"))?;

    Ok(Instruction { n, dir })
}

fn parse_from_to(input: &str) -> IResult<&str, Direction> {
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;

    Ok((
        input,
        Direction {
            from: from as usize,
            to: to as usize,
        },
    ))
}

fn parse_move(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("move ")(input)?;
    let (input, n) = complete::u32(input)?;

    Ok((input, n as usize))
}

fn parse_stacks(input: &str) -> anyhow::Result<BTreeMap<usize, Vec<char>>> {
    let stacks = input
        .lines()
        .fold(BTreeMap::new(), |mut m: BTreeMap<usize, Vec<char>>, l| {
            let (_, r) = parse_box_entries(l).unwrap();
            for (i, o) in r.iter().enumerate() {
                if let Some(c) = o {
                    // Stacks are 1-indexed.
                    let v = m.entry(i + 1).or_default();
                    // We're going top-down, so always insert at the beginning.
                    v.insert(0, *c);
                }
            }
            m
        });

    Ok(stacks)
}

fn parse_box_entries(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(tag(" "), parse_box_entry)(input)
}

fn parse_box_entry(input: &str) -> IResult<&str, Option<char>> {
    alt((parse_box, parse_empty))(input)
}

fn parse_box(input: &str) -> IResult<&str, Option<char>> {
    let (input, c) = delimited(tag("["), take(1usize), tag("]"))(input)?;

    // We know next() will return Some because we did a take(1)
    Ok((input, c.chars().next()))
}

fn parse_empty(input: &str) -> IResult<&str, Option<char>> {
    let (input, _) = take_while_m_n(3, 3, |c: char| c.is_whitespace())(input)?;
    Ok((input, None))
}

#[cfg(test)]
mod tests {
    use crate::parse_stacks;

    static EXAMPLE_STACKS: &str = "    [D]    
[N] [C]    
[Z] [M] [P]";

    #[test]
    fn parse_example_stacks() {
        let s = parse_stacks(EXAMPLE_STACKS).unwrap();

        let v1 = s.get(&0).unwrap();
        assert_eq!(&vec!['Z', 'N'], v1);

        let v2 = s.get(&1).unwrap();
        assert_eq!(&vec!['M', 'C', 'D'], v2);

        let v3 = s.get(&2).unwrap();
        assert_eq!(&vec!['P'], v3);
    }
}
