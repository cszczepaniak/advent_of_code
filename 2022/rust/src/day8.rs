use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<usize> {
    let grid: Grid = input.parse()?;

    let mut seen = 0;
    for row in 0..grid.n_rows() {
        for col in 0..grid.n_cols() {
            let height = grid.rows[row][col];

            for d in Direction::all() {
                let mut ray = grid.ray(col, row, d);
                if ray.all(|n| n < height) {
                    seen += 1;
                    break;
                }
            }
        }
    }

    Ok(seen)
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

    Direction::all()
        .map(|d| {
            let mut num_trees = 0;
            for n in grid.ray(col, row, d) {
                num_trees += 1;
                if n >= height {
                    break;
                }
            }
            num_trees
        })
        .product()
}

struct Grid {
    rows: Vec<Vec<isize>>,
}

impl<'a> Grid {
    fn ray(&'a self, x: usize, y: usize, direction: Direction) -> Ray<'a> {
        Ray::new(x, y, direction, self)
    }

    fn n_rows(&self) -> usize {
        self.rows.len()
    }

    fn n_cols(&self) -> usize {
        self.rows[0].len()
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> impl Iterator<Item = Self> {
        [Self::Up, Self::Down, Self::Left, Self::Right].into_iter()
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

struct Ray<'a> {
    grid: &'a Grid,
    curr: (isize, isize),
    direction: Direction,
}

impl<'a> Ray<'a> {
    fn new(x: usize, y: usize, direction: Direction, grid: &'a Grid) -> Self {
        let x = x as isize;
        let y = y as isize;

        let (dx, dy) = direction.delta();
        let start = (x + dx, y + dy);

        Self {
            grid,
            curr: start,
            direction,
        }
    }
}

impl<'a> Iterator for Ray<'a> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.curr;
        if x < 0 || y < 0 {
            return None;
        }

        let res = self.grid.rows.get(y as usize)?.get(x as usize)?;
        let (dx, dy) = self.direction.delta();
        self.curr.0 += dx;
        self.curr.1 += dy;

        Some(*res)
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
