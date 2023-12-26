use num::{CheckedMul, Integer};
use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Add;

pub fn read_file_line_iter(file: &str) -> impl Iterator<Item = String> {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);
    reader.lines().into_iter().map(|x| x.unwrap()).into_iter()
}

pub fn read_file_line_vec(file: &str) -> Vec<String> {
    read_file_line_iter(file).collect()
}

pub fn read_file_line_vec_vec(file: &str) -> Vec<Vec<u8>> {
    read_file_line_iter(file)
        .map(|x| x.as_bytes().to_vec())
        .collect()
}

pub fn math_sqrt<T: Integer + CheckedMul + std::cmp::Ord + Copy>(v: T) -> T {
    let mut x = v;
    while x.checked_mul(&x).unwrap().cmp(&v) == Ordering::Greater {
        x = (x + v.div_floor(&x)) / (T::one() + T::one());
    }
    return x;
}

pub fn dijkstra<V: Ord + Copy, E: Ord + Copy + Add<Output = E>, F>(
    start: V,
    fn_edges: F,
) -> BTreeMap<V, Option<(V, E)>>
where
    F: Fn(V) -> BTreeMap<V, E>,
{
    let mut ans = BTreeMap::new();
    let mut prio = BinaryHeap::new();

    // start is the special case that doesn't have a predecessor
    ans.insert(start, None);

    for (new, weight) in fn_edges(start) {
        ans.insert(new, Some((start, weight)));
        prio.push(Reverse((weight, new, start)));
    }

    while let Some(Reverse((dist_new, new, prev))) = prio.pop() {
        match ans[&new] {
            // what we popped is what is in ans, we'll compute it
            Some((p, d)) if p == prev && d == dist_new => {}
            // otherwise it's not interesting
            _ => continue,
        }

        for (next, weight) in fn_edges(new) {
            match ans.get(&next) {
                // if ans[next] is a lower dist than the alternative one, we do nothing
                Some(Some((_, dist_next))) if dist_new + weight >= *dist_next => {}
                // if ans[next] is None then next is start and so the distance won't be changed, it won't be added again in prio
                Some(None) => {}
                // the new path is shorter, either new was not in ans or it was farther
                _ => {
                    ans.insert(next, Some((new, weight + dist_new)));
                    prio.push(Reverse((weight + dist_new, next, new)));
                }
            }
        }
    }

    ans
}
