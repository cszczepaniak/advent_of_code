use std::{
    env::current_dir,
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

use chrono::Datelike;
use clap::{Parser, Subcommand};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::SetCookie { value, filepath } => set_cookie(value, filepath)?,
        Commands::Download {
            year,
            day,
            output,
            config,
        } => download(year, day, output, config).await?,
        Commands::Init { year, day } => init(year, day)?,
    };

    Ok(())
}

fn set_cookie(val: String, filepath: Option<String>) -> anyhow::Result<()> {
    let filepath = filepath.unwrap_or("~/.aoc_dl".to_string());
    let filepath = shellexpand::tilde(&filepath);

    let f = File::create(filepath.to_string())?;
    serde_yaml::to_writer(f, &Config { cookie: val })?;

    Ok(())
}

async fn download(
    year: Option<usize>,
    day: Option<usize>,
    output: Option<String>,
    config: Option<String>,
) -> anyhow::Result<()> {
    let year = get_year(year)?;
    let day = day.unwrap_or_else(|| chrono::Local::now().day() as usize);
    if day < 1 || day > 25 {
        anyhow::bail!("day must be between 1 and 25")
    }

    let output = output.unwrap_or("input".to_string());

    let config = config.unwrap_or("~/.aoc_dl".to_string());
    let config = shellexpand::tilde(&config);
    let config_file = File::open(config.to_string())?;
    let config: Config = serde_yaml::from_reader(config_file)?;

    let client = reqwest::Client::builder().build()?;
    let res = client
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header("Cookie", format!("session={}", config.cookie))
        .send()
        .await?;

    let res = res.error_for_status()?.bytes().await?;

    let output_path = Path::new(&output).join(format!("day{day}.txt"));
    File::create(output_path)?.write_all(&res)?;

    Ok(())
}

fn init(year: Option<usize>, day: Option<usize>) -> anyhow::Result<()> {
    let year = year.unwrap_or_else(|| chrono::Local::now().year() as usize);
    let day = day.unwrap_or_else(|| chrono::Local::now().day() as usize);

    let reg = Handlebars::new();

    let output = format!("src/bin/day{}.rs", day);
    let f = File::create(output)?;

    reg.render_template_to_write(BIN_TEMPLATE, &json!({ "day": day, "year": year }), f)?;

    let output = format!("src/day{}.rs", day);
    let f = File::create(output)?;

    reg.render_template_to_write(LIB_TEMPLATE, &json!({}), f)?;

    let lib_file = "src/lib.rs";
    let mut f = OpenOptions::new().write(true).append(true).open(lib_file)?;
    writeln!(f, "pub mod day{};", day)?;

    Ok(())
}

fn get_year(input: Option<usize>) -> anyhow::Result<usize> {
    if let Some(y) = input {
        return Ok(y);
    }

    let from_dir = current_dir()?
        .components()
        .last()
        .ok_or(anyhow::anyhow!("problem parsing current path"))?
        .as_os_str()
        .to_string_lossy()
        .parse::<usize>();

    if let Ok(y) = from_dir {
        return Ok(y);
    }

    Ok(chrono::Local::now().day() as usize)
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    SetCookie {
        value: String,

        #[arg(short, long)]
        filepath: Option<String>,
    },
    Download {
        #[arg(short, long)]
        year: Option<usize>,

        #[arg(short, long)]
        day: Option<usize>,

        #[arg(short, long)]
        output: Option<String>,

        #[arg(short, long)]
        config: Option<String>,
    },
    Init {
        #[arg(short, long)]
        year: Option<usize>,

        #[arg(short, long)]
        day: Option<usize>,
    },
}

#[derive(Deserialize, Serialize)]
struct Config {
    cookie: String,
}

static BIN_TEMPLATE: &str = "use advent::day{{day}};
use common::runner;

fn main() -> anyhow::Result<()> {
    let input = common::network::get_input({{year}}, {{day}})?;

    runner::run_solution(&input, day{{day}}::part_one)?;
    runner::run_solution(&input, day{{day}}::part_two)?;

    Ok(())
}";

static LIB_TEMPLATE: &str = "pub fn part_one(input: &str) -> usize {
    todo!()
}

pub fn part_two(input: &str) -> usize {
    todo!()
}";
