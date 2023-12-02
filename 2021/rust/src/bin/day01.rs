use advent::input::{self, InputError};

fn main() -> Result<(), InputError> {
    let nums: Vec<usize> = input::read_input("./input/day01.txt")?;

    let increases = nums
        .iter()
        .take(nums.len() - 1)
        .zip(nums.iter().skip(1))
        .filter(|(&a, &b)| b > a)
        .count();
    println!("{}", increases);

    let increases = nums
        .iter()
        .take(nums.len() - 3)
        .zip(nums.iter().skip(3))
        .filter(|(&a, &b)| b > a)
        .count();
    println!("{}", increases);

    Ok(())
}
