use std::collections::HashSet;

use advent_code_2023::helper::*;

type Graph = Vec<Vec<u8>>;
type Pos = (usize, usize);
type PosSet = HashSet<Pos>;

fn get_start_pos(graph: &Graph) -> Pos {
    for (r, row) in graph.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == b'S' {
                return (r, c);
            }
        }
    }
    (0, 0)
}

fn get_next_pos(pos: &Pos, graph: &Graph) -> PosSet {
    let mut result = HashSet::new();
    if pos.0 > 0 && graph[pos.0 - 1][pos.1] != b'#' {
        result.insert((pos.0 - 1, pos.1));
    }
    if pos.0 < graph.len() - 1 && graph[pos.0 + 1][pos.1] != b'#' {
        result.insert((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 && graph[pos.0][pos.1 - 1] != b'#' {
        result.insert((pos.0, pos.1 - 1));
    }
    if pos.1 < graph[0].len() - 1 && graph[pos.0][pos.1 + 1] != b'#' {
        result.insert((pos.0, pos.1 + 1));
    }
    result
}

fn get_next_set(pos_set: &PosSet, graph: &Graph) -> PosSet {
    let mut next_set = HashSet::new();
    for pos in pos_set {
        for next_pos in get_next_pos(pos, graph) {
            next_set.insert(next_pos);
        }
    }
    next_set
}

fn main_d21p1() {
    let graph: Graph = read_file_line_vec_vec("input/d21p1.txt");
    let start_pos = get_start_pos(&graph);
    let mut pos_set = PosSet::new();
    pos_set.insert(start_pos);
    for _ in 0..64 {
        pos_set = get_next_set(&pos_set, &graph);
    }
    println!("{}", pos_set.len());
}

fn main() {
    main_d21p1();
}
