use advent::day4;
use common::runner;

fn main() -> anyhow::Result<()> {
    let input = common::network::get_input(2022, 4)?;

    runner::run_solution(&input, day4::part_one)?;
    runner::run_solution(&input, day4::part_two)?;

    Ok(())
}
