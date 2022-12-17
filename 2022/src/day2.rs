use std::str::FromStr;

pub fn part_one(input: &str) -> usize {
    let mut total = 0;
    for l in input.lines() {
        total += process_part_one_line(l);
    }
    total
}

fn process_part_one_line(l: &str) -> usize {
    let mut cs = l.chars();
    let other_c = cs.next().unwrap();
    cs.next().unwrap();
    let me_c = cs.next().unwrap();

    let me = match me_c {
        'X' => Choice::Rock,
        'Y' => Choice::Paper,
        'Z' => Choice::Scissors,
        _ => unreachable!("expect goo input"),
    };
    let other = match other_c {
        'A' => Choice::Rock,
        'B' => Choice::Paper,
        'C' => Choice::Scissors,
        _ => unreachable!("expect goo input"),
    };

    me.score() + me.calculate_result(&other).score()
}

pub fn part_two(input: &str) -> usize {
    let mut total = 0;
    for l in input.lines() {
        total += process_part_two_line(l);
    }

    total
}

fn process_part_two_line(l: &str) -> usize {
    let mut cs = l.chars();
    let other_c = cs.next().unwrap();
    cs.next().unwrap();
    let desired_result_c = cs.next().unwrap();

    let other = match other_c {
        'A' => Choice::Rock,
        'B' => Choice::Paper,
        'C' => Choice::Scissors,
        _ => unreachable!("expect good input"),
    };

    let desired_result = match desired_result_c {
        'X' => GameResult::Loss,
        'Y' => GameResult::Tie,
        'Z' => GameResult::Win,
        _ => unreachable!("expect good input"),
    };

    other.should_choose(&desired_result).score() + desired_result.score()
}

#[derive(Debug)]
enum GameResult {
    Win,
    Loss,
    Tie,
}

impl GameResult {
    fn score(&self) -> usize {
        match self {
            GameResult::Loss => 0,
            GameResult::Tie => 3,
            GameResult::Win => 6,
        }
    }
}

// FromStr parses the desired result for part 2.
impl FromStr for GameResult {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Tie),
            "Z" => Ok(Self::Win),
            _ => anyhow::bail!("malformed input"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn score(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn beats(&self) -> Self {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }

    fn beaten_by(&self) -> Self {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    fn should_choose(&self, desired: &GameResult) -> Choice {
        match desired {
            GameResult::Win => self.beats(),
            GameResult::Loss => self.beaten_by(),
            GameResult::Tie => self.clone(),
        }
    }

    fn calculate_result(&self, other: &Self) -> GameResult {
        if other == &self.beaten_by() {
            GameResult::Win
        } else if other == &self.beats() {
            GameResult::Loss
        } else {
            GameResult::Tie
        }
    }
}
