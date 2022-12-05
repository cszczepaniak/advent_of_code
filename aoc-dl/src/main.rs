use std::{env::current_dir, fs::File, io::Write, path::Path};

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
        Commands::Init { day, output } => init(day, output)?,
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

fn init(day: Option<usize>, output: Option<String>) -> anyhow::Result<()> {
    let day = day.unwrap_or_else(|| chrono::Local::now().day() as usize);
    let output = output.unwrap_or(format!("src/bin/day{}.rs", day));

    let f = File::create(output)?;

    let reg = Handlebars::new();
    reg.render_template_to_write(TEMPLATE, &json!({ "day": day }), f)?;

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
        day: Option<usize>,

        #[arg(short, long)]
        output: Option<String>,
    },
}

#[derive(Deserialize, Serialize)]
struct Config {
    cookie: String,
}

static TEMPLATE: &str = "use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let input = common::get_input(2022, 4)?;

    // let part_one = input
    //    .lines()
    //    .map(|l| l.parse::<TInput>().unwrap())

    // println!(\"part 1: {part_one}\");

    // let part_two = input
    //    .lines()
    //    .map(|l| l.parse::<TInput>().unwrap())

    // println!(\"part 2: {part_two}\");
}
";
