use std::{collections::HashMap, process::exit};

use advent_code_2023::helper::*;

type Graph = Vec<Vec<u8>>;

fn main_d14p1() {
    let graph = read_file_line_vec_vec("input/d14p1.txt");
    let rows = graph.len();
    let cols = graph[0].len();
    let mut result = 0;
    for c in 0..cols {
        let mut idx = 0;
        for r in 0..rows {
            if graph[r][c] == b'#' {
                idx = r + 1;
            }
            if graph[r][c] == b'O' {
                // println!("{}", rows - idx);
                result += rows - idx;
                idx += 1;
            }
        }
    }

    println!("{result}");
}

fn north_load(graph: &Graph) -> u64 {
    let mut result = 0;
    let rows = graph.len();
    let cols = graph[0].len();
    for r in 0..rows {
        for c in 0..cols {
            if graph[r][c] == b'O' {
                result += (rows - r) as u64;
            }
        }
    }
    result
}

fn tilt_north(graph: &mut Graph) {
    let rows = graph.len();
    let cols = graph[0].len();
    for c in 0..cols {
        let mut idx = 0;
        for r in 0..rows {
            if graph[r][c] == b'#' {
                idx = r + 1;
            }
            if graph[r][c] == b'O' {
                graph[r][c] = b'.';
                graph[idx][c] = b'O';
                idx += 1;
            }
        }
    }
}

fn tilt_south(graph: &mut Graph) {
    let rows = graph.len();
    let cols = graph[0].len();
    for c in 0..cols {
        let mut idx = rows - 1;
        for r in (0..rows).rev() {
            if graph[r][c] == b'#' {
                idx = r - 1;
            }
            if graph[r][c] == b'O' {
                graph[r][c] = b'.';
                graph[idx][c] = b'O';
                idx -= 1;
            }
        }
    }
}

fn tilt_west(graph: &mut Graph) {
    let rows = graph.len();
    let cols = graph[0].len();
    for r in 0..rows {
        let mut idx = 0;
        for c in 0..cols {
            if graph[r][c] == b'#' {
                idx = c + 1;
            }
            if graph[r][c] == b'O' {
                graph[r][c] = b'.';
                graph[r][idx] = b'O';
                idx += 1;
            }
        }
    }
}

fn tilt_east(graph: &mut Graph) {
    let rows = graph.len();
    let cols = graph[0].len();
    for r in 0..rows {
        let mut idx = cols - 1;
        for c in (0..cols).rev() {
            if graph[r][c] == b'#' {
                idx = c - 1;
            }
            if graph[r][c] == b'O' {
                graph[r][c] = b'.';
                graph[r][idx] = b'O';
                idx -= 1;
            }
        }
    }
}

fn rotate(graph: &mut Graph) {
    tilt_north(graph);
    tilt_west(graph);
    tilt_south(graph);
    tilt_east(graph);
}

fn print_graph(graph: &Graph) {
    for v in graph {
        for u in v {
            print!("{}", *u as char);
        }
        println!();
    }
}

fn main_d14p2() {
    let mut graph = read_file_line_vec_vec("input/d14p1.txt");
    let mut hmap = HashMap::new();
    hmap.insert(graph.clone(), 0);
    println!("{:?} = {:?}", 0, north_load(&graph));
    let mut i = 1;
    while i <= 1000000000u64 {
        rotate(&mut graph);
        // print_graph(&graph);
        println!("loop={:?} north_load={:?}", i, north_load(&graph));
        if let Some(j) = hmap.get(&graph) {
            let interval = i - j;
            println!("interval = {interval} = {i} - {j}");
            let rem = (1000000000u64 - i) % interval;
            i = 1000000000u64 - rem;
        }
        hmap.insert(graph.clone(), i);
        i += 1;
    }

    println!("result = {:?}", north_load(&graph));
}

fn main() {
    main_d14p2();
}
