use std::collections::{HashMap, HashSet, VecDeque};

use advent_code_2023::helper::*;

type Node = (usize, usize);
type Graph = Vec<Vec<u8>>;
type Path = HashMap<Node, usize>;

fn get_start_pos(graph: &Graph) -> Node {
    for (c, ch) in graph[0].iter().enumerate() {
        if *ch == b'.' {
            return (0, c);
        }
    }
    panic!();
}

fn get_end_pos(graph: &Graph) -> Node {
    for (c, ch) in graph[graph.len() - 1].iter().enumerate() {
        if *ch == b'.' {
            return (graph.len() - 1, c);
        }
    }
    panic!()
}

fn get_next_set(graph: &Graph, node: &Node) -> HashSet<Node> {
    let ch = graph[node.0][node.1];
    let mut result = HashSet::new();

    if b".^".contains(&ch) && node.0 > 0 && graph[node.0 - 1][node.1] != b'#' {
        result.insert((node.0 - 1, node.1));
    }
    if b".v".contains(&ch) && node.0 < graph.len() - 1 && graph[node.0 + 1][node.1] != b'#' {
        result.insert((node.0 + 1, node.1));
    }
    if b".<".contains(&ch) && node.1 > 0 && graph[node.0][node.1 - 1] != b'#' {
        result.insert((node.0, node.1 - 1));
    }
    if b".>".contains(&ch) && node.1 < graph[0].len() - 1 && graph[node.0][node.1 + 1] != b'#' {
        result.insert((node.0, node.1 + 1));
    }

    result
}

fn print_path(path: &Path) {
    let mut path_vec: Vec<_> = path.iter().collect();
    path_vec.sort_by(|a, b| a.1.cmp(b.1));
    for (node, _) in path_vec {
        print!("{:?}, ", node);
    }
    println!();
}

fn get_longest_path(graph: &Graph, start_pos: Node, end_pos: Node) -> Path {
    let mut candidates = vec![];
    let mut q = VecDeque::new();
    q.push_front(HashMap::from([(start_pos, 0)]));
    while let Some(p) = q.pop_back() {
        // print_path(&p);
        let (last_node, _) = p.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
        for next_node in get_next_set(graph, last_node) {
            if !p.contains_key(&next_node) {
                let mut pp = p.clone();
                pp.insert(next_node, p.len());
                if next_node == end_pos {
                    println!("Pushing to Candidates len = {} path = {:?}", pp.len(), pp);
                    candidates.push(pp);
                } else {
                    q.push_front(pp);
                }
            }
        }
    }
    // Longest candidates
    candidates
        .into_iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
}

fn main_d23p1() {
    let graph = read_file_line_vec_vec("input/d23p1.txt");
    let start_pos = get_start_pos(&graph);
    let end_pos = get_end_pos(&graph);
    let longest_path = get_longest_path(&graph, start_pos, end_pos);
    println!("longest_path_len = {}", longest_path.len());
    print_path(&longest_path);
}

fn main_d23p2() {
    let mut graph = read_file_line_vec_vec("input/d23p1.txt");
    // Change ^v<> to .
    for v in &mut graph {
        for ch in v {
            if *ch != b'#' {
                *ch = b'.';
            }
        }
    }
    let start_pos = get_start_pos(&graph);
    let end_pos = get_end_pos(&graph);
    let longest_path = get_longest_path(&graph, start_pos, end_pos);
    println!("longest_path_len = {}", longest_path.len());
    print_path(&longest_path);
}

fn main() {
    main_d23p2();
}
