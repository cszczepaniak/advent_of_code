use std::str::FromStr;

use nom::{bytes::complete::tag, sequence::preceded, Finish, IResult};

pub fn part_one(input: &str) -> isize {
    let mut sum = 0;
    let mut x = 1;
    let mut cycle = 1;
    for instruction in input.lines().map(|l| {
        l.trim()
            .parse::<Instruction>()
            .expect("failed to parse instruction")
    }) {
        match instruction {
            Instruction::Add(n) => {
                if is_reporting_cycle(cycle) {
                    sum += x * cycle as isize;
                } else if is_reporting_cycle(cycle + 1) {
                    sum += x * (cycle + 1) as isize;
                }
                x += n;
                cycle += 2;
            }
            Instruction::Noop => {
                if is_reporting_cycle(cycle) {
                    sum += x * cycle as isize;
                }
                cycle += 1;
            }
        }
    }

    sum
}

fn is_reporting_cycle(cycle: usize) -> bool {
    if cycle < 20 {
        return false;
    }
    (cycle - 20) % 40 == 0
}

pub fn part_two(input: &str) -> String {
    let mut res = String::new();
    let mut x: isize = 1;
    let mut col = 0;
    for instruction in input.lines().map(|l| {
        l.trim()
            .parse::<Instruction>()
            .expect("failed to parse instruction")
    }) {
        match instruction {
            Instruction::Add(n) => {
                draw(&mut res, &mut col, x);
                draw(&mut res, &mut col, x);
                x += n;
            }
            Instruction::Noop => {
                draw(&mut res, &mut col, x);
            }
        }
    }

    res
}

fn draw(output: &mut String, col: &mut usize, x: isize) {
    if col_contains_x(*col, x) {
        output.push('#');
    } else {
        output.push('.');
    }
    *col += 1;
    if *col >= 40 {
        output.push('\n');
        *col = 0;
    }
}

fn col_contains_x(col: usize, x: isize) -> bool {
    col >= x as usize - 1 && col <= x as usize + 1
}

enum Instruction {
    Add(isize),
    Noop,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r: IResult<_, _, nom::error::Error<&str>> = tag("noop")(s);
        if let Ok(_) = r {
            return Ok(Self::Noop);
        }

        let (_, n) = preceded(
            tag("addx "),
            nom::character::complete::i32::<_, nom::error::Error<&str>>,
        )(s)
        .finish()
        .map_err(|_| anyhow::anyhow!("failed to parse addx"))?;
        Ok(Self::Add(n as isize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = "addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop";
        assert_eq!(13140, part_one(input));
    }
}
