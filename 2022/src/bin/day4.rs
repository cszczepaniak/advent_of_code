use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::char,
    character::complete::one_of,
    combinator::map_res,
    multi::{many0, many1},
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

fn main() -> anyhow::Result<()> {
    let input = common::get_input(2022, 4)?;

    let part_one = input
        .lines()
        .map(|l| l.parse::<InputLine>().unwrap())
        .filter(|InputLine(r1, r2)| r1.contains_range(&r2) || r2.contains_range(&r1))
        .count();

    println!("part 1: {part_one}");

    let part_two = input
        .lines()
        .map(|l| l.parse::<InputLine>().unwrap())
        .filter(|InputLine(r1, r2)| r1.overlaps(&r2))
        .count();

    println!("part 2: {part_two}");

    Ok(())
}

#[derive(Debug)]
struct ElfRange {
    start: isize,
    end: isize,
}

impl ElfRange {
    fn contains_range(&self, other: &Self) -> bool {
        other.start >= self.start && self.end >= other.end
    }

    fn contains_point(&self, pt: isize) -> bool {
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

fn decimal(input: &str) -> IResult<&str, isize> {
    map_res(
        many1(terminated(one_of("0123456789"), many0(char('_')))),
        |r| String::from_iter(r).parse(),
    )(input)
}

fn parse_range(input: &str) -> IResult<&str, ElfRange> {
    let (input, (start, end)) = separated_pair(decimal, tag("-"), decimal)(input)?;

    Ok((input, ElfRange { start, end }))
}
