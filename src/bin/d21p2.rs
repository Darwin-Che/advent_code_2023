use std::collections::{HashMap, HashSet};

use advent_code_2023::helper::*;

type Graph = Vec<Vec<u8>>;
type Pos = (usize, usize, i64, i64);
type PosSet = HashSet<Pos>;

fn get_start_pos(graph: &Graph) -> Pos {
    for (r, row) in graph.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == b'S' {
                return (r, c, 0, 0);
            }
        }
    }
    (0, 0, 0, 0)
}

fn get_next_pos(pos: &Pos, graph: &Graph) -> PosSet {
    let mut result = HashSet::new();
    let rows = graph.len();
    let cols = graph[0].len();
    if graph[(pos.0 + rows - 1) % rows][pos.1] != b'#' {
        if pos.0 > 0 {
            result.insert((pos.0 - 1, pos.1, pos.2, pos.3));
        } else {
            result.insert((rows - 1, pos.1, pos.2 - 1, pos.3));
        }
    }
    if graph[(pos.0 + 1) % rows][pos.1] != b'#' {
        if pos.0 < graph.len() - 1 {
            result.insert((pos.0 + 1, pos.1, pos.2, pos.3));
        } else {
            result.insert((0, pos.1, pos.2 + 1, pos.3));
        }
    }
    if graph[pos.0][(pos.1 + cols - 1) % cols] != b'#' {
        if pos.1 > 0 {
            result.insert((pos.0, pos.1 - 1, pos.2, pos.3));
        } else {
            result.insert((pos.0, cols - 1, pos.2, pos.3 - 1));
        }
    }
    if graph[pos.0][(pos.1 + 1) % cols] != b'#' {
        if pos.1 < graph[0].len() - 1 {
            result.insert((pos.0, pos.1 + 1, pos.2, pos.3));
        } else {
            result.insert((pos.0, 0, pos.2, pos.3 + 1));
        }
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

fn print_pos_set(pos_set: &PosSet, n: u64) {
    let i = n as i64;
    let mut hmap: HashMap<(i64, i64), u32> = HashMap::new();
    for pos in pos_set {
        *hmap.entry((pos.2, pos.3)).or_default() += 1;
    }
    for r in -i..=i {
        for c in -i..=i {
            print!("{:5}  ", hmap.get(&(r, c)).unwrap_or(&0));
        }
        println!();
    }
}

fn main_d21p1() {
    let graph: Graph = read_file_line_vec_vec("input/d21p1.txt");
    let start_pos = get_start_pos(&graph);
    let mut pos_set = PosSet::new();
    pos_set.insert(start_pos);
    // let n = 4 as u64;
    let n = 202300 as u64;
    // Brute force
    // for _ in 0..(65 + 131 * n) {
    //     pos_set = get_next_set(&pos_set, &graph);
    // }
    // println!("{}", pos_set.len());
    // println!("============");
    // print_pos_set(&pos_set, n);

    // Formula
    let mut result = 5817
        + 5816
        + 5789
        + 5844
        + (n - 1) * (6785 + 6776 + 6749 + 6757)
        + n * (970 + 981 + 980 + 980);
    for i in 0..n {
        let t = if (n - i) % 2 == 0 { 7717 } else { 7693 };
        if i == 0 {
            result += t;
        } else {
            result += 4 * i * t;
        }
    }
    println!("{result}");
}

fn main() {
    main_d21p1();
}

/*
~/advent_code_2023
czc % ./d21p2
34870
============
  981   5817    980
 5789   7693   5844
  970   5816    980

~/advent_code_2023
czc % ./d21p2
189238
============
    0      0    981   5817    980      0      0
    0    981   6749   7693   6785    980      0
  981   6749   7693   7717   7693   6785    980
 5789   7693   7717   7693   7717   7693   5844
  970   6757   7693   7717   7693   6776    980
    0    970   6757   7693   6776    980      0
    0      0    970   5816    980      0      0

~/advent_code_2023
czc % ./d21p2
312652
============
    0      0      0    981   5817    980      0      0      0
    0      0    981   6749   7693   6785    980      0      0
    0    981   6749   7693   7717   7693   6785    980      0
  981   6749   7693   7717   7693   7717   7693   6785    980
 5789   7693   7717   7693   7717   7693   7717   7693   5844
  970   6757   7693   7717   7693   7717   7693   6776    980
    0    970   6757   7693   7717   7693   6776    980      0
    0      0    970   6757   7693   6776    980      0      0
    0      0      0    970   5816    980      0      0      0

*/
