use common::runner_main;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete,
    combinator::map,
    sequence::{separated_pair, tuple},
    Finish, IResult,
};

runner_main!(2022, day 7, part1: part_one, part2: part_two);

fn part_one(input: &str) -> anyhow::Result<usize> {
    const DIR_MAX: usize = 100000;

    let (tree, root) = build_tree(input)?;

    let mut size_of_dirs = 0usize;
    traverse(root, &tree, |s| {
        if s < DIR_MAX {
            size_of_dirs += s;
        }
    });
    Ok(size_of_dirs)
}

fn part_two(input: &str) -> anyhow::Result<usize> {
    const TOTAL_SPACE: usize = 70000000;
    const NEEDED_SPACE: usize = 30000000;

    let (tree, root) = build_tree(input)?;

    let root_size = traverse(root, &tree, |_| {});

    let curr_unused_space = TOTAL_SPACE - root_size;
    println!("unused space: {curr_unused_space}");

    let need_to_free = NEEDED_SPACE - curr_unused_space;
    println!("need to free: {need_to_free}");

    let mut min_to_free = usize::MAX;
    traverse(root, &tree, |s| {
        if s >= need_to_free && s < min_to_free {
            min_to_free = s;
        }
    });

    Ok(min_to_free)
}

fn build_tree(input: &str) -> anyhow::Result<(Tree<NodeData>, NodeId)> {
    let mut arena: Tree<NodeData> = Tree::new();
    let root = arena.new_node(NodeData::Dir("/"));
    let mut curr = root;

    for l in input.lines() {
        let (_, parsed) = parse_line(l)
            .finish()
            .map_err(|_| anyhow::anyhow!("error parsing command"))?;

        match parsed {
            InputLine::Command(c) => process_command(c, curr, &arena).map_or((), |n| curr = n),
            InputLine::FileSize(size) => process_file(curr, size, &mut arena),
            InputLine::Dir(d) => process_dir(curr, d, &mut arena),
        }
    }

    Ok((arena, root))
}

fn traverse<F>(root: NodeId, arena: &Tree<NodeData>, mut delegate: F) -> usize
where
    F: FnMut(usize),
{
    traverse_internal(root, arena, &mut delegate)
}

fn traverse_internal<F>(root: NodeId, arena: &Tree<NodeData>, delegate: &mut F) -> usize
where
    F: FnMut(usize),
{
    let mut total_for_level = 0usize;
    for c in root.children(&arena) {
        if let Some(data) = arena.get_data(c) {
            match data {
                NodeData::Dir(_) => total_for_level += traverse_internal(c, arena, delegate),
                NodeData::FileSize(size) => total_for_level += size,
            }
        }
    }

    delegate(total_for_level);
    total_for_level
}

fn process_command(c: Command, curr: NodeId, arena: &Tree<NodeData>) -> Option<NodeId> {
    match c {
        Command::GoToParent => curr.parent(&arena),
        Command::ChangeDir { target } => curr.children(&arena).find(|&c| {
            arena.get_data(c).map_or(false, |d| match d {
                NodeData::Dir(d) => d == &target,
                NodeData::FileSize(_) => false,
            })
        }),
        Command::List => None,
    }
}

fn process_dir<'a>(curr: NodeId, dir: &'a str, a: &mut Tree<NodeData<'a>>) {
    let n = a.new_node(NodeData::Dir(dir));
    curr.append(n, a);
}

fn process_file<'a>(curr: NodeId, file_size: usize, a: &mut Tree<NodeData<'a>>) {
    let n = a.new_node(NodeData::FileSize(file_size));
    curr.append(n, a);
}

#[derive(Debug)]
enum InputLine<'a> {
    Command(Command<'a>),
    FileSize(usize),
    Dir(&'a str),
}

fn parse_line(input: &str) -> IResult<&str, InputLine> {
    let cmd = map(parse_command, InputLine::Command);
    let file = map(parse_file, InputLine::FileSize);
    let dir = map(parse_dir, InputLine::Dir);

    alt((cmd, file, dir))(input)
}

#[derive(Debug)]
enum NodeData<'a> {
    Dir(&'a str),
    FileSize(usize),
}

fn parse_file(input: &str) -> IResult<&str, usize> {
    let (input, (size, _)) = separated_pair(complete::u32, tag(" "), take_while(|_| true))(input)?;

    Ok((input, size as usize))
}

fn parse_dir(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = take_while(|_| true)(input)?;

    Ok((input, name))
}

#[derive(Debug)]
enum Command<'a> {
    ChangeDir { target: &'a str },
    GoToParent,
    List,
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ")(input)?;

    match tag("ls")(input) {
        Ok((input, _)) => return Ok((input, Command::List)),
        Err(nom::Err::Error(_)) => {}
        Err(err) => return Err(err),
    }

    let (input, (_, target)) = tuple((tag("cd "), take_while(|_| true)))(input)?;
    match target {
        ".." => Ok((input, Command::GoToParent)),
        _ => Ok((input, Command::ChangeDir { target })),
    }
}

struct Tree<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Tree<T> {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn new_node(&mut self, data: T) -> NodeId {
        let next = self.nodes.len();

        self.nodes.push(Node {
            parent: None,
            next_sibling: None,
            prev_sibling: None,
            first_child: None,
            last_child: None,
            data,
        });

        NodeId { index: next }
    }

    fn get_data(&self, id: NodeId) -> Option<&T> {
        self.nodes.get(id.index).map(|v| &v.data)
    }
}

struct Node<T> {
    parent: Option<NodeId>,
    next_sibling: Option<NodeId>,
    prev_sibling: Option<NodeId>,
    first_child: Option<NodeId>,
    last_child: Option<NodeId>,

    data: T,
}

#[derive(Debug, Clone, Copy)]
struct NodeId {
    index: usize,
}

impl NodeId {
    fn append<T>(self, other_idx: Self, tree: &mut Tree<T>) {
        let n = &mut tree.nodes[self.index];

        let prev_sib = if let Some(prev_last) = n.last_child {
            // Hook up this node as the last child on the parent
            n.last_child = Some(other_idx);

            // Hook up this node as the next sibling on the previous last child
            let prev = &mut tree.nodes[prev_last.index];
            prev.next_sibling = Some(other_idx);

            Some(prev_last)
        } else {
            n.first_child = Some(other_idx);
            n.last_child = Some(other_idx);
            None
        };

        let other = &mut tree.nodes[other_idx.index];
        other.prev_sibling = prev_sib;
        other.next_sibling = None;
        other.parent = Some(self);
    }

    fn parent<T>(self, tree: &Tree<T>) -> Option<Self> {
        tree.nodes[self.index].parent
    }

    fn children<T>(self, tree: &Tree<T>) -> Children<'_, T> {
        Children::new(tree, self)
    }
}

struct Children<'a, T> {
    tr: &'a Tree<T>,
    curr: Option<NodeId>,
}

impl<'a, T> Children<'a, T> {
    fn new(tr: &'a Tree<T>, start: NodeId) -> Self {
        Self {
            tr,
            curr: tr.nodes[start.index].first_child,
        }
    }
}

impl<'a, T> Iterator for Children<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.curr {
            let next = self
                .tr
                .nodes
                .get(curr.index)
                .map_or(None, |n| n.next_sibling);

            self.curr = next;
            Some(curr)
        } else {
            None
        }
    }
}
