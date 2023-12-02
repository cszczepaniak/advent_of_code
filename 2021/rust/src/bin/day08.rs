use std::{collections::HashSet, iter::FromIterator, str::FromStr};

use advent::input::{self, InputError};

fn main() -> Result<(), InputError> {
    let values: Vec<Value> = input::read_input("./input/day08.txt")?;
    let res: usize = values
        .iter()
        .map(|val| {
            val.output
                .iter()
                .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
                .count()
        })
        .sum();
    println!("{}", res);

    let mut res = 0;
    for val in values.iter() {
        res += get_output_number(&val.input, &val.output);
    }
    println!("{}", res);

    Ok(())
}

fn process_three_five_nine(inputs: &Vec<String>, one: &str) -> (String, String, String, Segment) {
    let five_long: Vec<HashSet<char>> = inputs
        .iter()
        .filter(|s| s.len() == 5)
        .map(|s| s.chars().collect())
        .collect();
    let six_long: Vec<HashSet<char>> = inputs
        .iter()
        .filter(|s| s.len() == 6)
        .map(|s| s.chars().collect())
        .collect();

    let one_set: HashSet<char> = one.chars().collect();
    let mut seg2 = '0';
    let mut three_five = Vec::new();
    let mut three_five_str = Vec::new();
    for s in six_long.iter() {
        for f in five_long.iter() {
            let res: Vec<&char> = s.difference(f).collect();
            if res.iter().count() == 1 {
                let c = **res.first().unwrap();
                if one_set.contains(&c) {
                    seg2 = c;
                }
                three_five.push((f, c));
                three_five_str.push(String::from_iter(f));
            }
        }
    }
    let five: Vec<String> = three_five
        .iter()
        .filter(|(_, seg)| one_set.contains(seg))
        .map(|(hs, _)| String::from_iter(hs.iter()))
        .collect();
    let five = five[0].clone();
    let three = three_five_str.iter().find(|s| **s != five).unwrap().clone();
    let three_hs: HashSet<char> = three.chars().collect();
    let five_hs: HashSet<char> = five.chars().collect();
    let nine = String::from_iter(three_hs.union(&five_hs));
    (three, five, nine, seg2.into())
}

fn get_output_number(input: &Vec<String>, output: &Vec<String>) -> usize {
    let mut input_clone = input.clone();
    input_clone.iter_mut().for_each(|s| {
        let mut cs: Vec<char> = s.chars().collect();
        cs.sort_by(|a, b| a.cmp(b));
        *s = String::from_iter(cs);
    });
    let one = input.iter().find(|s| s.len() == 2).unwrap();
    let four = input.iter().find(|s| s.len() == 4).unwrap();
    let seven = input.iter().find(|s| s.len() == 3).unwrap();
    let eight = input.iter().find(|s| s.len() == 7).unwrap();

    let (three, five, nine, seg) = process_three_five_nine(input, one);
    let seg_char: char = seg.into();
    let six = String::from_iter(eight.chars().filter(|c| c != &seg_char));
    let two = get_two(one, &three, &five, eight);
    let mut zero = "".to_string();
    for inp in input.iter() {
        let mut found = None;
        for n in vec![one, &two, &three, four, &five, &six, seven, eight, &nine] {
            if compare_chars(inp, n) {
                found = Some(0);
                break;
            }
        }
        if let None = found {
            zero = inp.clone();
        }
    }

    let one = one.clone();
    let four = four.clone();
    let seven = seven.clone();
    let eight = eight.clone();
    let ss = SevenSegment {
        zero,
        one,
        two,
        three,
        four,
        five,
        six,
        seven,
        eight,
        nine,
    };
    ss.map_output(output)
}

fn compare_chars(a: &str, b: &str) -> bool {
    let a_set: HashSet<char> = a.chars().collect();
    let b_set: HashSet<char> = b.chars().collect();
    a_set.difference(&b_set).count() == 0 && b_set.difference(&a_set).count() == 0
}

fn get_two(one: &str, three: &str, five: &str, eight: &str) -> String {
    let o: HashSet<char> = one.chars().collect();
    let t: HashSet<char> = three.chars().collect();
    let f: HashSet<char> = five.chars().collect();
    let e: HashSet<char> = eight.chars().collect();

    let parts: HashSet<char> = e.difference(&f).map(|c| *c).collect();
    let other_parts: HashSet<char> = t.difference(&o).map(|c| *c).collect();
    String::from_iter(parts.union(&other_parts))
}

/*
 aaaa      0000
b    c    1    2
b    c    1    2
 ....      3333
e    f    4    5
e    f    4    5
 gggg      6666
*/
struct SevenSegment {
    zero: String,
    one: String,
    two: String,
    three: String,
    four: String,
    five: String,
    six: String,
    seven: String,
    eight: String,
    nine: String,
}

impl SevenSegment {
    fn map_output(&self, output: &Vec<String>) -> usize {
        let mut mult = 1;
        let mut res = 0;
        for s in output.iter().rev() {
            let dig = self.map_digit(s);
            res += dig * mult;
            mult *= 10;
        }
        res
    }

    fn map_digit(&self, digit: &str) -> usize {
        if compare_chars(&digit, &self.zero) {
            return 0;
        }
        if compare_chars(&digit, &self.one) {
            return 1;
        }
        if compare_chars(&digit, &self.two) {
            return 2;
        }
        if compare_chars(&digit, &self.three) {
            return 3;
        }
        if compare_chars(&digit, &self.four) {
            return 4;
        }
        if compare_chars(&digit, &self.five) {
            return 5;
        }
        if compare_chars(&digit, &self.six) {
            return 6;
        }
        if compare_chars(&digit, &self.seven) {
            return 7;
        }
        if compare_chars(&digit, &self.eight) {
            return 8;
        }
        if compare_chars(&digit, &self.nine) {
            return 9;
        }
        usize::MAX
    }
}

#[derive(Debug)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl From<char> for Segment {
    fn from(c: char) -> Self {
        match c {
            'a' => Segment::A,
            'b' => Segment::B,
            'c' => Segment::C,
            'd' => Segment::D,
            'e' => Segment::E,
            'f' => Segment::F,
            'g' => Segment::G,
            _ => panic!("bad time"),
        }
    }
}

impl From<Segment> for char {
    fn from(s: Segment) -> Self {
        match s {
            Segment::A => 'a',
            Segment::B => 'b',
            Segment::C => 'c',
            Segment::D => 'd',
            Segment::E => 'e',
            Segment::F => 'f',
            Segment::G => 'g',
        }
    }
}

struct Value {
    input: Vec<String>,
    output: Vec<String>,
}

impl FromStr for Value {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_input, raw_output) = s.split_once(" | ").ok_or("invalid input")?;
        let mut input = Vec::new();
        for p in raw_input.splitn(10, " ") {
            input.push(p.to_string());
        }
        let mut output = Vec::new();
        for p in raw_output.splitn(4, " ") {
            output.push(p.to_string());
        }
        Ok(Self { input, output })
    }
}
