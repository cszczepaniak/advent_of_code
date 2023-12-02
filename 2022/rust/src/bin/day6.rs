use advent::day6;
use common::runner;

fn main() -> anyhow::Result<()> {
    let input = common::network::get_input(2022, 6)?;

    runner::run_solution(&input, day6::part_one)?;
    runner::run_solution(&input, day6::part_two)?;

    Ok(())
}
