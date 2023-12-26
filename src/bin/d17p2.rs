use std::collections::BTreeMap;

use advent_code_2023::helper::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Dir {
    NIL,
    L,
    R,
    U,
    D,
}

type Graph = Vec<Vec<u32>>;
type Node = (usize, usize, Dir, usize); // (r, c, dir, dir_cnt)

fn dir_opposite(dir1: Dir, dir2: Dir) -> bool {
    dir1 == Dir::L && dir2 == Dir::R
        || dir1 == Dir::R && dir2 == Dir::L
        || dir1 == Dir::U && dir2 == Dir::D
        || dir1 == Dir::D && dir2 == Dir::U
}

fn next_node(node: Node, graph: &Graph, dir: Dir) -> Option<Node> {
    if dir_opposite(node.2, dir) {
        return None;
    }

    if node.3 < 4 {
        if node.2 == Dir::NIL || dir == node.2 {
            next_node_helper(node, graph, dir)
        } else {
            None
        }
    } else if node.3 == 10 {
        if dir != node.2 {
            next_node_helper(node, graph, dir)
        } else {
            None
        }
    } else {
        next_node_helper(node, graph, dir)
    }
}

fn next_node_helper(node: Node, graph: &Graph, dir: Dir) -> Option<Node> {
    let dir_cnt = if node.2 == dir { node.3 + 1 } else { 1 };
    if dir == Dir::U && node.0 > 0 {
        Some((node.0 - 1, node.1, dir, dir_cnt))
    } else if dir == Dir::D && node.0 < graph.len() - 1 {
        Some((node.0 + 1, node.1, dir, dir_cnt))
    } else if dir == Dir::L && node.1 > 0 {
        Some((node.0, node.1 - 1, dir, dir_cnt))
    } else if dir == Dir::R && node.1 < graph[1].len() - 1 {
        Some((node.0, node.1 + 1, dir, dir_cnt))
    } else {
        None
    }
}

fn next_nodes(node: Node, graph: &Graph) -> BTreeMap<Node, u32> {
    let mut result = BTreeMap::new();
    let dirs = [Dir::U, Dir::D, Dir::L, Dir::R];
    for dir in dirs {
        if let Some(n) = next_node(node, graph, dir) {
            result.insert(n, graph[n.0][n.1]);
        }
    }
    println!("next_nodes {:?}     {:?}", node, result);
    result
}

fn main_d17p1() {
    let file_iter = read_file_line_iter("input/d17p1.txt");
    let graph: Graph = file_iter
        .map(|s| {
            s.chars()
                .into_iter()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    let mut len_arr = vec![];
    let result = dijkstra((0, 0, Dir::NIL, 0), |node| next_nodes(node, &graph));
    for (node, e) in result {
        if node.0 == graph.len() - 1 && node.1 == graph[0].len() - 1 && node.3 >= 4 {
            len_arr.push(e.unwrap().1);
            println!("{:?}", e);
        }
    }
    println!("{:?}", len_arr.iter().min().unwrap());
}

fn main() {
    main_d17p1();
}
