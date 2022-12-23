use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::map,
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

pub fn part_one(input: &str) -> isize {
    count_excluded_points(input, 2_000_000)
}

pub fn part_two(input: &str) -> anyhow::Result<isize> {
    let beacon_pos =
        find_distress_beacon(input, 4_000_000).ok_or(anyhow::anyhow!("no beacon found"))?;

    Ok(beacon_pos.x * 4_000_000 + beacon_pos.y)
}

fn merged_ranges_for_row(readings: &Vec<Reading>, row: isize) -> Vec<InclusiveRange> {
    let mut ranges = Vec::new();
    for r in readings.iter() {
        let range = r.x_range_for_y(row);
        if let Some(range) = range {
            ranges.push(range);
        }
    }

    merge_ranges(ranges.iter().collect())
}

fn count_excluded_points(input: &str, row: isize) -> isize {
    let mut readings = Vec::new();
    let mut beacons_by_row: HashMap<isize, HashSet<isize>> = HashMap::new();
    for l in input.lines() {
        let (_, r) = parse_reading(l).finish().unwrap();
        beacons_by_row
            .entry(r.beacon.y)
            .or_default()
            .insert(r.beacon.x);
        readings.push(r);
    }

    let ranges = merged_ranges_for_row(&readings, 2_000_000);
    let beacons: Vec<isize> = beacons_by_row
        .get(&row)
        .into_iter()
        .flatten()
        .map(|&n| n)
        .collect();

    let mut num = 0;
    for r in ranges {
        let mut len = r.stop - r.start + 1;
        for b in beacons.iter() {
            if r.contains(*b) {
                len -= 1;
            }
        }
        num += len;
    }

    num
}

fn merge_ranges(mut ranges: Vec<&InclusiveRange>) -> Vec<InclusiveRange> {
    let mut res = Vec::new();
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut curr = *ranges[0];
    for i in 0..ranges.len() {
        let merged = curr.merge(*ranges[i]);
        if let Some(m) = merged {
            curr = m;
        } else {
            res.push(curr);
            curr = *ranges[i];
        }
    }
    res.push(curr);
    res
}

fn find_distress_beacon(input: &str, max_coord: isize) -> Option<Point> {
    let mut readings = Vec::new();
    let mut beacons_by_row: HashMap<isize, HashSet<isize>> = HashMap::new();
    for l in input.lines() {
        let (_, r) = parse_reading(l).finish().unwrap();
        beacons_by_row
            .entry(r.beacon.y)
            .or_default()
            .insert(r.beacon.x);
        readings.push(r);
    }

    for y in 0..=max_coord {
        let ranges = merged_ranges_for_row(&readings, y);
        if ranges
            .iter()
            .any(|r| r.contains(0) && r.contains(max_coord))
        {
            // If any of the ranges contain the entire search row, the distress beacon isn't here.
            continue;
        }

        let mut empty = 0;
        let mut num_empty = 0;
        for range in ranges {
            if range.contains(empty) {
                empty = range.stop + 1;
            } else {
                num_empty += 1;
                if num_empty > 1 {
                    break;
                }
            }
            if range.stop >= max_coord {
                break;
            }
        }
        if num_empty == 1 && empty <= max_coord {
            return Some(Point { x: empty, y });
        }
    }

    None
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct InclusiveRange {
    start: isize,
    stop: isize,
}

impl InclusiveRange {
    fn contains(&self, val: isize) -> bool {
        val >= self.start && val <= self.stop
    }

    fn merge(self, other: Self) -> Option<Self> {
        if self.contains(other.start) {
            Some(Self {
                start: self.start,
                stop: self.stop.max(other.stop),
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Reading {
    sensor: Point,
    beacon: Point,
}

impl Reading {
    fn x_range_for_y(&self, y: isize) -> Option<InclusiveRange> {
        let radius = self.sensor.distance_to(&self.beacon);
        let dist_to_sensor = (y - self.sensor.y).abs();
        if dist_to_sensor > radius {
            return None;
        }
        let delta_x = -dist_to_sensor + radius;
        Some(InclusiveRange {
            start: self.sensor.x - delta_x,
            stop: self.sensor.x + delta_x,
        })
    }
}

fn parse_reading(input: &str) -> IResult<&str, Reading> {
    let (input, sensor) = delimited(tag("Sensor at "), parse_point, tag(": "))(input)?;
    let (input, beacon) = preceded(tag("closest beacon is at "), parse_point)(input)?;
    Ok((input, Reading { sensor, beacon }))
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        tuple((
            preceded(tag("x="), complete::i32),
            preceded(tag(", y="), complete::i32),
        )),
        |(x, y)| Point {
            x: x as isize,
            y: y as isize,
        },
    )(input)
}

#[derive(Debug, Default, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn distance_to(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let ans = count_excluded_points(input, 10);
        assert_eq!(26, ans);
    }

    #[test]
    fn test_part_two_example() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let ans = find_distress_beacon(input, 20);
        assert_eq!(Some(Point { x: 14, y: 11 }), ans);
    }

    #[test]
    fn test_x_range_for_y() {
        let r = Reading {
            sensor: Point { x: 8, y: 7 },
            beacon: Point { x: 2, y: 10 },
        };

        let range = r.x_range_for_y(7);
        assert_eq!(
            Some(InclusiveRange {
                start: -1,
                stop: 17,
            }),
            range
        );

        let range = r.x_range_for_y(-2);
        assert_eq!(Some(InclusiveRange { start: 8, stop: 8 }), range);

        let range = r.x_range_for_y(0);
        assert_eq!(Some(InclusiveRange { start: 6, stop: 10 }), range);

        let range = r.x_range_for_y(16);
        assert_eq!(Some(InclusiveRange { start: 8, stop: 8 }), range);

        let range = r.x_range_for_y(14);
        assert_eq!(Some(InclusiveRange { start: 6, stop: 10 }), range);
    }
}
