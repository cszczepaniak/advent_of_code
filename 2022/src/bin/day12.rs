use advent::day12;
use common::runner;

fn main() -> anyhow::Result<()> {
    let input = common::network::get_input(2022, 12)?;

    runner::run_solution(&input, day12::part_one)?;
    runner::run_solution(&input, day12::part_two)?;

    Ok(())
}
