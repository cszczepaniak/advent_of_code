use advent::day10;
use common::runner;

fn main() -> anyhow::Result<()> {
    let input = common::network::get_input(2022, 10)?;

    runner::run_solution(&input, day10::part_one)?;
    runner::run_solution(&input, day10::part_two)?;

    Ok(())
}
