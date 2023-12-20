use std::{collections::HashMap, process::exit};

use advent_code_2023::helper::*;

type Graph<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse(file_str: &[String]) -> (Vec<bool>, Graph) {
    let mut steps = vec![];

    let steps_str = &file_str[0];
    for ch in steps_str.chars() {
        if ch == 'L' {
            steps.push(true);
        } else {
            steps.push(false);
        }
    }

    (steps, parse_map(&file_str[2..]))
}

fn parse_map(map_str: &[String]) -> Graph {
    let mut graph = Graph::new();
    for line in map_str {
        let line_str = line.trim();
        if line_str == "" {
            continue;
        }

        let k = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        if graph.contains_key(k) {
            println!("Dup Key {k}");
            exit(1);
        }

        graph.insert(k, (left, right));
    }

    graph
}

fn graph_step<'a>(place: &'a str, step: bool, graph: &'a Graph) -> &'a str {
    let (l, r) = graph.get(&place).unwrap();
    if step {
        *l
    } else {
        *r
    }
}

fn check_pv_end(pv: &Vec<&str>) -> usize {
    pv.into_iter().filter(|x| &x[2..3] == "Z").count()
}

fn graph_step_all<'a>(mut place: &'a str, steps: &[bool], graph: &'a Graph) -> &'a str {
    for step in steps {
        place = graph_step(place, *step, graph);
    }
    place
}

fn loop_start<'a>(place: &'a str, steps: &[bool], graph: &'a Graph) -> (&'a str, u64, u64) {
    let mut past_map: HashMap<&str, u64> = HashMap::new();
    let mut pp = place;
    past_map.insert(pp, 0);

    let mut i = 1;
    loop {
        let next_pp = graph_step_all(pp, &steps, &graph);
        println!("{next_pp}");
        if let Some(past_i) = past_map.get(next_pp) {
            return (next_pp, *past_i, i - *past_i);
        }
        past_map.insert(next_pp, i);
        pp = next_pp;
        i += 1;
    }
}

fn loop_find_offsets<'a>(
    mut place: &'a str,
    loop_len: u64,
    steps: &[bool],
    graph: &'a Graph,
) -> Vec<u64> {
    let mut result = vec![];
    for l in 0..loop_len {
        for i in 0..steps.len() {
            if &place[2..3] == "Z" {
                result.push(i as u64 + l * steps.len() as u64);
            }
            place = graph_step(place, steps[i], graph);
        }
    }
    result
}

// Starting with Point P, there's eventually going to be a loop.
// Then every 'Z' Place will be either at Constant C or at step A * x + B
// This can be represented by Vec<u64> and Vec<(u64, u64)>
// Basically we want to find the smallest number that is the intersection of these set.

fn main_d8p2() {
    let file_str = read_file_line_vec("input/d8p1.txt");
    let (steps, graph) = parse(&file_str);

    println!("steps len = {}", steps.len());

    let mut pv: Vec<&str> = graph
        .keys()
        .map(|k| *k)
        .collect::<Vec<&str>>()
        .into_iter()
        .filter(|k| &k[2..3] == "A")
        .collect();

    // for p in &pv {
    //     println!("p = {p}");

    //     let (loop_start_str, loop_start_idx, loop_len) = loop_start(p, &steps, &graph);
    //     let offsets = loop_find_offsets(loop_start_str, loop_len, &steps, &graph);

    //     println!(
    //         "loop_start_idx = {} loop_start_idx = {} loop_len = {}",
    //         loop_start_str,
    //         loop_start_idx * steps.len() as u64,
    //         loop_len * steps.len() as u64
    //     );
    //     println!("offsets = {:?}", offsets);
    // }

    let mut result = 0;
    while let zn = check_pv_end(&pv) {
        // if zn > 0 {
        //     println!("{zn}: {:?}", pv);
        // }
        pv = pv
            .into_iter()
            .map(|p| graph_step(p, steps[result % steps.len()], &graph))
            .collect();
        result += 1;
        // if result < 100000 {
        //     println!("{result}: pv = {:?}", pv);
        // }
        if result % 1000000 == 0 {
            println!("{result}: pv = {:?}", pv);
            println!("Progress {result}");
        }
    }
    println!("Answer {}", result);
}

fn main_d8p1() {
    let file_str = read_file_line_vec("input/d8p1.txt");
    let (steps, graph) = parse(&file_str);

    let mut p = "AAA";

    let mut result = 0;
    while p != "ZZZ" {
        p = graph_step(p, steps[result % steps.len()], &graph);
        result += 1;
    }

    println!("{}", result);
}

fn main() {
    main_d8p2();
}
