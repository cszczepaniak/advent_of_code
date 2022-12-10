use std::{collections::HashSet, str::FromStr};

pub fn part_one(input: &str) -> anyhow::Result<usize> {
    let grid: Grid = input.parse()?;

    let mut res = HashSet::new();
    // Insert the corners; we won't hit them below
    res.insert((0, 0));
    res.insert((0, grid.n_cols() - 1));
    res.insert((grid.n_rows() - 1, 0));
    res.insert((grid.n_rows() - 1, grid.n_cols() - 1));

    for row in 1..grid.n_rows() - 1 {
        let mut max = -1;
        for col in 0..grid.n_cols() {
            let height = grid.rows[row][col];
            if height > max {
                max = height;
                res.insert((row, col));
            }
        }
        let mut max = -1;
        for col in (0..grid.n_cols()).rev() {
            let height = grid.rows[row][col];
            if height > max {
                max = height;
                res.insert((row, col));
            }
        }
    }

    for col in 1..grid.n_cols() - 1 {
        let mut max = -1;
        for row in 0..grid.n_rows() {
            let height = grid.rows[row][col];
            if height > max {
                max = height;
                res.insert((row, col));
            }
        }
        let mut max = -1;
        for row in (0..grid.n_rows()).rev() {
            let height = grid.rows[row][col];
            if height > max {
                max = height;
                res.insert((row, col));
            }
        }
    }

    Ok(res.len())
}

pub fn part_two(input: &str) -> anyhow::Result<usize> {
    let grid: Grid = input.parse()?;

    let mut max_score = 0;

    for row in 1..grid.n_rows() - 1 {
        for col in 1..grid.n_cols() - 1 {
            let score = calculate_scenic_score(&grid, (row, col));
            if score > max_score {
                max_score = score;
            }
        }
    }

    Ok(max_score)
}

fn calculate_scenic_score(grid: &Grid, (row, col): (usize, usize)) -> usize {
    let height = grid.rows[row][col];
    let mut score = 1;

    let mut count = 0;
    for i in (row + 1)..grid.n_rows() {
        count += 1;
        if grid.rows[i][col] >= height {
            break;
        }
    }
    score *= count;

    let mut count = 0;
    for i in (0..row).rev() {
        count += 1;
        if grid.rows[i][col] >= height {
            break;
        }
    }
    score *= count;

    let mut count = 0;
    for i in (col + 1)..grid.n_cols() {
        count += 1;
        if grid.rows[row][i] >= height {
            break;
        }
    }
    score *= count;

    let mut count = 0;
    for i in (0..col).rev() {
        count += 1;
        if grid.rows[row][i] >= height {
            break;
        }
    }
    score *= count;

    score
}

struct Grid {
    rows: Vec<Vec<isize>>,
}

impl<'a> Grid {
    fn n_rows(&self) -> usize {
        self.rows.len()
    }

    fn n_cols(&self) -> usize {
        self.rows[0].len()
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Self { rows: Vec::new() };
        for l in s.lines().map(|l| l.trim()) {
            let mut row = Vec::new();
            for c in l.chars() {
                let n = match c {
                    '0' => 0,
                    _ => (c as isize - '1' as isize) + 1,
                };
                row.push(n);
            }

            grid.rows.push(row);
        }

        Ok(grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "30373
    25512
    65332
    33549
    35390";

    #[test]
    fn test_part_one_example() {
        assert_eq!(21, part_one(EXAMPLE).unwrap());
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(8, part_two(EXAMPLE).unwrap());
    }
}
