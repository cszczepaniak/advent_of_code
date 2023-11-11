use std::{fs, ops::AddAssign};

fn main() {
    let mut digits = Vec::new();
    for l in fs::read_to_string("input/day11.txt").unwrap().lines() {
        let mut row = Vec::new();
        for ch in l.chars() {
            row.push(ch.to_digit(10).unwrap());
        }
        digits.push(row);
    }

    let mut b = Board::new(digits);
    let mut n = 1;
    loop {
        b.step();
        if b.check_all() {
            break;
        }
        n += 1;
    }
    println!("{:?}", n);
}

#[derive(Clone, Copy, Debug)]
enum FlashState {
    NotFlashed(u32),
    Flashed,
}

impl AddAssign<u32> for FlashState {
    fn add_assign(&mut self, rhs: u32) {
        match self {
            FlashState::NotFlashed(n) => {
                *n += rhs;
                if *n >= 10 {
                    *self = FlashState::Flashed
                }
            }
            FlashState::Flashed => {}
        }
    }
}

#[derive(Debug)]
struct Board {
    digits: Vec<Vec<FlashState>>,
    width: usize,
    height: usize,
    flashes: usize,
}

impl Board {
    fn new(digits: Vec<Vec<u32>>) -> Board {
        Self {
            flashes: 0,
            height: digits.len(),
            width: digits.first().unwrap().len(),
            digits: digits
                .into_iter()
                .map(|d| d.into_iter().map(|d| FlashState::NotFlashed(d)).collect())
                .collect(),
        }
    }

    fn step(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.flash_at(i as isize, j as isize);
            }
        }
        for i in 0..self.height {
            for j in 0..self.width {
                if let FlashState::Flashed = self.digits[i][j] {
                    self.digits[i][j] = FlashState::NotFlashed(0);
                }
            }
        }
    }

    fn check_all(&self) -> bool {
        for i in 0..self.height {
            for j in 0..self.width {
                if let FlashState::NotFlashed(n) = self.digits[i][j] {
                    if n > 0 {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn get_at(&self, row: isize, col: isize) -> Option<FlashState> {
        if row < 0 || col < 0 || row > (self.height - 1) as isize || col > (self.width - 1) as isize
        {
            None
        } else {
            Some(self.digits[row as usize][col as usize])
        }
    }

    fn inc_at(&mut self, row: isize, col: isize) {
        if row >= 0 && col >= 0 && row < self.height as isize && col < self.width as isize {
            self.digits[row as usize][col as usize] += 1;
        }
    }

    fn flash_at(&mut self, row: isize, col: isize) {
        if let None = self.get_at(row, col) {
            return;
        }
        if let FlashState::Flashed = self.get_at(row, col).unwrap() {
            return;
        }
        self.inc_at(row, col);
        if let FlashState::Flashed = self.get_at(row, col).unwrap() {
            self.flashes += 1;
            self.flash_at(row - 1, col - 1);
            self.flash_at(row - 1, col);
            self.flash_at(row - 1, col + 1);
            self.flash_at(row, col - 1);
            self.flash_at(row, col + 1);
            self.flash_at(row + 1, col - 1);
            self.flash_at(row + 1, col);
            self.flash_at(row + 1, col + 1);
        }
    }
}
