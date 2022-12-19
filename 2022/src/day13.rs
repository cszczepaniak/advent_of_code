use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace0},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, terminated, tuple},
    Finish, IResult,
};

pub fn part_one(input: &str) -> usize {
    let mut sum = 0;
    for (i, p) in input
        .split("\n\n")
        .map(|s| parse_pair(s).unwrap())
        .enumerate()
    {
        if let Some(res) = compare_nodes(&p.left, &p.right) {
            if res {
                sum += i + 1;
            }
        }
    }

    sum
}

pub fn part_two(input: &str) -> usize {
    let mut packets = Vec::new();
    for p in input.split("\n\n").map(|s| parse_pair(s).unwrap()) {
        packets.push(p.left);
        packets.push(p.right);
    }
    let (_, marker_one) = parse_node("[[2]]").finish().unwrap();
    packets.push(marker_one);
    let (_, marker_two) = parse_node("[[6]]").finish().unwrap();
    packets.push(marker_two);

    packets.sort();

    let (marker_one, _) = packets
        .iter()
        .enumerate()
        .find(|&(_, item)| &Node::Vector(vec![Node::Vector(vec![Node::Scalar(2)])]) == item)
        .unwrap();

    let (marker_two, _) = packets
        .iter()
        .enumerate()
        .find(|&(_, item)| &Node::Vector(vec![Node::Vector(vec![Node::Scalar(6)])]) == item)
        .unwrap();

    (marker_one + 1) * (marker_two + 1)
}

#[derive(Debug)]
struct Pair {
    left: Node,
    right: Node,
}

fn parse_pair(input: &str) -> anyhow::Result<Pair> {
    let (input, left) = terminated(parse_node, multispace0)(input)
        .finish()
        .map_err(|_| anyhow::anyhow!("could not parse left side of pair"))?;

    let (_, right) = terminated(parse_node, multispace0)(input)
        .finish()
        .map_err(|_| anyhow::anyhow!("could not parse right side of pair"))?;

    Ok(Pair { left, right })
}

#[derive(Debug, PartialEq, Eq)]
enum Node {
    Scalar(usize),
    Vector(Vec<Node>),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let res = compare_nodes(self, other);
        match res {
            Some(res) if res => Some(std::cmp::Ordering::Less),
            Some(_) => Some(std::cmp::Ordering::Greater),
            None => Some(std::cmp::Ordering::Equal),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn compare_nodes(left: &Node, right: &Node) -> Option<bool> {
    match (left, right) {
        (Node::Scalar(l), Node::Scalar(r)) => compare_int(l, r),
        (Node::Scalar(l), Node::Vector(r)) => {
            compare_node_iters(vec![Node::Scalar(*l)].iter(), r.iter())
        }
        (Node::Vector(l), Node::Scalar(r)) => {
            compare_node_iters(l.iter(), vec![Node::Scalar(*r)].iter())
        }
        (Node::Vector(l), Node::Vector(r)) => compare_node_iters(l.iter(), r.iter()),
    }
}

fn compare_node_iters<'a>(
    left: impl Iterator<Item = &'a Node>,
    mut right: impl Iterator<Item = &'a Node>,
) -> Option<bool> {
    for l_n in left {
        if let Some(r_n) = right.next() {
            if let Some(res) = compare_nodes(&l_n, &r_n) {
                return Some(res);
            }
        } else {
            // The right iterator ran out of items first; the items are not in the right order
            return Some(false);
        }
    }

    if let Some(_) = right.next() {
        // The left iterator ran out of items first; the items are in the right order
        return Some(true);
    }

    // The lists were identical. No conclusion.
    None
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let res = parse_number(input);
    if let Ok((input, n)) = res {
        return Ok((input, Node::Scalar(n)));
    }

    let (input, ns) = delimited(
        tag("["),
        separated_list0(tuple((tag(","), multispace0)), parse_node),
        tag("]"),
    )(input)?;
    Ok((input, Node::Vector(ns)))
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map(complete::u32, |n| n as usize)(input)
}

fn compare_int(l: &usize, r: &usize) -> Option<bool> {
    if l < r {
        Some(true)
    } else if l > r {
        Some(false)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    use super::*;

    #[test]
    fn test_part_one_example() {
        let p = parse_pair(
            "[1,1,3,1,1]
            [1,1,5,1,1]",
        )
        .unwrap();

        assert_eq!(Some(true), compare_nodes(&p.left, &p.right));

        let p = parse_pair(
            "[[1],[2,3,4]]
            [[1],4]",
        )
        .unwrap();

        assert_eq!(Some(true), compare_nodes(&p.left, &p.right));

        let p = parse_pair(
            "[9]
            [[8,7,6]]",
        )
        .unwrap();

        assert_eq!(Some(false), compare_nodes(&p.left, &p.right));

        let p = parse_pair(
            "[[4,4],4,4]
            [[4,4],4,4,4]",
        )
        .unwrap();

        assert_eq!(Some(true), compare_nodes(&p.left, &p.right));

        let p = parse_pair(
            "[7,7,7,7]
            [7,7,7]",
        )
        .unwrap();

        assert_eq!(Some(false), compare_nodes(&p.left, &p.right));

        let p = parse_pair(
            "[]
            [3]",
        )
        .unwrap();

        assert_eq!(Some(true), compare_nodes(&p.left, &p.right));

        let p = parse_pair(
            "[[[]]]
            [[]]",
        )
        .unwrap();

        assert_eq!(Some(false), compare_nodes(&p.left, &p.right));

        let p = parse_pair(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]",
        )
        .unwrap();

        assert_eq!(Some(false), compare_nodes(&p.left, &p.right));
    }

    #[test]
    fn test_parse_node() {
        let (_, n) = parse_node("[]").finish().unwrap();
        assert_eq!(Node::Vector(vec![]), n);

        let (_, n) = parse_node("[0]").finish().unwrap();
        assert_eq!(Node::Vector(vec![Node::Scalar(0)]), n);

        let (_, n) = parse_node("[0, 1]").finish().unwrap();
        assert_eq!(Node::Vector(vec![Node::Scalar(0), Node::Scalar(1)]), n);

        let (_, n) = parse_node("[[]]").finish().unwrap();
        assert_eq!(Node::Vector(vec![Node::Vector(vec![])]), n);

        let (_, n) = parse_node("[[], 1]").finish().unwrap();
        assert_eq!(Node::Vector(vec![Node::Vector(vec![]), Node::Scalar(1)]), n);

        let (_, n) = parse_node("[1,[2,[3,[4,[5,6,7]]]],8,9]").finish().unwrap();
        assert_eq!(
            Node::Vector(vec![
                Node::Scalar(1),
                Node::Vector(vec![
                    Node::Scalar(2),
                    Node::Vector(vec![
                        Node::Scalar(3),
                        Node::Vector(vec![
                            Node::Scalar(4),
                            Node::Vector(vec![Node::Scalar(5), Node::Scalar(6), Node::Scalar(7),])
                        ])
                    ])
                ]),
                Node::Scalar(8),
                Node::Scalar(9),
            ],),
            n
        );
    }
}
