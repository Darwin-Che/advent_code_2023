use advent_code_2023::helper::*;
use itertools::Itertools;

type Graph = Vec<Vec<u8>>;

const PENALTY: usize = 1;

fn parse() -> Vec<Graph> {
    read_file_line_iter("input/d13p1.txt")
        .group_by(|s| s.trim().len())
        .into_iter()
        .filter(|(k, _)| *k != 0)
        .map(|(k, v)| v.map(|s| s.as_bytes().to_vec()).collect())
        .collect()
}

fn detect_vertical(graph: &Graph) -> Option<usize> {
    'outer: for v in 1..graph[0].len() {
        let mut p = 0;
        // check if v is a vertial line
        for c in v..graph[0].len() {
            if 2 * v <= c {
                continue;
            }
            for r in 0..graph.len() {
                if graph[r][c] != graph[r][2 * v - 1 - c] {
                    p += 1;
                    if p > PENALTY {
                        continue 'outer;
                    }
                }
            }
        }
        if p == PENALTY {
            return Some(v);
        }
    }
    None
}

fn detect_horizontal(graph: &Graph) -> Option<usize> {
    'outer: for h in 1..graph.len() {
        let mut p = 0;
        // check if h is a horizontal line
        for r in h..graph.len() {
            if 2 * h <= r {
                continue;
            }
            for c in 0..graph[0].len() {
                if graph[r][c] != graph[2 * h - 1 - r][c] {
                    p += 1;
                    if p > PENALTY {
                        continue 'outer;
                    }
                }
            }
        }
        if p == PENALTY {
            return Some(h);
        }
    }
    None
}

fn main_d13p1() {
    let graphs = parse();
    let mut result = 0;
    for graph in graphs {
        if let Some(h) = detect_horizontal(&graph) {
            println!("Detect h={h}");
            result += h * 100;
        }
        if let Some(v) = detect_vertical(&graph) {
            println!("Detect v={v}");
            result += v;
        }
    }
    println!("{result}");
}

fn main() {
    main_d13p1();
}
