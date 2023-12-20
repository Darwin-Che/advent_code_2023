use advent_code_2023::helper::*;

// S second -> (S - H) * H > D -> H^2 - S * H + D < 0

fn main_d6p1() {
    let lines = read_file_line_vec("input/d6p1.txt");
    let time_arr: Vec<u32> = lines[0]
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let dist_arr: Vec<u32> = lines[1]
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let n = dist_arr.len();

    let mut result = 1;

    for i in 0..n {
        let time = time_arr[i];
        let dist = dist_arr[i];
        println!("time={time} dist={dist}");

        let mut x = 0;

        for h in 0..=time {
            if (time - h) * h > dist {
                x += 1;
            }
        }
        result *= x;
    }

    println!("{result}");
}

fn main_d6p2() {
    let lines = read_file_line_vec("input/d6p1.txt");
    let time: u64 = lines[0]
        .split(":")
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();
    let dist: u64 = lines[1]
        .split(":")
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();

    println!("time={time} dist={dist}");

    // H^2 - S * H + D < 0
}

fn main() {
    main_d6p2();
}
