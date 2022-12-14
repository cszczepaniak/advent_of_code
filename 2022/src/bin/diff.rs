use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::Write,
};

use advent::bench::Bench;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let f = File::open(cli.mine)?;
    let mine: Vec<Bench> = serde_json::from_reader(f)?;
    let mine: HashMap<(usize, usize), Bench> =
        mine.into_iter().map(|b| ((b.day, b.part), b)).collect();

    let f = File::open(cli.other)?;
    let other: Vec<Bench> = serde_json::from_reader(f)?;
    let other: HashMap<(usize, usize), Bench> =
        other.into_iter().map(|b| ((b.day, b.part), b)).collect();

    let mut diffs = BTreeMap::new();

    for (k, v) in mine {
        let my_time = v.time;
        let other_time = other.get(&k).map_or(usize::MAX, |b| b.time);
        let rel_diff = ((my_time as f64 - other_time as f64) / (my_time as f64)) * 100.0;

        diffs.insert(
            k,
            Diff {
                my_time,
                other_time,
                rel_diff,
            },
        );
    }

    let mut f = File::create("compared_results.md")?;
    writeln!(f, "|Day|Part|Mine (ns)|Other (ns)|Diff (%)|")?;
    writeln!(f, "|-|-|-|-|-|")?;
    for ((day, part), diff) in diffs {
        writeln!(
            f,
            "|{}|{}|{}|{}|{:.2}|",
            day, part, diff.my_time, diff.other_time, diff.rel_diff
        )?;
    }

    Ok(())
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    mine: String,

    #[arg(short, long)]
    other: String,
}

struct Diff {
    my_time: usize,
    other_time: usize,
    rel_diff: f64,
}
