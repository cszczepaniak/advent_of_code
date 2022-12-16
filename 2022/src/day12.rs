use std::{
    collections::HashSet,
    ops::{Add, Sub},
    str::FromStr,
};

pub fn part_one(input: &str) -> anyhow::Result<usize> {
    find_shortest_path(
        input,
        |h, nh| nh <= h + 1,
        |g| g.start,
        |g, pos| pos == g.end,
    )
}

pub fn part_two(input: &str) -> anyhow::Result<usize> {
    find_shortest_path(
        input,
        |h, nh| nh >= h - 1,
        |g| g.end,
        |g, pos| g.height_at(pos) == 0,
    )
}

fn find_shortest_path<F1, F2, F3>(
    input: &str,
    height_filter: F1,
    get_start: F2,
    is_done: F3,
) -> anyhow::Result<usize>
where
    F1: Fn(usize, usize) -> bool,
    F2: Fn(&Grid) -> Position,
    F3: Fn(&Grid, Position) -> bool,
{
    let grid: Grid = input.parse()?;
    let mut last: Option<Entry> = None;
    let mut seen = HashSet::new();
    let mut entries = Vec::new();
    let mut idx = 0;

    entries.push(Entry {
        pos: get_start(&grid),
        last: None,
    });

    while idx < entries.len() && last.is_none() {
        let curr = &entries[idx];

        let (neighbors, n) = grid.neighbors_at(curr.pos, &height_filter, &seen);
        for i in 0..n {
            let neighbor = neighbors[i];
            if is_done(&grid, neighbor) {
                last = Some(Entry {
                    pos: neighbor,
                    last: Some(idx),
                });
                break;
            }

            seen.insert(neighbor);
            entries.push(Entry {
                pos: neighbor,
                last: Some(idx),
            });
        }
        idx += 1;
    }

    let mut n = 0;
    let mut curr_idx = last.unwrap().last.unwrap();
    while let Some(e) = entries.get(curr_idx).unwrap().last {
        n += 1;
        curr_idx = e
    }

    Ok(n + 1)
}

#[derive(Debug)]
struct Entry {
    pos: Position,
    last: Option<usize>,
}

struct Grid {
    grid: Vec<Vec<usize>>,
    start: Position,
    end: Position,
}

impl<'a> Grid {
    fn height_at(&self, pos: Position) -> usize {
        self.grid[pos.row][pos.col]
    }

    fn neighbors_at<F>(
        &'a self,
        pos: Position,
        filter_height: F,
        exclude: &'a HashSet<Position>,
    ) -> ([Position; 4], usize)
    where
        F: Fn(usize, usize) -> bool,
    {
        let mut res = [Position::default(); 4];
        let mut num = 0;
        for (i, n) in self
            .all_neighbors(pos)
            .filter(move |n_pos| {
                let curr_height = self.height_at(pos);
                let neighbor_height = self.height_at(*n_pos);
                // We're going in reverse: neighbors can be at minimum one less than us.
                !exclude.contains(&n_pos) && filter_height(curr_height, neighbor_height)
            })
            .enumerate()
        {
            res[i] = n;
            num += 1;
        }
        (res, num)
    }

    fn all_neighbors(&self, pos: Position) -> Neighbors {
        Neighbors {
            max_row: self.grid.len() - 1,
            max_col: self.grid[0].len() - 1,
            pos,
            dir: Some(Direction::Up),
        }
    }
}

struct Neighbors {
    max_row: usize,
    max_col: usize,
    pos: Position,
    dir: Option<Direction>,
}

impl<'a> Iterator for Neighbors {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(dir) = self.dir {
            match dir {
                Direction::Up => {
                    self.dir = Some(Direction::Left);
                    if self.pos.row == 0 {
                        continue;
                    }

                    return Some(self.pos - (1, 0));
                }
                Direction::Left => {
                    self.dir = Some(Direction::Down);
                    if self.pos.col == 0 {
                        continue;
                    }

                    return Some(self.pos - (0, 1));
                }
                Direction::Down => {
                    self.dir = Some(Direction::Right);
                    if self.pos.row == self.max_row {
                        continue;
                    }

                    return Some(self.pos + (1, 0));
                }
                Direction::Right => {
                    self.dir = None;
                    if self.pos.col == self.max_col {
                        continue;
                    }

                    return Some(self.pos + (0, 1));
                }
            }
        }
        None
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        let mut pos = None;
        let mut end = None;

        for (i, l) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in l.chars().enumerate() {
                match c {
                    'S' => {
                        pos = Some(Position { row: i, col: j });
                        row.push(0);
                    }
                    'E' => {
                        end = Some(Position { row: i, col: j });
                        row.push(25);
                    }
                    c => row.push(c as usize - 'a' as usize),
                }
            }
            grid.push(row);
        }

        Ok(Self {
            grid,
            start: pos.unwrap(),
            end: end.unwrap(),
        })
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Add<(usize, usize)> for Position {
    type Output = Self;

    fn add(self, rhs: (usize, usize)) -> Self::Output {
        Self::Output {
            row: self.row + rhs.0,
            col: self.col + rhs.1,
        }
    }
}

impl Sub<(usize, usize)> for Position {
    type Output = Self;

    fn sub(self, rhs: (usize, usize)) -> Self::Output {
        Self::Output {
            row: self.row - rhs.0,
            col: self.col - rhs.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part_one() {
        let input = common::network::get_input(2022, 12).unwrap();
        let res = part_one(&input).unwrap();

        assert_eq!(534, res);
    }

    #[test]
    fn test_part_two() {
        let input = common::network::get_input(2022, 12).unwrap();
        let res = part_two(&input).unwrap();

        assert_eq!(525, res);
    }

    #[test]
    fn test_all_neighbors() {
        let grid: Grid = EXAMPLE.parse().unwrap();

        let mut neighbors = grid.all_neighbors(Position { row: 0, col: 0 });
        assert_eq!(Some(Position { row: 1, col: 0 }), neighbors.next());
        assert_eq!(Some(Position { row: 0, col: 1 }), neighbors.next());
        assert_eq!(None, neighbors.next());

        let mut neighbors = grid.all_neighbors(Position { row: 1, col: 1 });
        assert_eq!(Some(Position { row: 0, col: 1 }), neighbors.next());
        assert_eq!(Some(Position { row: 1, col: 0 }), neighbors.next());
        assert_eq!(Some(Position { row: 2, col: 1 }), neighbors.next());
        assert_eq!(Some(Position { row: 1, col: 2 }), neighbors.next());
        assert_eq!(None, neighbors.next());

        let mut neighbors = grid.all_neighbors(Position { row: 4, col: 7 });
        assert_eq!(Some(Position { row: 3, col: 7 }), neighbors.next());
        assert_eq!(Some(Position { row: 4, col: 6 }), neighbors.next());
        assert_eq!(None, neighbors.next());
    }
}
