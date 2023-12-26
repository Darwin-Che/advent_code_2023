use core::panic;
use std::{
    cmp::{max, min},
    collections::HashMap,
    mem::swap,
};

use advent_code_2023::helper::*;
use itertools::Itertools;
use lazy_static::lazy_static;

type Item = [u32; 4];

type Items = Vec<Item>;

type Cond = (usize, bool, u32);

type Clause = (Cond, String);

type Clauses = Vec<Clause>;

type ClauseMap = HashMap<String, Clauses>;

lazy_static! {
    static ref OP_MAP: HashMap<&'static str, usize> =
        HashMap::from([("x", 0), ("m", 1), ("a", 2), ("s", 3),]);
}

fn parse() -> (ClauseMap, Items) {
    let mut file_iter = read_file_line_iter("input/d19p1.txt");

    let mut clause_map = ClauseMap::new();

    for line in file_iter.by_ref() {
        let l = line.trim();
        if l == "" {
            break;
        }
        // Parse clause
        let arr = l.split("{").collect_vec();
        let src = arr[0].to_string();
        let clauses_str = arr[1].split("}").next().unwrap();
        let mut clauses = Clauses::new();
        for clause_str in clauses_str.split(',') {
            let clause: Clause = match clause_str.split(':').collect_vec().as_slice() {
                [cond_str, dest] => {
                    let field: usize = *OP_MAP.get(&cond_str[..1]).unwrap();
                    let op = &cond_str[1..2] == ">"; // True means >
                    let v = cond_str[2..].parse().unwrap();
                    ((field, op, v), dest.to_string())
                }
                [dest] => {
                    // > 0 is always true
                    ((0, true, 0), dest.to_string())
                }
                _ => panic!(),
            };
            clauses.push(clause);
        }
        clause_map.insert(src, clauses);
    }

    let mut items = Items::new();
    for line in file_iter.by_ref() {
        let l = line.trim_matches(|c| c == '{' || c == '}');
        let mut item = [0; 4];
        for kv in l.split(',') {
            if let [k, v] = kv.split('=').collect_vec().as_slice() {
                let idx = *OP_MAP.get(k).unwrap();
                let val = v.parse().unwrap();
                item[idx] = val;
            } else {
                panic!();
            }
        }
        items.push(item);
    }

    (clause_map, items)
}

fn check_cond(item: &Item, cond: &Cond) -> bool {
    let (idx, op, val) = *cond;
    if op {
        item[idx] > val
    } else {
        item[idx] < val
    }
}

fn apply_workflow(item: &Item, clauses: &Clauses) -> String {
    for clause in clauses {
        if check_cond(item, &clause.0) {
            return clause.1.clone();
        }
    }
    panic!();
}

fn apply_rules(item: &Item, clause_map: &ClauseMap) -> bool {
    let mut k = "in".to_string();
    loop {
        let clauses = clause_map.get(&k).unwrap();
        k = apply_workflow(item, clauses);
        if k == "R" {
            return false;
        } else if k == "A" {
            return true;
        }
    }
}

fn main_d19p1() {
    let (clause_map, items) = parse();
    // println!("{:?}", clause_map);
    // println!("{:?}", items);
    let result = items
        .iter()
        .filter(|item| apply_rules(item, &clause_map))
        .map(|item| item.iter().sum::<u32>())
        .sum::<u32>();
    println!("result = {:?}", result);
}

type Itvl = [(u32, u32); 4];
type Itvls = Vec<Itvl>;

fn split_on_cond(itvl: &Itvl, cond: &Cond) -> (Option<Itvl>, Option<Itvl>) {
    let (idx, op, val) = *cond;
    if op {
        // condition is > val
        let itvl_0 = if itvl[idx].1 <= val {
            None
        } else {
            let mut tmp = itvl.clone();
            tmp[idx].0 = max(tmp[idx].0, val + 1);
            Some(tmp)
        };
        let itvl_1 = if itvl[idx].0 > val {
            None
        } else {
            let mut tmp = itvl.clone();
            tmp[idx].1 = min(tmp[idx].1, val);
            Some(tmp)
        };
        (itvl_0, itvl_1)
    } else {
        // condition is < val
        let itvl_0 = if itvl[idx].0 >= val {
            None
        } else {
            let mut tmp = itvl.clone();
            tmp[idx].1 = min(tmp[idx].1, val - 1);
            Some(tmp)
        };
        let itvl_1 = if itvl[idx].1 < val {
            None
        } else {
            let mut tmp = itvl.clone();
            tmp[idx].0 = max(tmp[idx].0, val);
            Some(tmp)
        };

        (itvl_0, itvl_1)
    }
}

fn split_on_workflow(mut itvl: Itvl, clauses: &Clauses) -> Vec<(Itvl, String)> {
    let mut result = vec![];
    for clause in clauses {
        let (itvl_true, itvl_false) = split_on_cond(&itvl, &clause.0);
        if let Some(itvl_result) = itvl_true {
            result.push((itvl_result, clause.1.clone()))
        }
        if let Some(itvl_next) = itvl_false {
            itvl = itvl_next;
        } else {
            return result;
        }
    }
    result
}

fn split_rules(clause_map: &ClauseMap) -> (Vec<Itvl>, Vec<Itvl>) {
    let mut approve = vec![];
    let mut reject = vec![];

    let mut itvls = vec![];
    itvls.push(([(1, 4000); 4], "in".to_string()));

    while itvls.len() > 0 {
        let mut next_itvls = vec![];
        for (itvl, s) in itvls.iter() {
            for (next_itvl, next_s) in split_on_workflow(*itvl, clause_map.get(s).unwrap()) {
                if next_s == "A" {
                    approve.push(next_itvl);
                } else if next_s == "R" {
                    reject.push(next_itvl);
                } else {
                    next_itvls.push((next_itvl, next_s));
                }
            }
        }
        swap(&mut itvls, &mut next_itvls);
    }

    (approve, reject)
}

fn main_d19p2() {
    let (clause_map, items) = parse();
    let (approve, reject) = split_rules(&clause_map);
    println!("{:?}", approve);
    let approve_cnt: u64 = approve
        .into_iter()
        .map(|arr| {
            let mut res = 1;
            for (a, b) in arr {
                res *= (b - a + 1) as u64;
            }
            res
        })
        .sum();
    println!("{:?}", approve_cnt);
    // println!("result = {}");
}

fn main() {
    main_d19p2();
}
