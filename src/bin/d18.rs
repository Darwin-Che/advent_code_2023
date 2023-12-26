use std::{
    cmp::{max, min},
    collections::BTreeMap,
    mem::swap,
    process::exit,
};

use advent_code_2023::helper::*;
use itertools::Itertools;

type PosType = u8;
type Pos = (i64, i64);
type Graph = BTreeMap<(i64, i64), PosType>;

fn parse_line(line: &str) -> (u8, i64) {
    let op = line[..1].as_bytes()[0];
    let arr: Vec<&str> = line[2..].split(' ').collect();
    let v: i64 = arr[0].trim().parse().unwrap();
    (op, v)
}

fn parse_line_p2(line: &str) -> (u8, i64) {
    let s = line.split('#').collect_vec()[1];
    let v = i64::from_str_radix(&s[..5], 16).unwrap();
    let op = match &s[5..6] {
        "0" => b'R',
        "1" => b'D',
        "2" => b'L',
        "3" => b'U',
        _ => panic!(),
    };
    (op, v)
}

fn print_graph(graph: &Graph) {
    let row_min = *graph.keys().map(|(r, _)| r).min().unwrap();
    let row_max = *graph.keys().map(|(r, _)| r).max().unwrap();
    let col_min = *graph.keys().map(|(_, c)| c).min().unwrap();
    let col_max = *graph.keys().map(|(_, c)| c).max().unwrap();
    for r in row_min..=row_max {
        for c in col_min..=col_max {
            let mut ch = *graph.get(&(r, c)).unwrap_or(&b'.') as char;
            // if r == 0 && c == 0 {
            //     ch = 'O';
            // }
            print!("{ch}");
        }
        println!();
    }
    exit(0);
}

fn gen_graph(ops: &Vec<(u8, i64)>) -> Graph {
    let mut graph = Graph::new();

    let mut cur_pos: Pos = (0, 0);
    graph.insert(cur_pos, b'#');

    for (op, v) in ops {
        let pos_diff = match op {
            b'U' => (-1, 0),
            b'D' => (1, 0),
            b'L' => (0, -1),
            b'R' => (0, 1),
            _ => panic!(),
        };

        for _ in 0..*v {
            cur_pos.0 += pos_diff.0;
            cur_pos.1 += pos_diff.1;
            graph.insert(cur_pos, b'#');
        }
        graph.insert(cur_pos, b'@');
    }

    graph
}

fn gen_dim(ops: &Vec<(u8, i64)>) -> (i64, i64, i64, i64) {
    let mut row_min = 0;
    let mut row_max = 0;
    let mut col_min = 0;
    let mut col_max = 0;

    let mut cur_pos: Pos = (0, 0);

    for (op, v) in ops {
        let pos_diff = match op {
            b'U' => (-*v, 0),
            b'D' => (*v, 0),
            b'L' => (0, -*v),
            b'R' => (0, *v),
            _ => panic!(),
        };

        cur_pos.0 += pos_diff.0;
        cur_pos.1 += pos_diff.1;

        row_min = min(row_min, cur_pos.0);
        row_max = max(row_max, cur_pos.0);
        col_min = min(col_min, cur_pos.1);
        col_max = max(col_max, cur_pos.1);
    }

    (row_min, row_max, col_min, col_max)
}

type Seg = (i64, i64);
type Segs = Vec<Seg>;

fn gen_col_segs(ops: &Vec<(u8, i64)>) -> BTreeMap<i64, Segs> {
    let mut col_segs: BTreeMap<i64, Segs> = BTreeMap::new();

    let mut cur_pos: Pos = (0, 0);

    for (op, v) in ops {
        let pos_diff = match op {
            b'U' => {
                col_segs
                    .entry(cur_pos.1)
                    .or_default()
                    .push((cur_pos.0 - *v, cur_pos.0));
                (-*v, 0)
            }
            b'D' => {
                col_segs
                    .entry(cur_pos.1)
                    .or_default()
                    .push((cur_pos.0, cur_pos.0 + *v));
                (*v, 0)
            }
            b'L' => (0, -*v),
            b'R' => (0, *v),
            _ => panic!(),
        };

        cur_pos.0 += pos_diff.0;
        cur_pos.1 += pos_diff.1;
    }

    col_segs
}

fn adjust_col_segs(
    col_segs_unadjusted: BTreeMap<i64, Segs>,
    row_min: i64,
    col_min: i64,
) -> BTreeMap<i64, Segs> {
    let mut col_segs = BTreeMap::new();
    for (k, v) in col_segs_unadjusted {
        col_segs.insert(
            k - col_min,
            v.into_iter()
                .map(|(a, b)| (a - row_min, b - row_min))
                .collect_vec(),
        );
    }
    col_segs
}

fn segs_area(segs: &Segs) -> i64 {
    let mut result = 0;
    for (a, b) in segs {
        result += *b - *a + 1;
    }
    result
}

fn segs_area_union(segs1: &Segs, segs2: &Segs) -> Segs {
    let mut result: Vec<(i64, i64)> = Vec::with_capacity(segs1.len() + segs2.len());

    let mut combined = Vec::with_capacity(segs1.len() + segs2.len());
    combined.append(&mut segs1.clone());
    combined.append(&mut segs2.clone());
    combined.sort();

    for seg in combined {
        if let Some(res_last) = result.last_mut() {
            if res_last.1 >= seg.0 {
                res_last.1 = max(res_last.1, seg.1);
            } else {
                result.push(seg);
            }
        } else {
            result.push(seg);
        }
    }

    result
}

fn print_segs(segs: &Segs) {
    println!("{:?}", segs);
}

fn merge_segs(cur_segs_input: &Segs, new_segs: &Segs) -> Segs {
    let mut cur_segs = cur_segs_input.clone();
    for new_seg in new_segs {
        let mut processed = false;
        let mut next_segs = Segs::with_capacity(cur_segs.len());
        for cur_seg in cur_segs.iter() {
            if next_segs.len() > 0 && cur_seg.0 == next_segs.last().unwrap().1 {
                next_segs.last_mut().unwrap().1 = cur_seg.1;
            } else if new_seg.0 == cur_seg.0 {
                if new_seg.1 != cur_seg.1 {
                    next_segs.push((new_seg.1, cur_seg.1));
                }
                processed = true;
            } else if new_seg.1 == cur_seg.1 {
                if cur_seg.0 != new_seg.0 {
                    next_segs.push((cur_seg.0, new_seg.0));
                }
                processed = true;
            } else if new_seg.0 == cur_seg.1 {
                if cur_seg.0 != new_seg.1 {
                    next_segs.push((cur_seg.0, new_seg.1));
                }
                processed = true;
            } else if new_seg.1 == cur_seg.0 {
                if new_seg.0 != cur_seg.1 {
                    next_segs.push((new_seg.0, cur_seg.1));
                }
                processed = true;
            } else if new_seg.1 < cur_seg.1 && new_seg.0 > cur_seg.0 {
                if cur_seg.0 != new_seg.0 {
                    next_segs.push((cur_seg.0, new_seg.0));
                }
                if new_seg.1 != cur_seg.1 {
                    next_segs.push((new_seg.1, cur_seg.1));
                }
                processed = true;
            } else {
                next_segs.push(*cur_seg);
            }
        }
        if !processed {
            next_segs.push(*new_seg);
            next_segs.sort();
        }
        // println!("{:?}", next_segs);
        swap(&mut cur_segs, &mut next_segs);
    }

    cur_segs
}

fn main_d18p1<F>(parse_line_fun: F)
where
    F: Fn(&str) -> (u8, i64),
{
    let mut result = 0;

    let ops = read_file_line_iter("input/d18p1.txt")
        .map(|s| parse_line_fun(&s))
        .collect_vec();
    // let graph = gen_graph(&ops);
    // print_graph(&graph);

    let (row_min, _, col_min, _) = gen_dim(&ops);

    let col_segs = adjust_col_segs(gen_col_segs(&ops), row_min, col_min);

    let mut cur_segs = vec![];
    let mut last_c = 0;

    for (c, segs) in col_segs.iter() {
        let width = c - last_c;
        if width > 0 {
            let area = segs_area(&cur_segs);
            println!("Area from {} to {} == {}", last_c, c - 1, area);
            result += area * width;
        }
        last_c = c + 1;

        let mut next_segs = merge_segs(&mut cur_segs, &segs);
        let seg_union = segs_area_union(&cur_segs, &next_segs);
        swap(&mut cur_segs, &mut next_segs);
        // print_segs(&seg_union);
        let area = segs_area(&seg_union);
        println!("Area on {} == {}", c, area);
        result += area;
    }

    println!("result = {result}");
}

fn main() {
    main_d18p1(parse_line_p2);
}
