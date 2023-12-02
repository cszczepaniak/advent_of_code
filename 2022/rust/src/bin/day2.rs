use advent::day2;
use common::runner;

fn main() -> anyhow::Result<()> {
    let input = common::network::get_input(2022, 2)?;

    runner::run_solution(&input, day2::part_one)?;
    runner::run_solution(&input, day2::part_two)?;

    Ok(())
}
