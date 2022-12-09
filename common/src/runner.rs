use std::fmt::Display;

pub trait Solution<T, R, E>
where
    R: Display,
{
    fn solve(&self, input: &str) -> Result<R, E>;
}

impl<R, E, F> Solution<Result<R, E>, R, E> for F
where
    R: Display,
    F: Fn(&str) -> Result<R, E>,
{
    fn solve(&self, input: &str) -> Result<R, E> {
        self(input)
    }
}

pub fn run_solution<R, E, T, S>(input: &str, s: S) -> Result<(), E>
where
    R: Display,
    S: Solution<T, R, E>,
{
    let res = s.solve(input)?;
    println!("{}", res);

    Ok(())
}

#[macro_export]
macro_rules! run {
    ($input:expr, part1: $f1:ident, part2: $f2:ident) => {
        println!("Running part 1...");
        let r = common::runner::run_solution($input, $f1);
        if let Err(ref err) = r {
            println!("Error running part 1: {}", err);
        }

        println!("Running part 2...");
        let r = common::runner::run_solution($input, $f2);
        if let Err(ref err) = r {
            println!("Error running part 2: {}", err);
        }
    };
}

#[macro_export]
macro_rules! runner_main {
    ($year:literal, day $day:literal, part1: $f1:ident, part2: $f2:ident) => {
        fn main() {
            let input = common::network::get_input($year, $day);
            if let Err(ref err) = input {
                println!("Error getting input: {}", err);
            }
            let input = input.unwrap();

            common::run!(&input, part1: $f1, part2: $f2);
        }
    };
}

#[macro_export]
macro_rules! runner_main_input {
    ($input:expr, part1: $f1:ident, part2: $f2:ident) => {
        fn main() {
            common::run!($input, part1: $f1, part2: $f2);
        }
    };
}
