use advent_code_2023::helper::*;
use std::cmp;

fn test_game(game_str: &str) -> bool {
    let mut total_x = parse_game(game_str);
    println!("{:?}", total_x);
    return total_x.0 <= 12 && total_x.1 <= 13 && total_x.2 <= 14;
}

fn parse_game(game_str: &str) -> (u32, u32, u32) {
    let mut total_x = (0, 0, 0);
    let ball_arr = game_str.split(',').collect::<Vec<&str>>();
    for ball_str in ball_arr {
        let ball_tuple = ball_str
            .trim()
            .split_ascii_whitespace()
            .collect::<Vec<&str>>();
        let x = ball_tuple[0].parse::<u32>().unwrap();
        let color = ball_tuple[1];
        match color {
            "red" => total_x.0 += x,
            "green" => total_x.1 += x,
            "blue" => total_x.2 += x,
            _ => (),
        };
    }
    total_x
}

fn main_d2p1() {
    let mut result = 0;
    for s in read_file_line_iter("input/d2p1.txt") {
        let mut sarr = s.split(':').collect::<Vec<&str>>();
        let game_num = sarr[0].split(' ').nth(1).unwrap().parse::<u32>().unwrap();
        let game_arr = sarr[1].split(';');

        let mut game_possible = true;
        for game_str in game_arr {
            game_possible = game_possible && test_game(game_str);
        }

        println!("{game_num} {game_possible}");
        if game_possible {
            result += game_num;
        }
    }
    println!("Result: {result}");
}

fn main_d2p2() {
    let mut result = 0;
    for s in read_file_line_iter("input/d2p1.txt") {
        let mut sarr = s.split(':').collect::<Vec<&str>>();
        let game_num = sarr[0].split(' ').nth(1).unwrap().parse::<u32>().unwrap();
        let game_arr = sarr[1].split(';');

        let mut game_min = (0, 0, 0);
        for game_str in game_arr {
            let game_num = parse_game(game_str);
            game_min.0 = cmp::max(game_min.0, game_num.0);
            game_min.1 = cmp::max(game_min.1, game_num.1);
            game_min.2 = cmp::max(game_min.2, game_num.2);
        }

        let game_power = game_min.0 * game_min.1 * game_min.2;

        println!("{game_num} {game_power}");
        result += game_power;
    }
    println!("Result: {result}");
}

fn main() {
    main_d2p2();
}
