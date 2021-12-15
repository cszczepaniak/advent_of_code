use std::{fs, str::FromStr};

use advent::input::InputError;

fn main() -> Result<(), InputError> {
    let mut board: Board = fs::read_to_string("./input/day09.txt")
        .map_err(|_| InputError::FileReadError("error reading file".to_string()))?
        .parse()
        .map_err(|_| InputError::ParseError("failed to parse board".to_string()))?;

    let lowest_points = board.get_lowest_points();

    let sum = lowest_points.iter().fold(0, |prev, &pt| {
        prev + board.data[pt.0][pt.1].unwrap_or(0) + 1
    });

    println!("{}", sum);
    println!("{:?}", lowest_points.len());

    let mut basin_sizes = Vec::with_capacity(lowest_points.len());
    for pt in lowest_points {
        basin_sizes.push(board.get_basin_size(pt, 0));
    }

    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));
    let answer: usize = basin_sizes.iter().take(3).product();
    println!("{:?}", answer);

    Ok(())
}

struct Board {
    width: usize,
    height: usize,
    data: Vec<Vec<Option<usize>>>,
}

#[derive(Clone, Copy)]
enum Neighbor {
    Up,
    Down,
    Left,
    Right,
}

impl Board {
    fn get_neighbor(&self, pt: (usize, usize), n: Neighbor) -> Option<usize> {
        let (row, col) = pt;
        match n {
            Neighbor::Up => {
                if row == 0 {
                    None
                } else {
                    self.data[row - 1][col]
                }
            }
            Neighbor::Down => {
                if row >= self.height - 1 {
                    None
                } else {
                    self.data[row + 1][col]
                }
            }
            Neighbor::Left => {
                if col == 0 {
                    None
                } else {
                    self.data[row][col - 1]
                }
            }
            Neighbor::Right => {
                if col >= self.width - 1 {
                    None
                } else {
                    self.data[row][col + 1]
                }
            }
        }
    }

    fn get_lowest_points(&self) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        for i in 0..self.height {
            for j in 0..self.width {
                let curr = self.data[i][j].unwrap();
                if curr < self.get_lowest_neighbor(i, j) {
                    res.push((i, j));
                }
            }
        }
        res
    }

    fn get_lowest_neighbor(&self, row_idx: usize, col_idx: usize) -> usize {
        let mut min = usize::MAX;
        for dir in [
            Neighbor::Left,
            Neighbor::Right,
            Neighbor::Up,
            Neighbor::Down,
        ] {
            if let Some(n) = self.get_neighbor((row_idx, col_idx), dir) {
                min = min.min(n);
            }
        }
        min
    }

    fn get_basin_size(&mut self, pt: (usize, usize), start_size: usize) -> usize {
        let mut size = 1;
        let (row, col) = pt;
        let curr = self.data[row][col].unwrap();
        self.data[row][col] = None;
        for dir in [
            Neighbor::Left,
            Neighbor::Right,
            Neighbor::Up,
            Neighbor::Down,
        ] {
            if let Some(neighbor) = self.get_neighbor(pt, dir) {
                if neighbor < 9 && neighbor > curr {
                    size += self.get_basin_size(point_from_direction(pt, dir), start_size)
                }
            }
        }
        size
    }
}

fn point_from_direction(pt: (usize, usize), dir: Neighbor) -> (usize, usize) {
    let (row, col) = pt;
    match dir {
        Neighbor::Up => (row - 1, col),
        Neighbor::Down => (row + 1, col),
        Neighbor::Left => (row, col - 1),
        Neighbor::Right => (row, col + 1),
    }
}

impl FromStr for Board {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10).map(|x| x as usize));
            }
            data.push(row);
        }
        Ok(Board {
            height: data.len(),
            width: data.first().unwrap().len(),
            data,
        })
    }
}
