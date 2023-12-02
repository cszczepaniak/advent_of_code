use advent::day13;
use common::runner;

fn main() -> anyhow::Result<()> {
    let input = common::network::get_input(2022, 13)?;

    runner::run_solution(&input, day13::part_one)?;
    runner::run_solution(&input, day13::part_two)?;

    Ok(())
}