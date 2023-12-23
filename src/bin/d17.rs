use advent_code_2023::helper::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    NIL,
    STR,
    TUR,
}

type Graph = Vec<Vec<u32>>;
type Node = (Dir, Dir, usize, usize);

fn main_d17p1() {
    let file_iter = read_file_line_iter("input");
    let graph: Graph = file_iter
        .map(|s| {
            s.chars()
                .into_iter()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect()
        })
        .collect();
}

fn main() {
    main_d17p1();
}
