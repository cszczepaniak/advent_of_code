use std::collections::HashSet;

use nom::{
    bytes::complete::tag, character::complete, combinator::map, multi::separated_list1,
    sequence::separated_pair, Finish, IResult,
};

pub fn part_one(input: &str) -> usize {
    let mut hs: HashSet<Point> = HashSet::new();
    let mut max_y = 0;
    for l in input.lines() {
        let (_, points) = parse_points(l).finish().unwrap();
        for p in points.iter() {
            if p.y > max_y {
                max_y = p.y;
            }
        }
        for (p1, p2) in points.iter().zip(points.iter().skip(1)) {
            fill_rocks(&mut hs, *p1, *p2);
        }
    }

    let mut count = 0;
    while let Some(pos) = simulate_sand(&hs, max_y + 1, |_| None) {
        hs.insert(pos);
        count += 1;
    }

    count
}

pub fn part_two(input: &str) -> usize {
    let mut hs: HashSet<Point> = HashSet::new();
    let mut max_y = 0;
    for l in input.lines() {
        let (_, points) = parse_points(l).finish().unwrap();
        for p in points.iter() {
            if p.y > max_y {
                max_y = p.y;
            }
        }
        for (p1, p2) in points.iter().zip(points.iter().skip(1)) {
            fill_rocks(&mut hs, *p1, *p2);
        }
    }

    let mut count = 0;
    while let Some(pos) = simulate_sand(&hs, max_y + 2, |pos| Some(pos)) {
        hs.insert(pos);
        count += 1;
    }

    count + 1
}

fn simulate_sand<F>(mapping: &HashSet<Point>, max_y: u32, handle_floor: F) -> Option<Point>
where
    F: Fn(Point) -> Option<Point>,
{
    let start = Point { x: 500, y: 0 };
    let mut pos = start;

    loop {
        // Can we go directly down?
        let mut attempt = pos;
        attempt.y += 1;

        if attempt.y >= max_y {
            return handle_floor(pos);
        }

        if !mapping.contains(&attempt) {
            pos = attempt;
            continue;
        }

        // Can we go down-left?
        attempt.x -= 1;
        if !mapping.contains(&attempt) {
            pos = attempt;
            continue;
        }

        // Can we go down-right?
        attempt.x += 2;
        if !mapping.contains(&attempt) {
            pos = attempt;
            continue;
        }

        // We can't go anywhere. We've come to rest.
        if pos == start {
            return None;
        } else {
            return Some(pos);
        }
    }
}

fn fill_rocks(mapping: &mut HashSet<Point>, mut p1: Point, mut p2: Point) {
    if p1.x > p2.x || p1.y > p2.y {
        (p2, p1) = (p1, p2);
    }

    for x in p1.x..=p2.x {
        for y in p1.y..=p2.y {
            mapping.insert(Point { x, y });
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(complete::u32, tag(","), complete::u32),
        |(x, y)| Point { x, y },
    )(input)
}

fn parse_points(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), parse_point)(input)
}
