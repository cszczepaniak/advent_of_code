use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let part_one: usize =
        common::parse_input_lines::<PartOneInput, anyhow::Error>("./input/day2.txt")?
            .iter()
            .map(|input| input.me.score() + input.me.calculate_result(&input.other).score())
            .sum();

    println!("part 1: {part_one}");

    let part_two: usize =
        common::parse_input_lines::<PartTwoInput, anyhow::Error>("./input/day2.txt")?
            .iter()
            .map(|input| {
                input.other.should_choose(&input.desired_result).score()
                    + input.desired_result.score()
            })
            .sum();

    println!("part 2: {part_two}");

    Ok(())
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

impl FromStr for Choice {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => anyhow::bail!("unexpected input"),
        }
    }
}

struct PartOneInput {
    me: Choice,
    other: Choice,
}

impl FromStr for PartOneInput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((other, me)) = s.split_once(" ") {
            Ok(PartOneInput {
                other: other.parse::<Choice>()?,
                me: me.parse::<Choice>()?,
            })
        } else {
            anyhow::bail!("malformed input");
        }
    }
}

struct PartTwoInput {
    other: Choice,
    desired_result: GameResult,
}

impl FromStr for PartTwoInput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((other, desired_result)) = s.split_once(" ") {
            Ok(PartTwoInput {
                other: other.parse::<Choice>()?,
                desired_result: desired_result.parse::<GameResult>()?,
            })
        } else {
            anyhow::bail!("malformed input");
        }
    }
}
