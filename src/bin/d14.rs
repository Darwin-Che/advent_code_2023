use advent_code_2023::helper::*;

type Graph = Vec<Vec<u8>>;

fn main_d14p1() {
    let graph = read_file_line_vec_vec("input/d14p1.txt");
    let rows = graph.len();
    let cols = graph[0].len();
    let mut result = 0;
    for c in 0..cols {
        let mut idx = 0;
        for r in 0..rows {
            if graph[r][c] == b'#' {
                idx = r + 1;
            }
            if graph[r][c] == b'O' {
                // println!("{}", rows - idx);
                result += (rows - idx);
                idx += 1;
            }
        }
    }

    println!("{result}");
}

fn main() {
    main_d14p1();
}
