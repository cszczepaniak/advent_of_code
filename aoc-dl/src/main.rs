use std::{fs::File, io::Write, path::Path};

use chrono::Datelike;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

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
    let year = year.unwrap_or_else(|| chrono::Local::now().year() as usize);
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

    let res = res.bytes().await?;

    let output_path = Path::new(&output).join(format!("day{day}.txt"));
    File::create(output_path)?.write_all(&res)?;

    Ok(())
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
}

#[derive(Deserialize, Serialize)]
struct Config {
    cookie: String,
}
