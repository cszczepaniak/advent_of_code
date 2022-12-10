use std::{collections::HashSet, ops::Add, str::FromStr};

pub fn part_one(input: &str) -> usize {
    num_tail_positions(
        input
            .lines()
            .map(|l| l.trim().parse::<Instruction>().expect("bad input")),
        [Point::default(); 2],
    )
}

pub fn part_two(input: &str) -> usize {
    num_tail_positions(
        input
            .lines()
            .map(|l| l.trim().parse::<Instruction>().expect("bad input")),
        [Point::default(); 10],
    )
}

fn num_tail_positions<const N: usize>(
    instructions: impl Iterator<Item = Instruction>,
    mut rope: [Point; N],
) -> usize {
    let mut res = HashSet::new();
    res.insert(rope[rope.len() - 1]);

    for instruction in instructions {
        let (delta, n) = match instruction {
            Instruction::Up(n) => ((0, 1), n),
            Instruction::Down(n) => ((0, -1), n),
            Instruction::Left(n) => ((-1, 0), n),
            Instruction::Right(n) => ((1, 0), n),
        };

        for _ in 0..n {
            rope[0] = rope[0] + delta;
            for i in 0..rope.len() - 1 {
                rope[i + 1] = new_tail_pos(rope[i], rope[i + 1]);
            }
            res.insert(rope[rope.len() - 1]);
        }
    }

    res.len()
}

fn new_tail_pos(head: Point, curr_tail: Point) -> Point {
    let (dx, dy) = distance(head, curr_tail);

    if dx.abs() <= 1 && dy.abs() <= 1 {
        return curr_tail;
    }

    curr_tail + (dx.signum(), dy.signum())
}

fn distance(head: Point, tail: Point) -> (isize, isize) {
    (head.x - tail.x, head.y - tail.y)
}

#[derive(Default, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Add<(isize, isize)> for Point {
    type Output = Self;

    fn add(self, (dx, dy): (isize, isize)) -> Self::Output {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

enum Instruction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, num) = s
            .split_once(" ")
            .ok_or(anyhow::anyhow!("malformed input"))?;

        let ins = match dir {
            "U" => Instruction::Up(num.parse()?),
            "D" => Instruction::Down(num.parse()?),
            "L" => Instruction::Left(num.parse()?),
            "R" => Instruction::Right(num.parse()?),
            _ => anyhow::bail!("malformed direction: {}", dir),
        };
        Ok(ins)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2";
        assert_eq!(13, part_one(input))
    }

    #[test]
    fn part_two_test() {
        let input = "R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20";
        assert_eq!(36, part_two(input));
    }
}
