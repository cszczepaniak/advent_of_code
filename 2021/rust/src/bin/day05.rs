use std::{collections::HashMap, str::FromStr};

use advent::input::{self, InputError};

fn main() -> Result<(), InputError> {
    let segments: Vec<LineSegment> = input::read_input("./input/day05.txt")?;
    let mut points_covered = HashMap::new();
    for s in segments
        .iter()
        .filter(|s| s.is_horizontal() || s.is_vertical())
    {
        let points = s.points_covered();
        for p in points {
            *points_covered.entry(p).or_insert(0) += 1;
        }
    }
    let res = points_covered.iter().filter(|(_, &ct)| ct >= 2).count();
    println!("{}", res);

    let mut points_covered = HashMap::new();
    for s in segments
        .iter()
        .filter(|s| s.is_horizontal() || s.is_vertical() || s.is_perfect_diag())
    {
        let points = s.points_covered();
        for p in points {
            *points_covered.entry(p).or_insert(0) += 1;
        }
    }
    let res = points_covered.iter().filter(|(_, &ct)| ct >= 2).count();
    println!("{}", res);
    Ok(())
}

struct LineSegment {
    start: (isize, isize),
    end: (isize, isize),
}

impl LineSegment {
    fn points_covered(&self) -> Vec<(isize, isize)> {
        let mut res = Vec::new();

        let delta_x = self.end.0 - self.start.0;
        let delta_y = self.end.1 - self.start.1;
        let inc_x = if delta_x == 0 {
            0
        } else {
            delta_x / delta_x.abs()
        };
        let inc_y = if delta_y == 0 {
            0
        } else {
            delta_y / delta_y.abs()
        };
        let mut curr = self.start;
        while curr != self.end {
            res.push(curr);
            curr.0 += inc_x;
            curr.1 += inc_y;
        }
        res.push(curr);
        res
    }

    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn is_perfect_diag(&self) -> bool {
        (self.start.0 - self.end.0).abs() == (self.start.1 - self.end.1).abs()
    }
}

impl FromStr for LineSegment {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(" -> ").ok_or("bad input")?;
        let (start_str, end_str) = parts;

        let start_coords = start_str
            .split_once(",")
            .ok_or("error parsing coordinate pair")?;

        let (x, y) = start_coords;
        let start = (
            x.parse().map_err(|_| "error parsing coordinate")?,
            y.parse().map_err(|_| "error parsing coordinate")?,
        );

        let end_coords = end_str
            .split_once(",")
            .ok_or("error parsing coordinate pair")?;

        let (x, y) = end_coords;
        let end = (
            x.parse().map_err(|_| "error parsing coordinate")?,
            y.parse().map_err(|_| "error parsing coordinate")?,
        );

        Ok(LineSegment { start, end })
    }
}
