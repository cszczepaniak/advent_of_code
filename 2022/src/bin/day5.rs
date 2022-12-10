use advent::day5;
use common::runner;

fn main() -> anyhow::Result<()> {
    let input = common::network::get_input(2022, 5)?;

    runner::run_solution(&input, day5::part_one)?;
    runner::run_solution(&input, day5::part_two)?;

    Ok(())
}
