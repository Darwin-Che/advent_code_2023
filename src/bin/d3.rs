use advent_code_2023::helper::*;

fn is_symbol(ch: u8) -> bool {
    return !((ch >= b'0' && ch <= b'9') || (ch == b'.'));
}

fn is_symbol_arr(arr: &[u8]) -> bool {
    return arr.iter().any(|x| is_symbol(*x));
}

fn is_digit(ch: u8) -> bool {
    return ch >= b'0' && ch <= b'9';
}

fn check_if_part(i: usize, mut s: usize, mut e: usize, graph: &Vec<Vec<u8>>) -> bool {
    if s > 0 {
        s -= 1;
    }
    if e < graph[0].len() - 1 {
        e += 1;
    }
    if is_symbol(graph[i][s]) || is_symbol(graph[i][e]) {
        return true;
    }
    if i > 0 && is_symbol_arr(&graph[i - 1][s..=e]) {
        return true;
    }
    if i < graph.len() - 1 && is_symbol_arr(&graph[i + 1][s..=e]) {
        return true;
    }
    return false;
}

fn get_num_locs(line: &[u8]) -> Vec<(u64, usize, usize)> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut val = 0;
    for i in 0..=line.len() {
        if i == line.len() || !is_digit(line[i]) {
            if start != i {
                result.push((val, start, i - 1));
            }
            val = 0;
            start = i + 1;
        } else {
            val = val * 10 + (line[i] - b'0') as u64;
        }
    }
    return result;
}

fn calc_line(i: usize, graph: &Vec<Vec<u8>>) -> u64 {
    let mut result = 0;
    let mut count = 0;
    for (val, s, e) in get_num_locs(&graph[i]) {
        if check_if_part(i, s, e, graph) {
            result += val;
            count += 1;
            println!("{val}   {count}");
        }
    }
    result
}

fn main_d3p1() {
    let graph = read_file_line_vec_vec("input/d3p2.txt");

    let mut result = 0;
    for i in 0..graph.len() {
        result += calc_line(i, &graph);
    }
    println!("{result}");
}

fn check_if_gear_line(j: usize, num_locs_line: &Vec<(u64, usize, usize)>, result: &mut Vec<u64>) {
    for (val, s, e) in num_locs_line {
        if (*s == 0 || j >= *s - 1) && j <= *e + 1 {
            result.push(*val);
        }
    }
}

fn check_if_gear(i: usize, j: usize, num_locs: &Vec<Vec<(u64, usize, usize)>>) -> u64 {
    let mut result = vec![];
    if i > 0 {
        check_if_gear_line(j, &num_locs[i - 1], &mut result);
    }
    check_if_gear_line(j, &num_locs[i], &mut result);
    if i < num_locs.len() - 1 {
        check_if_gear_line(j, &num_locs[i + 1], &mut result);
    }
    if result.len() == 2 {
        return result[0] * result[1];
    }
    return 0;
}

fn main_d3p2() {
    let graph = read_file_line_vec_vec("input/d3p2.txt");

    let mut num_locs = vec![];
    for i in 0..graph.len() {
        num_locs.push(get_num_locs(&graph[i]));
    }

    let mut result = 0;

    for i in 0..graph.len() {
        for j in 0..graph[i].len() {
            if graph[i][j] == b'*' {
                result += check_if_gear(i, j, &num_locs);
            }
        }
    }

    println!("{result}");
}

fn main() {
    main_d3p1();
}
