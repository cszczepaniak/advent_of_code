use advent::day3;
use common::runner;

fn main() -> anyhow::Result<()> {
    let input = common::network::get_input(2022, 3)?;

    runner::run_solution(&input, day3::part_one)?;
    runner::run_solution(&input, day3::part_two)?;

    Ok(())
}
