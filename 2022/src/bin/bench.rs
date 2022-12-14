use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};

use regex::Regex;
use serde::Deserialize;

fn main() -> anyhow::Result<()> {
    let criterion_dir = Path::new("target").join("criterion");
    let file_pattern = Regex::new("day(\\d+)")?;
    let mut outputs = HashMap::new();
    for e in fs::read_dir(criterion_dir)? {
        let e = e?;
        let n = e.file_name();

        let other = n
            .to_str()
            .ok_or(anyhow::anyhow!("error converting file path"))?;

        if !file_pattern.is_match(other) {
            continue;
        }

        let cap = file_pattern.captures(other).unwrap();
        let day_num: usize = cap.get(1).unwrap().as_str().parse()?;

        for i in [1, 2] {
            let p = e
                .path()
                .join(format!("part{i}"))
                .join("new")
                .join("estimates.json");
            let f = OpenOptions::new().read(true).open(&p)?;
            let br: BenchResult = serde_json::from_reader(f)?;

            outputs.insert(
                (day_num, i),
                Output {
                    name: format!("day {day_num} part {i}"),
                    time: br
                        .slope
                        .unwrap_or(br.mean.unwrap_or_default())
                        .point_estimate,
                },
            );
        }
    }

    let target_file = "results.md";
    let mut f = File::create(target_file)?;

    writeln!(f, "# Results")?;
    writeln!(f, "|Puzzle|Duration (ns)|")?;
    writeln!(f, "|-|-|")?;
    for i in 1..=outputs.len() / 2 {
        for j in [1, 2] {
            let res = outputs.get(&(i, j)).unwrap();
            writeln!(f, "|{}|{:.0}|", res.name, res.time)?;
        }
    }

    Ok(())
}

struct Output {
    name: String,
    time: f64,
}

#[derive(Deserialize)]
struct BenchResult {
    slope: Option<DataPoint>,
    mean: Option<DataPoint>,
}

#[derive(Deserialize, Default)]
struct DataPoint {
    point_estimate: f64,
}

/*
"slope": {
    "confidence_interval": {
      "confidence_level": 0.95,
      "lower_bound": 31658.264695126007,
      "upper_bound": 31817.016710509768
    },
    "point_estimate": 31728.65020503916,
    "standard_error": 40.17443861211054
  }
   */
