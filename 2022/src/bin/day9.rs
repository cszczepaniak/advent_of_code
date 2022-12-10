use advent::day9;
use common::runner;

fn main() -> anyhow::Result<()> {
    let input = common::network::get_input(2022, 9)?;

    runner::run_solution(&input, day9::part_one)?;
    runner::run_solution(&input, day9::part_two)?;

    Ok(())
}
