use advent::day7;
use common::runner;

fn main() -> anyhow::Result<()> {
    let input = common::network::get_input(2022, 7)?;

    runner::run_solution(&input, day7::part_one)?;
    runner::run_solution(&input, day7::part_two)?;

    Ok(())
}
