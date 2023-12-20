use advent_code_2023::helper::*;

fn calc_next(arr: &[i64], tail: i64) -> i64 {
    // println!("{:?} {:?}", arr, tail);
    let n = arr.len();
    if n == 1 {
        return tail + arr[0];
    }
    if arr.iter().all(|x| *x == 0) {
        return tail;
    }
    let mut next_arr = vec![0; n - 1];
    for i in 0..n - 1 {
        next_arr[i] = arr[i + 1] - arr[i];
    }
    calc_next(&next_arr, arr[n - 1] + tail)
}

fn main_d9p1() {
    let mut result = 0;
    let file_iter = read_file_line_iter("input/d9p1.txt");
    for line in file_iter {
        let arr: Vec<_> = line
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let next_val = calc_next(&arr, 0);
        result += next_val;
    }
    println!("{result}");
}

fn calc_next_2(arr: &[i64], mut tail: Vec<i64>) -> Vec<i64> {
    // println!("{:?} {:?}", arr, tail);
    let n = arr.len();
    if n == 1 {
        tail.push(arr[0]);
        return tail;
    }
    if arr.iter().all(|x| *x == 0) {
        return tail;
    }
    let mut next_arr = vec![0; n - 1];
    for i in 0..n - 1 {
        next_arr[i] = arr[i + 1] - arr[i];
    }
    tail.push(arr[0]);
    calc_next_2(&next_arr, tail)
}

fn main_d9p2() {
    let mut result = 0;
    let file_iter = read_file_line_iter("input/d9p1.txt");
    for line in file_iter {
        let arr: Vec<_> = line
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let tail = calc_next_2(&arr, Vec::new());
        let mut next_val = 0;
        for i in 0..tail.len() {
            if i % 2 == 0 {
                next_val += tail[i];
            } else {
                next_val -= tail[i];
            }
        }
        result += next_val;
    }
    println!("{result}");
}

fn main() {
    main_d9p2();
}
