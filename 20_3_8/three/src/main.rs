use std::fmt::Debug;
use std::io;
use std::str::FromStr;

struct Edge<'a> {
    node: Option<&Node>,
    next: Option<&Edge<'a>>,
}

struct Node {
    value: i64,
    edges: Option<&Edge>,
}

fn read_and_divide_line<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut target_line = String::new();
    io::stdin().read_line(&mut target_line).unwrap();
    target_line
        .trim()
        .split_whitespace()
        .map(|c| T::from_str(c).unwrap())
        .collect()
}

fn main() {
    let tree_vertex_num: i64 = read_and_divide_line()[0];

    let first_line_nodes = read_and_divide_line();
    let root = Node {
        value: first_line_nodes[0],
        edges: None,
    };
    let next_node = Node {
        value: first_line_nodes[1],

    }

    for _ in 1..tree_vertex_num {}
}
