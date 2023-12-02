use std::str::FromStr;

use advent::input::{read_input, InputError};

fn main() -> Result<(), InputError> {
    let dirs: Vec<Direction> = read_input("./input/day02.txt")?;

    let mut horizontal = 0;
    let mut depth = 0;
    for dir in dirs.iter() {
        match dir {
            Direction::Down(val) => depth += val,
            Direction::Up(val) => depth -= val,
            Direction::Forward(val) => horizontal += val,
        }
    }
    println!("{}", depth * horizontal);

    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal = 0;
    for dir in dirs.iter() {
        match dir {
            Direction::Down(val) => aim += val,
            Direction::Up(val) => aim -= val,
            Direction::Forward(val) => {
                horizontal += val;
                depth += val * aim;
            }
        }
    }
    println!("{}", depth * horizontal);

    Ok(())
}

enum Direction {
    Down(usize),
    Up(usize),
    Forward(usize),
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((dir, val)) = s.split_once(" ") {
            match (dir, val.parse().ok()) {
                ("down", Some(val)) => Ok(Direction::Down(val)),
                ("up", Some(val)) => Ok(Direction::Up(val)),
                ("forward", Some(val)) => Ok(Direction::Forward(val)),
                (_, Some(_)) => Err("bad direction"),
                (_, None) => Err("bad value"),
            }
        } else {
            Err("malformed input")
        }
    }
}
