use std::{fs::File, io::BufRead, process::Command};

use advent::bench::Bench;
use clap::Parser;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1},
    combinator::{map, opt},
    sequence::{delimited, preceded, terminated},
    IResult,
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let output = Command::new("go")
        .arg("test")
        .arg("-short")
        .arg("-bench")
        .arg("-run=^$")
        .args(["-bench", "^(BenchmarkAll)$"])
        .arg("github.com/joshprzybyszewski/aoc2022")
        .current_dir(cli.path)
        .output()?;

    let mut benches = Vec::new();

    for l in output.stdout.lines() {
        let l = l?;
        let res = parse_bench(&l);

        let bench = match res {
            Ok((_, bench)) => Some(bench),
            Err(nom::Err::Error(_)) => None,
            Err(_) => anyhow::bail!("issue parsing bench"),
        };

        if let Some(bench) = bench {
            benches.push(bench);
        }
    }

    let f = File::create(cli.output)?;
    serde_json::to_writer(f, &benches)?;

    Ok(())
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    output: String,
}

fn parse_bench(input: &str) -> IResult<&str, Bench> {
    let (input, day) = parse_day(input)?;
    let (input, part) = parse_part(input)?;
    let (input, time) = parse_time(input)?;

    Ok((input, Bench { day, part, time }))
}

fn parse_day(input: &str) -> IResult<&str, usize> {
    preceded(tag("BenchmarkAll/Day_"), usize)(input)
}

fn parse_part(input: &str) -> IResult<&str, usize> {
    delimited(
        tag("/Part_"),
        alt((
            map(tag("One"), |_: &str| 1usize),
            map(tag("Two"), |_: &str| 2usize),
        )),
        opt(tag("-12")),
    )(input)
}

fn parse_time(input: &str) -> IResult<&str, usize> {
    let (input, _) = multispace1(input)?;
    let (input, _) = usize(input)?;
    let (input, _) = multispace1(input)?;
    terminated(usize, tag(" ns/op"))(input)
}

fn usize(input: &str) -> IResult<&str, usize> {
    map(complete::u32, |n| n as usize)(input)
}
