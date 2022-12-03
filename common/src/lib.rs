use std::{fs, io, str::FromStr};

pub fn parse_input_lines<T, E>(path: &str) -> Result<Vec<T>, E>
where
    T: FromStr<Err = E>,
    E: From<io::Error>,
{
    fs::read_to_string(path)?
        .lines()
        .map(|l| l.parse::<T>())
        .collect()
}

pub fn parse_input_delim<T, E>(path: &str, delim: &str) -> Result<Vec<T>, E>
where
    T: FromStr<Err = E>,
    E: From<io::Error>,
{
    fs::read_to_string(path)?
        .split(delim)
        .map(|l| l.parse::<T>())
        .collect()
}
