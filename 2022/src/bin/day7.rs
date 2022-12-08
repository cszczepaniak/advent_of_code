use indextree::{Arena, NodeId};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete,
    combinator::map,
    sequence::{separated_pair, tuple},
    Finish, IResult,
};

const DIR_MAX: usize = 100000;
const TOTAL_SPACE: usize = 70000000;
const NEEDED_SPACE: usize = 30000000;

#[allow(unused)]
static EXAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

fn main() -> anyhow::Result<()> {
    let input = common::get_input(2022, 7)?;

    let mut arena: Arena<NodeData> = Arena::new();
    let root = arena.new_node(NodeData::Dir("/"));
    let mut curr = root;

    for l in input.lines() {
        let (_, parsed) = parse_line(l)
            .finish()
            .map_err(|_| anyhow::anyhow!("error parsing command"))?;

        match parsed {
            InputLine::Command(c) => {
                let next = process_command(c, curr, &arena);
                if let Some(next) = next {
                    curr = next;
                }
            }
            InputLine::FileSize(size) => process_file(curr, size, &mut arena),
            InputLine::Dir(d) => process_dir(curr, d, &mut arena),
        }
    }

    let mut size_of_dirs = 0usize;
    let root_size = traverse(root, &arena, |s| {
        if s < DIR_MAX {
            size_of_dirs += s;
        }
    });
    println!("part 1 answer: {size_of_dirs}");

    let curr_unused_space = TOTAL_SPACE - root_size;
    println!("unused space: {curr_unused_space}");

    let need_to_free = NEEDED_SPACE - curr_unused_space;
    println!("need to free: {need_to_free}");

    let mut min_to_free = usize::MAX;
    traverse(root, &arena, |s| {
        if s >= need_to_free && s < min_to_free {
            min_to_free = s;
        }
    });
    println!("part 1 answer: {min_to_free}");

    Ok(())
}

fn traverse<F>(root: NodeId, arena: &Arena<NodeData>, mut delegate: F) -> usize
where
    F: FnMut(usize),
{
    traverse_internal(root, arena, &mut delegate)
}

fn traverse_internal<F>(root: NodeId, arena: &Arena<NodeData>, delegate: &mut F) -> usize
where
    F: FnMut(usize),
{
    let mut total_for_level = 0usize;
    for c in root.children(&arena) {
        if let Some(data) = arena.get(c) {
            let data = data.get();
            match data {
                NodeData::Dir(_) => total_for_level += traverse_internal(c, arena, delegate),
                NodeData::FileSize(size) => total_for_level += size,
            }
        }
    }

    delegate(total_for_level);
    total_for_level
}

fn process_command(c: Command, curr: NodeId, arena: &Arena<NodeData>) -> Option<NodeId> {
    match c {
        Command::GoToParent => curr.ancestors(&arena).skip(1).next(),
        Command::ChangeDir { target } => curr.children(&arena).find(|&c| {
            if let Some(data) = arena.get(c) {
                let data = data.get();
                match data {
                    NodeData::Dir(d) => return d == &target,
                    NodeData::FileSize(_) => {}
                }
            }
            false
        }),
        Command::List => None,
    }
}

fn process_dir<'a>(curr: NodeId, dir: &'a str, a: &mut Arena<NodeData<'a>>) {
    let n = a.new_node(NodeData::Dir(dir));
    curr.append(n, a);
}

fn process_file<'a>(curr: NodeId, file_size: usize, a: &mut Arena<NodeData<'a>>) {
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

struct MyTree<T> {
    nodes: Vec<MyNode<T>>,
}

impl<T> MyTree<T> {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn new_node(&mut self, data: T) -> MyNodeId {
        let next = self.nodes.len();

        self.nodes.push(MyNode {
            parent: None,
            next_sibling: None,
            prev_sibling: None,
            first_child: None,
            last_child: None,
            data,
        });

        MyNodeId { index: next }
    }

    fn get(&self, id: MyNodeId) -> Option<&MyNode<T>> {
        self.nodes.get(id.index)
    }

    fn get_data(&self, id: MyNodeId) -> Option<&T> {
        self.nodes.get(id.index).map(|v| &v.data)
    }
}

struct MyNode<T> {
    parent: Option<MyNodeId>,
    next_sibling: Option<MyNodeId>,
    prev_sibling: Option<MyNodeId>,
    first_child: Option<MyNodeId>,
    last_child: Option<MyNodeId>,

    data: T,
}

#[derive(Debug, Clone, Copy)]
struct MyNodeId {
    index: usize,
}

impl MyNodeId {
    fn append<T>(self, other_idx: Self, tree: &mut MyTree<T>) {
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

    fn parent<T>(self, tree: &MyTree<T>) -> Option<Self> {
        tree.nodes[self.index].parent
    }

    fn children<T>(self, tree: &MyTree<T>) -> Children<'_, T> {
        Children::new(tree, self)
    }
}

struct Children<'a, T> {
    tr: &'a MyTree<T>,
    curr: Option<MyNodeId>,
}

impl<'a, T> Children<'a, T> {
    fn new(tr: &'a MyTree<T>, start: MyNodeId) -> Self {
        Self {
            tr,
            curr: tr.nodes[start.index].first_child,
        }
    }
}

impl<'a, T> Iterator for Children<'a, T> {
    type Item = MyNodeId;

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
