use advent::day1;
use common::runner;

fn main() -> anyhow::Result<()> {
    let input = common::network::get_input(2022, 1)?;

    runner::run_solution(&input, day1::part_one)?;
    runner::run_solution(&input, day1::part_two)?;

    Ok(())
}
