use advent::day14;
use common::runner;

fn main() -> anyhow::Result<()> {
    let input = common::network::get_input(2022, 14)?;

    runner::run_solution(&input, day14::part_one)?;
    runner::run_solution(&input, day14::part_two)?;

    Ok(())
}