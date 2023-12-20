use std::mem::swap;

use advent_code_2023::helper::*;

type GMap = Vec<Vec<u8>>;
type Pos = (usize, usize);

// const EXPAND_RATE: usize = 2;
const EXPAND_RATE: usize = 1000000;

fn find_galaxy(map: &GMap) -> Vec<Pos> {
    let mut result = vec![];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == b'#' {
                result.push((i, j));
            }
        }
    }
    result
}

fn find_empty_row(map: &GMap) -> Vec<usize> {
    let mut result = vec![];
    for i in 0..map.len() {
        if map[i].iter().all(|x| *x == b'.') {
            result.push(i);
        }
    }
    result
}

fn find_empty_col(map: &GMap) -> Vec<usize> {
    let mut result = vec![];
    for j in 0..map[0].len() {
        if (0..map.len()).all(|i| map[i][j] == b'.') {
            result.push(j);
        }
    }
    result
}

fn calc_dist(p1: &Pos, p2: &Pos, empty_row: &Vec<usize>, empty_col: &Vec<usize>) -> u64 {
    let (mut r1, mut c1) = *p1;
    let (mut r2, mut c2) = *p2;
    if r1 > r2 {
        swap(&mut r1, &mut r2);
    }
    if c1 > c2 {
        swap(&mut c1, &mut c2);
    }
    let empty_row_cnt =
        empty_row.partition_point(|x| *x < r2) - empty_row.partition_point(|x| *x < r1);
    let empty_col_cnt =
        empty_col.partition_point(|x| *x < c2) - empty_col.partition_point(|x| *x < c1);
    ((r2 - r1) + (c2 - c1) + empty_col_cnt * (EXPAND_RATE - 1) + empty_row_cnt * (EXPAND_RATE - 1))
        as u64
}

fn main_d11p1() {
    let map = read_file_line_vec_vec("input/d11p1.txt");
    let galaxy_arr = find_galaxy(&map);
    let empty_row = find_empty_row(&map);
    let empty_col = find_empty_col(&map);

    let mut result = 0;

    for i in 0..galaxy_arr.len() {
        for j in i + 1..galaxy_arr.len() {
            let dist = calc_dist(&galaxy_arr[i], &galaxy_arr[j], &empty_row, &empty_col);
            // println!("{:?} <> {:?} = {}", galaxy_arr[i], galaxy_arr[j], dist);
            result += dist;
        }
    }

    println!("{result}");
}

fn main() {
    main_d11p1();
}
