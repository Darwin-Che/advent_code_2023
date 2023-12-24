use std::collections::{HashMap, HashSet, VecDeque};

use advent_code_2023::helper::*;

type Node = (usize, usize);
type Graph = Vec<Vec<u8>>;
type Path = HashMap<Node, (usize, usize)>;

type ReducedGraph = HashMap<Node, HashMap<Node, usize>>;

fn find_reduced_edge(graph: &Graph, src: &Node, reduced_graph: &mut ReducedGraph) {
    let mut result = HashMap::new();
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_front((*src, 0usize));
    visited.insert(*src);
    while let Some((d, l)) = q.pop_back() {
        for nd in get_next_set(graph, &d) {
            if nd == *src {
                continue;
            } else if reduced_graph.contains_key(&nd) {
                result.insert(nd, l + 1);
            } else {
                if !visited.contains(&nd) {
                    visited.insert(nd);
                    q.push_front((nd, l + 1));
                }
            }
        }
    }
    let edges = reduced_graph.get_mut(src).unwrap();
    for (n, l) in result {
        edges.insert(n, l);
    }
}

fn reduce_graph(graph: &Graph, start_pos: Node, end_pos: Node) -> ReducedGraph {
    let mut reduced_graph = ReducedGraph::new();

    reduced_graph.insert(start_pos, HashMap::new());
    reduced_graph.insert(end_pos, HashMap::new());

    // First, identify all Split Node
    for r in 1..graph.len() - 1 {
        for c in 0..graph[0].len() {
            if get_next_set(graph, &(r, c)).len() > 2 {
                reduced_graph.insert((r, c), HashMap::new());
            }
        }
    }

    // Next, connect the Split Nodes
    for d in reduced_graph.clone().keys() {
        find_reduced_edge(graph, &d, &mut reduced_graph);
    }

    // Print
    for (nd, v) in &reduced_graph {
        println!("{:?} => {:?}", nd, v);
    }

    reduced_graph
}

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
    path_vec.sort_by(|a, b| a.1 .0.cmp(&b.1 .0));
    for (node, _) in path_vec {
        print!("{:?}, ", node);
    }
    println!();
}

fn path_length(path: &Path) -> usize {
    path.iter().map(|x| x.1 .1).sum()
}

fn get_longest_path(graph: &ReducedGraph, start_pos: Node, end_pos: Node) -> Path {
    let mut candidates = vec![];
    let mut q = VecDeque::new();
    q.push_front(HashMap::from([(start_pos, (0, 0))]));
    while let Some(p) = q.pop_back() {
        // print_path(&p);
        let (last_node, _) = p.iter().max_by(|a, b| a.1 .0.cmp(&b.1 .0)).unwrap();
        for (next_node, l) in graph.get(last_node).unwrap() {
            if !p.contains_key(&next_node) {
                let mut pp = p.clone();
                pp.insert(*next_node, (p.len(), *l));
                if *next_node == end_pos {
                    println!(
                        "Pushing to Candidates len = {}, {}",
                        pp.len(),
                        path_length(&pp)
                    );
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
        .max_by(|a, b| path_length(a).cmp(&path_length(b)))
        .unwrap()
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

    let reduced_graph = reduce_graph(&graph, start_pos, end_pos);

    let longest_path = get_longest_path(&reduced_graph, start_pos, end_pos);
    println!("longest_path_len = {}", path_length(&longest_path));
    print_path(&longest_path);
}

fn main() {
    main_d23p2();
}
