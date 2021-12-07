use std::{ops::Not, str::FromStr};

use advent::input::{self, InputError};

fn main() -> Result<(), InputError> {
    let bits: Vec<BitVec> = input::read_input("./input/day03.txt")?;

    let most_common = find_most_common(&bits);
    let gamma: usize = most_common.clone().into();
    let least_common = !most_common;
    let epsilon: usize = least_common.into();
    println!("{}", gamma * epsilon);

    // Part 2
    let mut bits_copy = bits.clone();
    let mut index = 0;
    while bits_copy.len() > 1 {
        let most_common = find_most_common(&bits_copy);
        bits_copy.retain(|bit_vec| bit_vec.0[index] == most_common.0[index]);
        index += 1;
    }
    let o2: usize = bits_copy.first().unwrap().into();

    let mut bits_copy = bits.clone();
    let mut index = 0;
    while bits_copy.len() > 1 {
        let most_common = !find_most_common(&bits_copy);
        bits_copy.retain(|bit_vec| bit_vec.0[index] == most_common.0[index]);
        index += 1;
    }
    let co2: usize = bits_copy.first().unwrap().into();
    println!("{}", co2 * o2);

    Ok(())
}

fn find_most_common(all: &Vec<BitVec>) -> BitVec {
    let mut bit_freq = vec![0; all[0].0.len()];
    for bit_vec in all.iter() {
        for (i, bit) in bit_vec.0.iter().enumerate() {
            match bit {
                Bit::One => bit_freq[i] += 1,
                Bit::Zero => bit_freq[i] -= 1,
            }
        }
    }
    bit_freq
        .iter()
        .map(|n| if n >= &0 { Bit::One } else { Bit::Zero })
        .collect::<Vec<_>>()
        .into()
}

#[derive(Clone, PartialEq)]
enum Bit {
    One,
    Zero,
}

impl Not for Bit {
    type Output = Bit;

    fn not(self) -> Self::Output {
        match self {
            Bit::One => Bit::Zero,
            Bit::Zero => Bit::One,
        }
    }
}

#[derive(Clone)]
struct BitVec(Vec<Bit>);

impl Not for BitVec {
    type Output = BitVec;

    fn not(self) -> Self::Output {
        let bits: Vec<_> = self.0.iter().map(|b| !b.clone()).collect();
        bits.into()
    }
}

impl From<BitVec> for usize {
    fn from(bv: BitVec) -> Self {
        Self::from(&bv)
    }
}

impl From<&BitVec> for usize {
    fn from(bv: &BitVec) -> Self {
        let mut res = 0;
        for (i, b) in bv.0.iter().rev().enumerate() {
            if let Bit::One = b {
                res |= 1 << i;
            }
        }
        res
    }
}

impl From<Vec<Bit>> for BitVec {
    fn from(v: Vec<Bit>) -> Self {
        Self(v)
    }
}

impl FromStr for BitVec {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<Bit>, _> = s
            .chars()
            .map(|c| match c {
                '0' => Ok(Bit::Zero),
                '1' => Ok(Bit::One),
                _ => Err("invalid bit"),
            })
            .collect();
        match res {
            Ok(v) => Ok(BitVec(v)),
            Err(s) => Err(s),
        }
    }
}
