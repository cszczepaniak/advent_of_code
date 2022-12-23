use advent::day15;
use common::runner;

fn main() -> anyhow::Result<()> {
    let input = common::network::get_input(2022, 15)?;

    runner::run_solution(&input, day15::part_one)?;
    runner::run_solution(&input, day15::part_two)?;

    Ok(())
}