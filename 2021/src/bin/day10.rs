use std::{collections::LinkedList, fs};

use advent::input::InputError;

fn main() -> Result<(), InputError> {
    let mut score = 0;
    for line in fs::read_to_string("./input/day10.txt")
        .map_err(|_| InputError::FileReadError("could not read input file".to_string()))?
        .lines()
    {
        if let Some(ch) = first_invalid_character(line) {
            match ch {
                ')' => score += 3,
                ']' => score += 57,
                '}' => score += 1197,
                '>' => score += 25137,
                _ => {}
            }
        }
    }
    println!("{}", score);

    let mut scores = Vec::new();
    for line in fs::read_to_string("./input/day10.txt")
        .map_err(|_| InputError::FileReadError("could not read input file".to_string()))?
        .lines()
        .filter(|l| first_invalid_character(l).is_none())
    {
        scores.push(score_to_complete(line));
    }

    scores.sort_unstable();
    println!("middle score: {}", scores[scores.len() / 2]);

    Ok(())
}

fn first_invalid_character(line: &str) -> Option<char> {
    let mut stack = LinkedList::new();

    for ch in line.chars() {
        if is_opening(ch) {
            stack.push_back(ch);
            continue;
        }
        if let Some(last) = stack.pop_back() {
            if last != get_matching(ch) {
                return Some(ch);
            }
        }
    }
    None
}

fn score_to_complete(line: &str) -> usize {
    let mut stack = LinkedList::new();

    for ch in line.chars() {
        if is_opening(ch) {
            stack.push_back(ch);
            continue;
        }
        stack.pop_back();
    }

    let mut score = 0;
    while let Some(ch) = stack.pop_back() {
        score *= 5;
        match ch {
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            _ => panic!("invalid char!"),
        }
    }
    score
}

fn is_opening(ch: char) -> bool {
    match ch {
        '(' | '<' | '{' | '[' => true,
        _ => false,
    }
}

fn get_matching(ch: char) -> char {
    match ch {
        '(' => ')',
        ')' => '(',
        '<' => '>',
        '>' => '<',
        '{' => '}',
        '}' => '{',
        '[' => ']',
        ']' => '[',
        _ => panic!("unexpected character"),
    }
}
