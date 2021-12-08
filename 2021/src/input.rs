use std::{error::Error, fmt::Display, fs, marker::PhantomData, str::FromStr};

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

pub struct InputReader<'a, T> {
    delim: &'a str,
    line_bounds: (usize, usize),
    marker: PhantomData<T>,
}

impl<'a, T> Default for InputReader<'a, T> {
    fn default() -> Self {
        Self {
            delim: "\n",
            line_bounds: (usize::MIN, usize::MAX),
            marker: PhantomData,
        }
    }
}

impl<'a, T> InputReader<'a, T> {
    pub fn with_delim(self, delim: &'a str) -> Self {
        Self { delim, ..self }
    }

    pub fn with_line_bounds(self, min: usize, max: usize) -> Self {
        Self {
            line_bounds: (min, max),
            ..self
        }
    }

    pub fn read(self, path: &str) -> Result<Vec<T>, InputError>
    where
        T: FromStr,
    {
        let ls: Vec<String> = fs::read_to_string(path)
            .map_err(|err| InputError::FileReadError(err.to_string()))?
            .lines()
            .skip(self.line_bounds.0)
            .take(self.line_bounds.1 - self.line_bounds.0)
            .map(|s| s.to_string())
            .collect();
        ls.join("\n")
            .split(self.delim)
            .map(|s| {
                s.trim()
                    .parse()
                    .map_err(|_| InputError::ParseError(s.to_string()))
            })
            .collect()
    }
}

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
