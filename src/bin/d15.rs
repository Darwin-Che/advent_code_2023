use std::collections::HashMap;

use advent_code_2023::helper::*;

type Box = Vec<(String, u64, bool)>;

fn calc_hash(word: &str) -> u64 {
    let mut hash = 0;
    for ch in word.as_bytes() {
        hash += *ch as u64;
        hash *= 17;
        hash %= 256;
    }
    hash
}

fn main_d15p1() {
    let line = read_file_line_vec("input/d15p1.txt");
    let words: Vec<&str> = line[0].split(',').collect();
    let mut result: u64 = 0;
    for word in &words {
        result += calc_hash(word);
    }
    println!("{} {}", words.len(), result);
}

fn main_d15p2() {
    let line = read_file_line_vec("input/d15p1.txt");
    let words: Vec<&str> = line[0].split(',').collect();
    let mut boxes: Vec<Box> = vec![Vec::new(); 256];
    'outer_loop: for word in &words {
        let n = word.len();
        if &word[n - 1..] == "-" {
            let hash = calc_hash(&word[..n - 1]) as usize;
            let b = &mut boxes[hash];
            for x in b {
                if x.0 == word[..n - 1] {
                    x.2 = false;
                }
            }
        } else {
            let hash = calc_hash(&word[..n - 2]) as usize;
            let b = &mut boxes[hash];
            for x in b.iter_mut() {
                if x.2 && x.0 == word[..n - 2] {
                    x.1 = word[n - 1..].parse().unwrap();
                    continue 'outer_loop;
                }
            }
            b.push((
                word[..n - 2].to_owned(),
                word[n - 1..].parse().unwrap(),
                true,
            ));
        }
    }

    let mut result = 0;
    for (idx, b) in boxes.iter().enumerate() {
        let mut cnt = 1;
        for t in b {
            if t.2 {
                println!("{:?} {:?}", idx, t);
                result += (idx as u64 + 1) * cnt * t.1;
                cnt += 1;
            }
        }
    }
    println!("{} {}", words.len(), result);
}

fn main() {
    main_d15p2();
}
