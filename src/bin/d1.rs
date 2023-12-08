use std::collections::HashMap;

use advent_code_2023::helper::*;

fn hmap_gen() -> HashMap<&'static str, u32> {
    HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ])
}

fn mainp2_helper(s: &str, hmap: &HashMap<&'static str, u32>) -> Option<u32> {
    for (k, v) in hmap {
        if s.starts_with(k) {
            return Some(*v);
        }
    }
    None
}

fn mainp2() {
    let hmap = hmap_gen();
    let mut result = 0;
    for s in read_file_line_iter("input/d1p1.txt") {
        let mut ld = 0;
        let mut fd = 0;
        let mut first = true;
        let mut ss = s.as_str();
        while ss.len() > 0 {
            if let Some(d) = mainp2_helper(ss, &hmap) {
                if first {
                    print!("{d}");
                    fd = d;
                }
                ld = d;
                first = false;
            }
            (_, ss) = { ss }.split_at(1)
        }
        println!("{ld}");

        result += fd * 10 + ld;
    }
    println!("{result}");
}

fn mainp1() {
    let mut result = 0;
    for s in read_file_line_iter("input/d1p1.txt") {
        let mut ld = 0;
        let mut fd = 0;
        let mut first = true;
        for c in s.chars() {
            if let Some(d) = c.to_digit(10) {
                if first {
                    print!("{d}");
                    fd = d;
                }
                ld = d;
                first = false;
            }
        }
        println!("{ld}");

        result += fd * 10 + ld;
    }
    println!("{result}");
}

fn main() {
    mainp2();
}
