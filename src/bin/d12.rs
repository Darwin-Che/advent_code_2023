use std::collections::HashMap;

use advent_code_2023::helper::*;
use itertools::Itertools;

fn check_arrangement(pat: &[u8], group_cnt: &[usize], pos_arr: &Vec<usize>) -> bool {
    let mut line = pat.to_owned();
    for pos in pos_arr {
        line[*pos] = b'#';
    }

    // Check if line is the same as group_cnt
    let groups: Vec<usize> = line
        .iter()
        .group_by(|k| **k == b'#')
        .into_iter()
        .filter(|(k, _)| *k)
        .map(|(_, v)| v.count())
        .collect();

    groups == *group_cnt
}

fn calc_arrangement_count_force(pat: &[u8], group_cnt: &[usize]) -> usize {
    // println!("calc_arrangement_count_force {:?} {:?}", pat, group_cnt);
    let group_sum = group_cnt.iter().sum::<usize>();
    let exist_in_pat = pat.iter().filter(|x| **x == b'#').count();
    if group_sum < exist_in_pat {
        return 0;
    }
    let flexible_cnt = group_sum - exist_in_pat;
    let pos_vec: Vec<usize> = pat
        .iter()
        .enumerate()
        .filter(|(_, val)| **val == b'?')
        .map(|(idx, _)| idx)
        .collect();

    // Create all selection
    pos_vec
        .into_iter()
        .combinations(flexible_cnt)
        .filter(|pos_arr| check_arrangement(pat, group_cnt, pos_arr))
        .count()
}

fn split_pat_with_dot(pat: &[u8]) -> Vec<Vec<u8>> {
    pat.iter()
        .map(|v| *v)
        .group_by(|k| *k == b'.')
        .into_iter()
        .filter(|(k, _)| !*k)
        .map(|(_, v)| v.collect())
        .collect()
}

fn calc_arrangement_count_fast(pat: &[u8], group_cnt: &[usize]) -> usize {
    let pat_arr = split_pat_with_dot(pat);
    // So we have pat_arr as a list of string that contains only # and ?
    // we need to match that against group_cnt array
    // We can use 2d dp to solve this
    // dp_arr[i][j] = using the first i pat_arr, we can cover the first j group
    // Thus dp_arr[i][j] = sum(dp_arr[i-1][k], 0 <= k <= j)
    let mut dp_arr = vec![vec![0; group_cnt.len() + 1]; pat_arr.len() + 1];
    dp_arr[0][0] = 1;
    for j in 1..=group_cnt.len() {
        dp_arr[0][j] = 0;
    }
    for i in 1..=pat_arr.len() {
        for j in 0..=group_cnt.len() {
            let mut acc = 0;
            for k in 0..=j {
                if dp_arr[i - 1][k] == 0 {
                    continue;
                }
                acc += dp_arr[i - 1][k]
                    * calc_arrangement_count_force(&pat_arr[i - 1], &group_cnt[k..j]);
            }
            dp_arr[i][j] = acc;
        }
    }
    dp_arr[pat_arr.len()][group_cnt.len()]
}

fn calc_arrangement_count_v3(
    pat: &[u8],
    group_cnt: &[usize],
    acc: usize,
    dp_map: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if pat.len() == 0 {
        if group_cnt.len() == 0 && acc == 0 {
            return 1;
        }
        if group_cnt.len() == 1 && acc == group_cnt[0] {
            return 1;
        }
        return 0;
    }
    if group_cnt.len() == 0 {
        if pat.contains(&b'#') {
            return 0;
        }
        return 1;
    }

    let key = (pat.len(), group_cnt.len(), acc);
    if let Some(v) = dp_map.get(&key) {
        return *v;
    }

    if acc == group_cnt[0] {
        if pat[0] == b'#' {
            return 0;
        } else {
            let v = calc_arrangement_count_v3(&pat[1..], &group_cnt[1..], 0, dp_map);
            *dp_map.entry(key).or_default() = v;
            return v;
        }
    }

    if pat[0] == b'#' {
        let v = calc_arrangement_count_v3(&pat[1..], &group_cnt[..], acc + 1, dp_map);
        *dp_map.entry(key).or_default() = v;
        return v;
    } else if pat[0] == b'?' {
        let mut v = 0;
        // let ? be #
        v += calc_arrangement_count_v3(&pat[1..], &group_cnt[..], acc + 1, dp_map);
        if acc == 0 {
            // let ? be .
            v += calc_arrangement_count_v3(&pat[1..], &group_cnt[..], acc, dp_map);
        }
        *dp_map.entry(key).or_default() = v;
        return v;
    } else if acc == 0 {
        let v = calc_arrangement_count_v3(&pat[1..], &group_cnt[..], acc, dp_map);
        *dp_map.entry(key).or_default() = v;
        return v;
    }
    0
}

fn main_d12p2() {
    let mut result = 0;
    let line_iter = read_file_line_iter("input/d12p1.txt");
    for line in line_iter {
        let str_arr = line.split(' ').collect::<Vec<&str>>();
        let str1 = str_arr[0];
        let str2 = str_arr[1];
        let pat = str1.as_bytes();
        let group_cnt: Vec<usize> = str2
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        print!("{:?} {:?} = ", pat, group_cnt);
        let mut dp_map = HashMap::new();
        // let cnt = calc_arrangement_count_force(&five_pat(pat), &five_group(&group_cnt));
        let cnt =
            calc_arrangement_count_v3(&five_pat(pat), &five_group(&group_cnt), 0, &mut dp_map);
        // let cnt = calc_arrangement_count_v3(pat, &group_cnt, 0, &mut dp_map);
        println!("{:?}", cnt);
        println!("{:?}", dp_map);
        result += cnt;
    }
    println!("{result}");
}

fn five_pat(pat: &[u8]) -> Vec<u8> {
    // [pat, &[b'?'], pat].concat()
    [pat, &[b'?'], pat, &[b'?'], pat, &[b'?'], pat, &[b'?'], pat].concat()
}

fn five_group(group_cnt: &[usize]) -> Vec<usize> {
    // [group_cnt, group_cnt].concat()
    [group_cnt, group_cnt, group_cnt, group_cnt, group_cnt].concat()
}

fn main_d12p1() {
    let mut result = 0;
    let line_iter = read_file_line_iter("input/d12p1.txt");
    for line in line_iter {
        let str_arr = line.split(' ').collect::<Vec<&str>>();
        let str1 = str_arr[0];
        let str2 = str_arr[1];
        let pat = str1.as_bytes();
        let group_cnt: Vec<usize> = str2
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let cnt = calc_arrangement_count_fast(pat, &group_cnt);
        println!("{:?} {:?} = {:?}", pat, group_cnt, cnt);
        result += cnt;
    }
    println!("{result}");
}

fn main() {
    main_d12p2();
}
