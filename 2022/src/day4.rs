use std::str::FromStr;

use nom::{bytes::complete::tag, character::complete, sequence::separated_pair, Finish, IResult};

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<InputLine>().unwrap())
        .filter(|InputLine(r1, r2)| r1.contains_range(&r2) || r2.contains_range(&r1))
        .count()
}

pub fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<InputLine>().unwrap())
        .filter(|InputLine(r1, r2)| r1.overlaps(&r2))
        .count()
}

#[derive(Debug)]
struct ElfRange {
    start: i32,
    end: i32,
}

impl ElfRange {
    fn contains_range(&self, other: &Self) -> bool {
        other.start >= self.start && self.end >= other.end
    }

    fn contains_point(&self, pt: i32) -> bool {
        pt >= self.start && pt <= self.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains_point(other.start)
            || self.contains_point(other.end)
            || other.contains_point(self.start)
            || other.contains_point(self.end)
    }
}

#[derive(Debug)]
struct InputLine(ElfRange, ElfRange);

impl FromStr for InputLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (r1, r2) = match separated_pair(parse_range, tag(","), parse_range)(s).finish() {
            Ok((_, res)) => res,
            Err(_) => anyhow::bail!("failed to parse"),
        };

        Ok(Self(r1, r2))
    }
}

fn parse_range(input: &str) -> IResult<&str, ElfRange> {
    let (input, (start, end)) = separated_pair(complete::i32, tag("-"), complete::i32)(input)?;

    Ok((input, ElfRange { start, end }))
}
