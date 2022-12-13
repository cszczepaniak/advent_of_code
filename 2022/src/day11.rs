use std::{cell::RefCell, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace0},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, preceded},
    Finish, IResult,
};

pub fn part_one(input: &str) -> anyhow::Result<usize> {
    let monkeys = parse_monkeys(input)?;

    let res = run(monkeys, 20, |n| n / 3);
    Ok(res)
}

pub fn part_two(input: &str) -> anyhow::Result<usize> {
    let monkeys = parse_monkeys(input)?;
    let lcm: usize = monkeys.iter().map(|m| m.borrow().test_divisor).product();

    let res = run(monkeys, 10000, |n| n % lcm);
    Ok(res)
}

fn parse_monkeys(input: &str) -> anyhow::Result<Vec<RefCell<Monkey>>> {
    let mut monkeys = Vec::new();
    for ch in input.split("\n\n") {
        let m: Monkey = ch.parse()?;
        monkeys.push(RefCell::new(m));
    }
    Ok(monkeys)
}

fn run<F>(mut monkeys: Vec<RefCell<Monkey>>, turns: usize, reduce: F) -> usize
where
    F: Fn(usize) -> usize,
{
    for _ in 0..turns {
        for i in 0..monkeys.len() {
            process_items(i, &monkeys, &reduce);
        }
    }

    monkeys.sort_by(|a, b| b.borrow().items_inspected.cmp(&a.borrow().items_inspected));

    monkeys[0].borrow().items_inspected * monkeys[1].borrow().items_inspected
}

fn process_items<F>(idx: usize, monkeys: &Vec<RefCell<Monkey>>, reduce: F)
where
    F: Fn(usize) -> usize,
{
    let mut me = monkeys[idx].borrow_mut();
    let num_items = me.items.len();

    for item in me.items.iter() {
        let mut new_item = me.op.eval(*item);
        new_item = reduce(new_item);

        let mut other = if new_item % me.test_divisor == 0 {
            monkeys[me.if_true].borrow_mut()
        } else {
            monkeys[me.if_false].borrow_mut()
        };

        other.items.push(new_item);
    }
    me.items.drain(0..num_items);
    me.items_inspected += num_items;
}

struct Monkey {
    items: Vec<usize>,
    op: Operation,
    items_inspected: usize,
    test_divisor: usize,
    if_true: usize,
    if_false: usize,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_monkey(s)
            .finish()
            .map(|(_, m)| m)
            .map_err(|_| anyhow::anyhow!("failed to parse monkey"))
    }
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = preceded(multispace0, parse_index)(input)?;
    let (input, items) = preceded(multispace0, parse_starting_items)(input)?;
    let (input, op) = preceded(multispace0, parse_operation)(input)?;
    let (input, test_divisor) = preceded(multispace0, parse_divisible_by)(input)?;
    let (input, if_true) = preceded(multispace0, parse_if_true)(input)?;
    let (input, if_false) = preceded(multispace0, parse_if_false)(input)?;

    Ok((
        input,
        Monkey {
            items,
            op,
            items_inspected: 0,
            test_divisor,
            if_true,
            if_false,
        },
    ))
}

fn usize(input: &str) -> IResult<&str, usize> {
    map(complete::u32, |n| n as usize)(input)
}

fn parse_index(input: &str) -> IResult<&str, usize> {
    let (input, index) = delimited(tag("Monkey "), usize, tag(":"))(input)?;
    Ok((input, index))
}

fn parse_starting_items(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, items) =
        preceded(tag("Starting items: "), separated_list0(tag(", "), usize))(input)?;

    Ok((input, items))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("Operation: new = ")(input)?;
    let (input, lhs) = parse_expression(input)?;
    let (input, op) = parse_operator(input)?;
    let (input, rhs) = parse_expression(input)?;

    Ok((input, Operation { rhs, lhs, op }))
}

struct Operation {
    rhs: Expression,
    lhs: Expression,
    op: Operator,
}

impl Operation {
    fn eval(&self, n: usize) -> usize {
        self.op.eval(self.lhs.eval(n), self.rhs.eval(n))
    }
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    let res: IResult<_, _, nom::error::Error<&str>> = tag(" * ")(input);
    if let Ok((input, _)) = res {
        return Ok((input, Operator::Multiply));
    }
    let (input, _) = tag(" + ")(input)?;
    Ok((input, Operator::Add))
}

enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn eval(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Multiply => lhs * rhs,
        }
    }
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    let map_input = map(tag("old"), |_| Expression::Input);
    let map_const = map(usize, |n| Expression::Const(n));
    alt((map_input, map_const))(input)
}

enum Expression {
    Input,
    Const(usize),
}

impl Expression {
    fn eval(&self, input: usize) -> usize {
        match self {
            Expression::Input => input,
            Expression::Const(val) => *val,
        }
    }
}

fn parse_divisible_by(input: &str) -> IResult<&str, usize> {
    preceded(tag("Test: divisible by "), usize)(input)
}

fn parse_if_true(input: &str) -> IResult<&str, usize> {
    preceded(tag("If true: throw to monkey "), usize)(input)
}

fn parse_if_false(input: &str) -> IResult<&str, usize> {
    preceded(tag("If false: throw to monkey "), usize)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_parse_monkey() {
        let input = "Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3";

        let m: Monkey = input.parse().unwrap();

        assert_eq!(vec![79, 60, 97], m.items);
        assert_eq!(4, m.op.eval(2));
        assert_eq!(13, m.test_divisor);
        assert_eq!(1, m.if_true);
        assert_eq!(3, m.if_false);
    }

    #[test]
    fn test_one() {
        assert_eq!(10605, part_one(EXAMPLE).unwrap())
    }

    #[test]
    fn test_two() {
        assert_eq!(2_713_310_158, part_two(EXAMPLE).unwrap())
    }
}
