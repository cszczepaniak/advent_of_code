use std::str::FromStr;

use advent::input::{InputError, InputReader};

fn main() -> Result<(), InputError> {
    let reader = InputReader::default()
        .with_delim(",")
        .with_line_bounds(0, 1);
    let bingo_nums: Vec<usize> = reader.read("./input/day04.txt")?;

    let reader = InputReader::default()
        .with_delim("\n\n")
        .with_line_bounds(2, usize::MAX);
    let bingo_boards: Vec<Board> = reader.read("./input/day04.txt")?;

    let mut score = None;
    let mut boards = bingo_boards.clone();
    for n in bingo_nums.iter() {
        for b in boards.iter_mut() {
            b.hit(*n);
            if b.is_winner() {
                score = Some(b.score() * n);
                break;
            }
        }
        if let Some(s) = score {
            println!("{}", s);
            break;
        }
    }

    let mut boards = bingo_boards.clone();
    for n in bingo_nums.iter() {
        for b in boards.iter_mut() {
            b.hit(*n);
        }
        if boards.len() > 1 {
            boards.retain(|b| !b.is_winner());
        }
        if boards.len() == 1 && boards.first().unwrap().is_winner() {
            println!("{}", boards.first().unwrap().score() * n);
            break;
        }
    }

    Ok(())
}

#[derive(Copy, Clone, Debug)]
enum Square {
    Miss(usize),
    Hit,
}

#[derive(Copy, Clone, Debug)]
struct Board {
    data: [[Square; 5]; 5],

    row_counts: [usize; 5],
    col_counts: [usize; 5],
}

impl Board {
    fn hit(&mut self, target: usize) -> bool {
        for (i, r) in self.data.iter().enumerate() {
            for (j, n) in r.iter().enumerate() {
                if let Square::Miss(n) = n {
                    if n == &target {
                        self.data[i][j] = Square::Hit;
                        self.row_counts[i] += 1;
                        self.col_counts[j] += 1;
                        return true;
                    }
                }
            }
        }
        false
    }

    fn score(&self) -> usize {
        self.data
            .iter()
            .map(|r| -> usize {
                r.iter()
                    .filter_map(|sq| match sq {
                        Square::Hit => None,
                        Square::Miss(n) => Some(*n),
                    })
                    .sum()
            })
            .sum()
    }

    fn is_winner(&self) -> bool {
        self.row_counts.iter().any(|&n| n >= 5) || self.col_counts.iter().any(|&n| n >= 5)
    }
}

impl FromStr for Board {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = [[Square::Miss(0); 5]; 5];

        for (i, l) in s.lines().enumerate() {
            let elems: Vec<&str> = l.split_whitespace().collect();
            if elems.len() != 5 {
                return Err("invalid number of elements in row");
            }
            for (j, e) in elems.iter().enumerate() {
                match e.parse() {
                    Ok(n) => data[i][j] = Square::Miss(n),
                    Err(_) => return Err("error parsing number"),
                }
            }
        }

        Ok(Self {
            data,
            row_counts: [0; 5],
            col_counts: [0; 5],
        })
    }
}
