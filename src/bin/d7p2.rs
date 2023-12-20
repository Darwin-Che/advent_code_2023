use std::{cmp::Ordering, collections::HashMap};

use advent_code_2023::helper::*;

fn calc_class(s: &[u8]) -> u32 {
    let mut m: HashMap<&u8, u32> = HashMap::new();
    let mut j_cnt = 0;
    for x in s {
        if *x == b'J' {
            j_cnt += 1;
        } else {
            *m.entry(x).or_insert(0) += 1;
        }
    }

    if j_cnt == 5 {
        return 7;
    }

    let mut vals: Vec<u32> = m.values().map(|x| *x).collect();
    vals.sort();
    vals.reverse();
    vals[0] += j_cnt;

    match vals.as_slice() {
        [5] => 7,
        [4, 1] => 6,
        [3, 2] => 5,
        [3, 1, 1] => 4,
        [2, 2, 1] => 3,
        [2, 1, 1, 1] => 2,
        _ => 1,
    }
}

fn calc_code(x: u8) -> u8 {
    match x {
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'T' => 10,
        b'J' => 1,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => panic!(),
    }
}

fn cmp_helper(s1: &[u8], s2: &[u8]) -> Ordering {
    let n = s1.len();
    for i in 0..n {
        let c1 = calc_code(s1[i]);
        let c2 = calc_code(s2[i]);
        if c1 < c2 {
            return Ordering::Less;
        } else if c1 > c2 {
            return Ordering::Greater;
        }
    }
    return Ordering::Equal;
}

fn cmp(s1: &str, s2: &str) -> Ordering {
    let c1 = calc_class(s1.as_bytes());
    let c2 = calc_class(s2.as_bytes());
    if c1 < c2 {
        Ordering::Less
    } else if c1 > c2 {
        Ordering::Greater
    } else {
        cmp_helper(s1.as_bytes(), s2.as_bytes())
    }
}

fn main_d7p1() {
    let line_arr: Vec<String> = read_file_line_vec("input/d7p1.txt");
    let mut pair_arr: Vec<(&str, u32)> = line_arr
        .iter()
        .map(|x| {
            let v: Vec<&str> = x.split(" ").collect();
            (v[0], v[1].parse::<u32>().unwrap())
        })
        .collect();

    pair_arr.sort_by(|a, b| cmp(a.0, b.0));

    let mut result: u64 = 0;
    let n = pair_arr.len();
    for i in 0..n {
        result += (i as u64 + 1) * pair_arr[i].1 as u64;
    }

    println!("{:?}", &pair_arr);

    println!("{result}");
}

fn main() {
    main_d7p1();
}
