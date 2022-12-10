use std::fmt::Display;

pub trait Solution<'a, T, R, E>
where
    R: Display,
{
    fn solve(&self, input: &'a str) -> Result<R, E>;
}

impl<'a, R, F> Solution<'a, R, R, anyhow::Error> for F
where
    R: Display,
    F: Fn(&'a str) -> R,
{
    fn solve(&self, input: &'a str) -> Result<R, anyhow::Error> {
        Ok(self(input))
    }
}

impl<'a, R, E, F> Solution<'a, Result<R, E>, R, E> for F
where
    R: Display,
    F: Fn(&'a str) -> Result<R, E>,
{
    fn solve(&self, input: &'a str) -> Result<R, E> {
        self(input)
    }
}

pub fn run_solution<'a, R, E, T, S>(input: &'a str, s: S) -> Result<(), E>
where
    R: Display,
    S: Solution<'a, T, R, E>,
{
    let res = s.solve(input)?;
    println!("{}", res);

    Ok(())
}
