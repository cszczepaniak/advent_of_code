use std::fs::File;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    cookie: String,
}

async fn download_input(year: usize, day: usize, cookie: &str) -> anyhow::Result<String> {
    let client = reqwest::Client::builder().build()?;
    let bs = client
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header("Cookie", format!("session={}", cookie))
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    Ok(String::from_utf8_lossy(&bs).to_string())
}

pub fn get_input(year: usize, day: usize) -> anyhow::Result<String> {
    let config = shellexpand::tilde("~/.aoc_dl");
    let config_file = File::open(config.to_string())?;
    let config: Config = serde_yaml::from_reader(config_file)?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { download_input(year, day, &config.cookie).await })
}
