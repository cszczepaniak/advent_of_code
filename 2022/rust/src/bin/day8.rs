use advent::day8;
use common::runner;

fn main() -> anyhow::Result<()> {
    let input = common::network::get_input(2022, 8)?;

    runner::run_solution(&input, day8::part_one)?;
    runner::run_solution(&input, day8::part_two)?;

    Ok(())
}
