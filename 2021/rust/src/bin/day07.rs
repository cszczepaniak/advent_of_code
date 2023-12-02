use advent::input::{self, InputError};

fn main() -> Result<(), InputError> {
    let nums: Vec<isize> = input::read_input_delim("./input/day07.txt", ",")?;
    let max_num = *nums.iter().max().unwrap();
    let mut min_fuel = isize::MAX;
    for pos in 0..max_num {
        min_fuel = min_fuel.min(cost_to_align_const(&nums, pos));
    }

    println!("{}", min_fuel);

    let mut min_fuel = isize::MAX;
    for pos in 0..max_num {
        min_fuel = min_fuel.min(cost_to_align_linear(&nums, pos));
    }

    println!("{}", min_fuel);

    Ok(())
}

fn cost_to_align_const(nums: &Vec<isize>, pos: isize) -> isize {
    nums.iter().map(|&n| (n - pos).abs()).sum()
}

fn cost_to_align_linear(nums: &Vec<isize>, pos: isize) -> isize {
    nums.iter()
        .map(|&n| {
            let dist = (n - pos).abs();
            dist * (1 + dist) / 2
        })
        .sum()
}
