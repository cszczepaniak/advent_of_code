use advent::day11;
use common::runner;

fn main() -> anyhow::Result<()> {
    let input = common::network::get_input(2022, 11)?;

    runner::run_solution(&input, day11::part_one)?;
    runner::run_solution(&input, day11::part_two)?;

    Ok(())
}
