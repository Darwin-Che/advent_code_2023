use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use advent_code_2023::helper::*;
use itertools::Itertools;

#[derive(Copy, Clone)]
struct Brick {
    x: usize,
    y: usize,
    z: usize,
    xd: usize,
    yd: usize,
    zd: usize,
}

type Bricks = Vec<Brick>;

type Depends = HashMap<usize, HashSet<usize>>; // Brick X replies on Brick Y, Z, ...

type TopView = Vec<Vec<(usize, usize)>>; // (min Z height that is empty, Corresponding Brick ID)

fn parse_brick(s: String) -> Brick {
    let arr: Vec<&str> = s.split('~').collect();
    let a0: Vec<usize> = arr[0].split(',').map(|s| s.parse().unwrap()).collect();
    let a1: Vec<usize> = arr[1].split(',').map(|s| s.parse().unwrap()).collect();
    Brick {
        x: a0[0],
        y: a0[1],
        z: a0[2],
        xd: a1[0] - a0[0] + 1,
        yd: a1[1] - a0[1] + 1,
        zd: a1[2] - a0[2] + 1,
    }
}

fn parse_bricks() -> Bricks {
    let mut snapshot: Bricks = read_file_line_iter("input/d22p1.txt")
        .into_iter()
        .map(parse_brick)
        .collect(); // All bricks
    snapshot.sort_by(|a, b| a.z.cmp(&b.z));
    snapshot
}

fn fall_brick(brick_id: usize, brick: &Brick, top_view: &mut TopView) -> (usize, HashSet<usize>) {
    let mut m: BTreeMap<usize, HashSet<usize>> = BTreeMap::new();
    m.insert(0, HashSet::new());
    for x in brick.x..brick.x + brick.xd {
        for y in brick.y..brick.y + brick.yd {
            let (z, id) = top_view[x][y];
            if z == 0 {
                continue;
            }
            m.entry(z).or_default().insert(id);
        }
    }
    let (z, ids) = m.last_key_value().unwrap();
    // Update the top_view
    for x in brick.x..brick.x + brick.xd {
        for y in brick.y..brick.y + brick.yd {
            top_view[x][y] = (z + brick.zd, brick_id)
        }
    }
    (*z, ids.clone())
}

fn bricks_after_fall(snapshot: &Bricks) -> (Bricks, Depends) {
    let x_max = snapshot.iter().map(|b| b.x).max().unwrap() + 1;
    let y_max = snapshot.iter().map(|b| b.y).max().unwrap() + 1;
    let mut top_view: TopView = vec![vec![(0, 0); x_max]; y_max];
    let mut bricks = Bricks::new();
    let mut depends = Depends::new();
    for (brick_id, brick) in snapshot.iter().enumerate() {
        let (z, d) = fall_brick(brick_id, brick, &mut top_view);
        depends.insert(brick_id, d);
        bricks.push(Brick { z, ..*brick });
    }
    (bricks, depends)
}

fn print_depends(depends: &Depends, _: &Bricks) {
    for (id, depend) in depends {
        println!("{:?} : {:?}", id, depend);
    }
}

fn main_d22p1() {
    let snapshot = parse_bricks();
    let (bricks, depends) = bricks_after_fall(&snapshot); // Position of the bricks after fall down

    // print_depends(&depends, &bricks);
    let mut notsafe = HashSet::new();
    for (_id, depend) in depends {
        if depend.len() == 1 {
            notsafe.insert(depend.into_iter().next().unwrap());
        }
    }
    println!("result = {}", bricks.len() - notsafe.len());
}

fn chain_reaction(id: usize, mut depends: Depends, reverse_depends: &Depends) -> usize {
    let mut result = 0;
    let mut q = VecDeque::new();
    q.push_front(id);
    while let Some(i) = q.pop_back() {
        for rd in reverse_depends.get(&i).into_iter().flatten() {
            depends.entry(*rd).and_modify(|v| {
                v.remove(&i);
            });
            if depends.get(rd).unwrap().len() == 0 {
                q.push_front(*rd);
                result += 1;
            }
        }
    }
    result
}

fn main_d22p2() {
    let snapshot = parse_bricks();
    let (bricks, depends) = bricks_after_fall(&snapshot); // Position of the bricks after fall down

    // Creating a reverse lookup
    let mut reverse_depends = Depends::new();
    for (id, depend) in &depends {
        for d in depend {
            reverse_depends.entry(*d).or_default().insert(*id);
        }
    }

    let mut notsafe = HashSet::new();
    for (_id, depend) in &depends {
        if depend.len() == 1 {
            notsafe.insert(*depend.into_iter().next().unwrap());
        }
    }

    let mut result = 0;
    for id in notsafe {
        let depends_clone = depends.clone();
        let x = chain_reaction(id, depends_clone, &reverse_depends);
        println!("{} = {}", id, x);
        result += x;
    }

    println!("result = {result}");
}

fn main() {
    main_d22p2();
}
