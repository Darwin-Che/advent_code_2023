use advent_code_2023::helper::*;
use std::collections::{HashMap, HashSet};

fn calc_points(x: u32) -> u32 {
    if x == 0 {
        0
    } else {
        2_u32.checked_pow(x - 1).unwrap()
    }
}

fn main_d4p2() {
    let mut result = 0;
    let lines = read_file_line_iter("input/d4p1.txt");
    let mut cards: HashMap<u32, u32> = HashMap::new();
    for line in lines {
        let split_1: Vec<&str> = line.split(":").collect();
        let (card_str, number_str) = (split_1[0], split_1[1]);
        let split_2: Vec<&str> = number_str.split("|").collect();
        let (win_str, got_str) = (split_2[0], split_2[1]);

        let card_id: u32 = card_str
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        print!("card_id = {card_id}; ");

        let card_num = match cards.remove(&card_id) {
            Some(x) => x + 1,
            None => 1,
        };

        result += card_num;

        let win_set: HashSet<u32> = win_str
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let got_set: HashSet<u32> = got_str
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let x = win_set.intersection(&got_set).count() as u32;

        println!("win_x = {x}");

        for i in card_id + 1..=card_id + x {
            *cards.entry(i).or_default() += card_num;
        }

        println!("cards = {:?}", cards);
    }

    println!("{result}");
}

fn main_d4p1() {
    let mut result = 0;
    let lines = read_file_line_iter("input/d4p1.txt");
    for line in lines {
        let split_1: Vec<&str> = line.split(":").collect();
        let (card_str, number_str) = (split_1[0], split_1[1]);
        let split_2: Vec<&str> = number_str.split("|").collect();
        let (win_str, got_str) = (split_2[0], split_2[1]);

        let card_id: u32 = card_str
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let win_set: HashSet<u32> = win_str
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let got_set: HashSet<u32> = got_str
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        println!("{:?}", win_set);
        println!("{:?}", got_set);

        let x = win_set.intersection(&got_set).count();

        let points = calc_points(x as u32);

        println!("{x} {points}");

        result += points;
    }

    println!("{result}");
}

fn main() {
    main_d4p2();
}
