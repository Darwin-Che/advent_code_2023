use advent_code_2023::helper::*;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Ray {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    vxi: i64,
    vyi: i64,
    vzi: i64,
}

type Rays = Vec<Ray>;

fn parse_input() -> Rays {
    let mut rays = vec![];
    for line in read_file_line_iter("input/d24p1.txt") {
        let arr: Vec<&str> = line.split('@').collect();
        let pos: Vec<f64> = arr[0]
            .split(',')
            .map(|v| v.trim().parse().unwrap())
            .collect();
        let vel: Vec<i64> = arr[1]
            .split(',')
            .map(|v| v.trim().parse().unwrap())
            .collect();
        rays.push(Ray {
            x: pos[0],
            y: pos[1],
            z: pos[2],
            vx: vel[0] as f64,
            vy: vel[1] as f64,
            vz: vel[2] as f64,
            vxi: vel[0],
            vyi: vel[1],
            vzi: vel[2],
        })
    }
    rays
}

// const AREA_X: (f64, f64) = (7f64, 27f64);
const AREA_X: (f64, f64) = (200000000000000f64, 400000000000000f64);
// const AREA_Y: (f64, f64) = (7f64, 27f64);
const AREA_Y: (f64, f64) = (200000000000000f64, 400000000000000f64);

fn test_intersect(ray1: Ray, ray2: Ray) -> bool {
    // ray1 : (x - x1) / vx1 = (y - y1) / vy1 => y = vy1 / vx1 * (x - x1) + y1
    // ray2 : (x - x2) / vx2 = (y - y2) / vy2 => y = vy2 / vx2 * (x - x2) + y2
    // let y equal, calc x
    // vy1 / vx1 * (x - x1) + y1 = vy2 / vx2 * (x - x2) + y2
    // vy1 / vx1 * (x - x1) - vy2 / vx2 * (x - x2) = y2 - y1
    // (vy1 / vx1 - vy2 / vx2) * x - vy1 / vx1 * x1 + vy2 / vx2 * x2 = y2 - y1
    // (vy1 / vx1 - vy2 / vx2) * x = y2 - y1 - vy2 / vx2 * x2 + vy1 / vx1 * x1
    if ray1.vyi * ray2.vxi == ray1.vxi * ray2.vyi {
        return false;
    }
    let de = ray1.vy / ray1.vx - ray2.vy / ray2.vx;
    let nu = ray2.y - ray1.y - ray2.vy / ray2.vx * ray2.x + ray1.vy / ray1.vx * ray1.x;
    let x = nu / de;
    // check if (x - x0) / vx >= 0
    if (x - ray1.x) / ray1.vx < 0.0 {
        return false;
    }
    if (x - ray2.x) / ray2.vx < 0.0 {
        return false;
    }
    if x < AREA_X.0 || x > AREA_X.1 {
        return false;
    }
    let y = ray1.vy / ray1.vx * (x - ray1.x) + ray1.y;
    if y < AREA_Y.0 || y > AREA_Y.1 {
        return false;
    }
    // println!("{:?} {:?}\n Intersect x = {:?} y = {:?}", ray1, ray2, x, y);
    true
}

fn main_d24p1() {
    let rays = parse_input();
    println!("{:?}", rays);
    let n = rays.len();
    let mut result = 0;
    for i in 0..n {
        for j in i + 1..n {
            if test_intersect(rays[i], rays[j]) {
                result += 1;
            }
        }
    }
    println!("result = {result}");
}

// x0 + xv0 * t1 = x1 + xv1 * t1
// x1 - x0 = (xv0 - xv1) * t1
// xv0 - xv1 = (x1 - x0) * t1
// xv0 = xv1 + (x1 - x0) * t1

// x0 + xv0 * t2 = x2 + xv2 * t2
// xv0 = xv2 + (x2 - x0) * t2

// xv1 - xv2 = (x2 - x0) * t2 - (x1 - x0) * t1

// x0 + xv0 * t3 = x3 + xv3 * t3

fn main() {
    main_d24p1();
}
