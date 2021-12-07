use std::{error::Error, fmt::Display, fs, str::FromStr};

#[derive(Debug)]
pub enum InputError {
    ParseError(String),
    FileReadError(String),
}

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::ParseError(s) => write!(f, "ParseError: {}", s),
            InputError::FileReadError(s) => write!(f, "FileReadError: {}", s),
        }
    }
}

impl Error for InputError {}

pub fn read_input<T>(path: &str) -> Result<Vec<T>, InputError>
where
    T: FromStr,
{
    read_input_delim(path, "\n")
}

pub fn read_input_delim<T>(path: &str, delim: &str) -> Result<Vec<T>, InputError>
where
    T: FromStr,
{
    fs::read_to_string(path)
        .map_err(|err| InputError::FileReadError(err.to_string()))?
        .split(delim)
        .map(|s| {
            s.trim()
                .parse()
                .map_err(|_| InputError::ParseError(s.to_string()))
        })
        .collect()
}
