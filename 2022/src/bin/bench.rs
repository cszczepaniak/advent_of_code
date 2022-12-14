use std::{
    fs::{self, File, OpenOptions},
    path::Path,
};

use advent::bench::Bench;
use regex::Regex;
use serde::Deserialize;

fn main() -> anyhow::Result<()> {
    let criterion_dir = Path::new("target").join("criterion");
    let file_pattern = Regex::new("day(\\d+)")?;
    let mut benches = Vec::new();
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

            let mut time = usize::MAX;
            if let Some(data_point) = br.slope {
                time = data_point.point_estimate as usize;
            }
            if let Some(data_point) = br.mean {
                time = data_point.point_estimate as usize;
            }

            benches.push(Bench {
                day: day_num,
                part: i,
                time,
            });
        }
    }

    let f = File::create("mine.json")?;
    serde_json::to_writer(f, &benches)?;

    Ok(())
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
