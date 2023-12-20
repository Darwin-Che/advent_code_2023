use std::{
    collections::{HashSet, VecDeque},
    mem::swap,
};

use advent_code_2023::helper::*;

type Maze = Vec<Vec<u8>>;

type Pos = (usize, usize);

fn find_start(maze: &Maze) -> Pos {
    for r in 0..maze.len() {
        for c in 0..maze[r].len() {
            if maze[r][c] == b'S' {
                return (r, c);
            }
        }
    }
    panic!();
}

fn find_adj(maze: &Maze, pos: Pos) -> (Pos, Pos) {
    match maze[pos.0][pos.1] {
        b'|' => ((pos.0 - 1, pos.1), (pos.0 + 1, pos.1)),
        b'-' => ((pos.0, pos.1 - 1), (pos.0, pos.1 + 1)),
        b'L' => ((pos.0 - 1, pos.1), (pos.0, pos.1 + 1)),
        b'J' => ((pos.0 - 1, pos.1), (pos.0, pos.1 - 1)),
        b'7' => ((pos.0 + 1, pos.1), (pos.0, pos.1 - 1)),
        b'F' => ((pos.0 + 1, pos.1), (pos.0, pos.1 + 1)),
        b'S' => {
            let mut tmp = vec![];
            if b"|7F".contains(&maze[pos.0 - 1][pos.1]) {
                tmp.push((pos.0 - 1, pos.1));
            }
            if b"|LJ".contains(&maze[pos.0 + 1][pos.1]) {
                tmp.push((pos.0 + 1, pos.1));
            }
            if b"-LF".contains(&maze[pos.0][pos.1 - 1]) {
                tmp.push((pos.0, pos.1 - 1));
            }
            if b"-J7".contains(&maze[pos.0][pos.1 + 1]) {
                tmp.push((pos.0, pos.1 + 1));
            }
            (tmp[0], tmp[1])
        }
        _ => panic!(),
    }
}

fn trace_link(maze: &Maze, pos: Pos, parent: Pos) -> Pos {
    let (pos1, pos2) = find_adj(maze, pos);
    if pos1 == parent {
        pos2
    } else {
        pos1
    }
}

fn main_d10p1() {
    let maze = read_file_line_vec_vec("input/d10p1.txt");
    let start = find_start(&maze);
    let (mut parent, _) = find_adj(&maze, start);
    let mut pos = start;
    let mut cnt = 0;
    loop {
        parent = trace_link(&maze, pos, parent);
        swap(&mut pos, &mut parent);
        cnt += 1;
        if pos == start {
            break;
        }
    }
    println!("step cnt = {cnt}");
}

// Loc is the middle of the spaces
// X X
//  L
// X X
type Loc = (usize, usize);

fn blocked_horizontal(maze: &Maze, loc: Loc, path: &HashSet<Pos>) -> bool {
    path.contains(&loc)
        && path.contains(&(loc.0, loc.1 + 1))
        && (b"FL-".contains(&maze[loc.0][loc.1]) || b"7J-".contains(&maze[loc.0][loc.1 + 1]))
}

fn blocked_vertical(maze: &Maze, loc: Loc, path: &HashSet<Pos>) -> bool {
    path.contains(&loc)
        && path.contains(&(loc.0 + 1, loc.1))
        && (b"F7|".contains(&maze[loc.0][loc.1]) || b"LJ|".contains(&maze[loc.0 + 1][loc.1]))
}

fn loc_neighbours(maze: &Maze, loc: Loc, path: &HashSet<Pos>) -> Vec<Loc> {
    let mut result = Vec::new();
    // check if top neighour exists
    if loc.0 > 0 && !blocked_horizontal(maze, loc, path) {
        result.push((loc.0 - 1, loc.1));
    }
    if loc.0 < maze.len() - 2 && !blocked_horizontal(maze, (loc.0 + 1, loc.1), path) {
        result.push((loc.0 + 1, loc.1));
    }
    if loc.1 > 0 && !blocked_vertical(maze, loc, path) {
        result.push((loc.0, loc.1 - 1));
    }
    if loc.1 < maze[0].len() - 2 && !blocked_vertical(maze, (loc.0, loc.1 + 1), path) {
        result.push((loc.0, loc.1 + 1));
    }
    result
}

fn loc_bfs(maze: &Maze, start_loc: Loc, path: &HashSet<Pos>) -> HashSet<Loc> {
    let mut visited = HashSet::new();
    visited.insert(start_loc);
    let mut q = VecDeque::new();
    q.push_back(start_loc);
    while let Some(loc) = q.pop_front() {
        let neighbours = loc_neighbours(maze, loc, path);
        for neighbour in neighbours {
            if visited.contains(&neighbour) {
                continue;
            }
            visited.insert(neighbour);
            q.push_back(neighbour);
        }
    }
    visited
}

fn loc_to_pos(maze: &Maze, loc: Loc) -> Vec<Pos> {
    let mut result = Vec::new();
    result.push(loc);
    if loc.0 == maze.len() - 2 {
        result.push((loc.0 + 1, loc.1));
    }
    if loc.1 == maze[0].len() - 2 {
        result.push((loc.0, loc.1 + 1));
    }
    if loc.0 == maze.len() - 2 && loc.1 == maze[0].len() - 2 {
        result.push((loc.0 + 1, loc.1 + 1));
    }
    return result;
}

fn pos_to_loc(maze: &Maze, mut pos: Pos) -> Loc {
    if pos.0 == maze.len() - 1 {
        pos.0 -= 1;
    }
    if pos.1 == maze[0].len() - 1 {
        pos.1 -= 1;
    }
    pos
}

fn print_loc(maze: &Maze, locs: &HashSet<Loc>) {
    let mut p = vec![vec!['.'; maze[0].len() - 1]; maze.len() - 1];
    for loc in locs {
        p[loc.0][loc.1] = 'O';
    }
    for v in p {
        for c in v {
            print!("{c}");
        }
        println!();
    }
}

fn print_pos(maze: &Maze, locs: &HashSet<Pos>) {
    let mut p = vec![vec!['.'; maze[0].len()]; maze.len()];
    for loc in locs {
        p[loc.0][loc.1] = 'O';
    }
    for v in p {
        for c in v {
            print!("{c}");
        }
        println!();
    }
}

fn patch_maze(maze: Maze) -> Maze {
    let r = maze.len();
    let c = maze[0].len();
    let mut new_maze = vec![vec![b'.'; c + 2]; r + 2];
    for i in 0..r {
        for j in 0..c {
            new_maze[i + 1][j + 1] = maze[i][j];
        }
    }
    new_maze
}

fn main_d10p2() {
    let mut maze = read_file_line_vec_vec("input/d10p1.txt");
    maze = patch_maze(maze);
    let start = find_start(&maze);
    let (mut parent, _) = find_adj(&maze, start);
    let mut pos = start;
    let mut cnt = 0;
    let mut path = HashSet::new();
    loop {
        path.insert(pos);
        parent = trace_link(&maze, pos, parent);
        swap(&mut pos, &mut parent);
        cnt += 1;
        if pos == start {
            break;
        }
    }

    let out_loc = loc_bfs(&maze, (0, 0), &path);
    // print_loc(&maze, &out_loc);

    let out_pos: HashSet<Pos> = out_loc
        .into_iter()
        .flat_map(|loc| {
            loc_to_pos(&maze, loc)
                .into_iter()
                .filter(|pos| !path.contains(&pos))
        })
        .collect();

    print_pos(&maze, &out_pos);

    let total = maze.len() as u64 * maze[0].len() as u64;
    println!("{}", total - path.len() as u64 - out_pos.len() as u64);
}

fn main() {
    main_d10p2();
}
