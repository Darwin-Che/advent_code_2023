use std::{cmp, collections::BTreeMap};

use advent_code_2023::helper::*;

type SegMap = BTreeMap<u64, (u64, u64)>;

// BTreeMap<source, (dest, len)>

fn parser() -> (Vec<u64>, Vec<SegMap>) {
    let mut file_iter = read_file_line_iter("input/d5p1.txt");
    let seeds_line = file_iter.next().unwrap();
    let seeds = seeds_line
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut maps = Vec::new();
    while let Some(line) = file_iter.next() {
        if line.trim() == "" {
            continue;
        }
        if line.contains("map") {
            maps.push(parser_map(&mut file_iter));
        }
    }

    (seeds, maps)
}

fn parser_map(file_iter: &mut impl Iterator<Item = String>) -> SegMap {
    let mut new_map = BTreeMap::new();

    while let Some(line) = file_iter.next() {
        if line.trim() == "" {
            return new_map;
        }
        let arr: Vec<u64> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        new_map.insert(arr[1], (arr[0], arr[2]));
    }

    new_map
}

fn resolve_one(mut seed: u64, maps: &Vec<SegMap>) -> u64 {
    // println!("Start {seed}");
    for map in maps {
        if let Some((s, (d, l))) = map.range(..=seed).next_back() {
            if seed < s + l {
                seed = d + (seed - s);
            }
        }
        // println!("Step {seed}");
    }
    // println!("");
    seed
}

fn resolve_segs_one_step(segs: Vec<(u64, u64)>, map: &SegMap) -> Vec<(u64, u64)> {
    let mut results = Vec::new();
    for (mut start, mut len) in segs {
        for (s, (d, l)) in map.range(..=start + len) {
            // if it overlaps with (start, start+len)
            if *s > start {
                let sz = cmp::min(*s - start, len);
                results.push((start, sz));
                start += sz;
                len -= sz;
            }
            if *s <= start && *s + *l > start {
                let sz = cmp::min(*s + *l - start, len);
                let sd = *d + start - *s;
                results.push((sd, sz));
                start += sz;
                len -= sz;
            }
        }
        if len != 0 {
            results.push((start, len));
        }
    }
    results
}

fn resolve_segs(mut segs: Vec<(u64, u64)>, maps: &Vec<SegMap>) -> Vec<(u64, u64)> {
    println!("{:?}", segs);
    for map in maps {
        segs = resolve_segs_one_step(segs, map);
        println!("{:?}", segs);
    }
    return segs;
}

fn main_d5p1() {
    let (seeds, maps) = parser();

    let mut lowest = u64::MAX;

    for seed in seeds {
        lowest = cmp::min(lowest, resolve_one(seed, &maps));
    }

    println!("Lowest {lowest}");
}

fn main_d5p2() {
    let (seeds, maps) = parser();

    let seeds_n = seeds.len() / 2;
    let mut initial_segs = vec![];
    for i in 0..seeds_n {
        initial_segs.push((seeds[2 * i], seeds[2 * i + 1]));
    }

    let mut segs = resolve_segs(initial_segs, &maps);
    segs.sort();

    println!("{:?}", segs);
}

fn main() {
    main_d5p2();
}
