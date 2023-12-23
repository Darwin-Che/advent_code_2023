use std::{
    cmp,
    collections::{HashMap, HashSet},
    fmt,
};

use advent_code_2023::helper::*;

type Graph = Vec<Vec<u8>>;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    L,
    R,
    U,
    D,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self {
            Direction::L => 'L',
            Direction::R => 'R',
            Direction::U => 'U',
            Direction::D => 'D',
        };
        write!(f, "{}", ch)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    r: usize,
    c: usize,
    dir: Direction,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node ({}, {}, {})", self.r, self.c, self.dir)
    }
}

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SOLVE_DIR: HashMap<(Direction, u8), Vec<Direction>> = {
        let mut hmap = HashMap::new();
        hmap.insert((Direction::L, b'.'), vec![Direction::L]);
        hmap.insert((Direction::L, b'-'), vec![Direction::L]);
        hmap.insert((Direction::L, b'|'), vec![Direction::U, Direction::D]);
        hmap.insert((Direction::L, b'\\'), vec![Direction::U]);
        hmap.insert((Direction::L, b'/'), vec![Direction::D]);
        hmap.insert((Direction::R, b'.'), vec![Direction::R]);
        hmap.insert((Direction::R, b'-'), vec![Direction::R]);
        hmap.insert((Direction::R, b'|'), vec![Direction::U, Direction::D]);
        hmap.insert((Direction::R, b'\\'), vec![Direction::D]);
        hmap.insert((Direction::R, b'/'), vec![Direction::U]);

        hmap.insert((Direction::U, b'.'), vec![Direction::U]);
        hmap.insert((Direction::U, b'|'), vec![Direction::U]);
        hmap.insert((Direction::U, b'-'), vec![Direction::L, Direction::R]);
        hmap.insert((Direction::U, b'\\'), vec![Direction::L]);
        hmap.insert((Direction::U, b'/'), vec![Direction::R]);
        hmap.insert((Direction::D, b'.'), vec![Direction::D]);
        hmap.insert((Direction::D, b'|'), vec![Direction::D]);
        hmap.insert((Direction::D, b'-'), vec![Direction::L, Direction::R]);
        hmap.insert((Direction::D, b'\\'), vec![Direction::R]);
        hmap.insert((Direction::D, b'/'), vec![Direction::L]);
        hmap
    };
}

fn move_node(r: usize, c: usize, dir: Direction, graph: &Graph) -> Option<Node> {
    match dir {
        Direction::L => {
            if c > 0 {
                Some(Node { r, c: c - 1, dir })
            } else {
                None
            }
        }
        Direction::R => {
            if c < graph[0].len() - 1 {
                Some(Node { r, c: c + 1, dir })
            } else {
                None
            }
        }
        Direction::U => {
            if r > 0 {
                Some(Node { r: r - 1, c, dir })
            } else {
                None
            }
        }
        Direction::D => {
            if r < graph.len() - 1 {
                Some(Node { r: r + 1, c, dir })
            } else {
                None
            }
        }
    }
}

fn next_nodes(node: Node, graph: &Graph) -> Vec<Node> {
    let dir_vec = SOLVE_DIR.get(&(node.dir, graph[node.r][node.c])).unwrap();
    dir_vec
        .iter()
        .filter_map(|dir| move_node(node.r, node.c, *dir, graph))
        .collect()
}

fn dfs(start_node: Node, graph: &Graph) -> HashSet<Node> {
    let mut hset = HashSet::new();
    let mut st = vec![];
    hset.insert(start_node);
    st.push(start_node);
    println!("{}", start_node);
    while let Some(n) = st.pop() {
        for nn in next_nodes(n, graph) {
            if !hset.contains(&nn) {
                // println!("{}", nn);
                hset.insert(nn);
                st.push(nn);
            }
        }
    }
    hset
}

fn print_pos_set(pos_set: &HashSet<(usize, usize)>, graph: &Graph) {
    let mut p = vec![vec!['.'; graph[0].len()]; graph.len()];
    for (r, c) in pos_set {
        p[*r][*c] = '#';
    }
    for v in p {
        for c in v {
            print!("{c}");
        }
        println!();
    }
}

fn main_d16p1() {
    let graph = read_file_line_vec_vec("input/d16p1.txt");
    let start_node = Node {
        r: 0,
        c: 0,
        dir: Direction::R,
    };
    let node_set = dfs(start_node, &graph);
    let mut pos_set = HashSet::new();
    for n in node_set {
        pos_set.insert((n.r, n.c));
    }
    // print_pos_set(&pos_set, &graph);
    println!("{}", pos_set.len());
}

fn p2_helper(start_node: Node, graph: &Graph) -> usize {
    let node_set = dfs(start_node, &graph);
    let mut pos_set = HashSet::new();
    for n in node_set {
        pos_set.insert((n.r, n.c));
    }
    // print_pos_set(&pos_set, &graph);
    // println!("{}", pos_set.len());
    pos_set.len()
}

fn main_d16p2() {
    let graph = read_file_line_vec_vec("input/d16p1.txt");
    let row = graph.len();
    let col = graph[0].len();

    let mut start_node_vec = vec![];

    // From left edge
    for r in 0..row {
        start_node_vec.push(Node {
            r,
            c: 0,
            dir: Direction::R,
        });
    }

    // From right edge
    for r in 0..row {
        start_node_vec.push(Node {
            r,
            c: col - 1,
            dir: Direction::L,
        });
    }

    // From top edge
    for c in 0..col {
        start_node_vec.push(Node {
            r: 0,
            c,
            dir: Direction::D,
        });
    }

    // From bottom edge
    for c in 0..col {
        start_node_vec.push(Node {
            r: row - 1,
            c,
            dir: Direction::U,
        });
    }

    let result = start_node_vec
        .into_iter()
        .map(|n| p2_helper(n, &graph))
        .max()
        .unwrap();

    println!("{result}");
}

fn main() {
    main_d16p2();
}
