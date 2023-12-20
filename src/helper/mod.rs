use num::{CheckedMul, Integer};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{prelude::*, BufReader};

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
