use advent_code_2023::helper::*;
use itertools::Itertools;
use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    process::exit,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Device {
    F(bool),
    C(BTreeMap<String, bool>),
    B(),
}

type Edges = Vec<String>;
type Graph = HashMap<String, Edges>;
type Devices = BTreeMap<String, Device>;

type State = (Graph, Devices);

fn parse() -> State {
    let mut devices = Devices::new();
    let mut graph = Graph::new();
    for line in read_file_line_iter("input/d20p1.txt") {
        let arr = line.split("->").collect_vec();
        let dests = arr[1]
            .split(",")
            .map(str::trim)
            .map(str::to_string)
            .collect_vec();
        let src = arr[0].trim();
        let (device_name, device) = if src == "broadcaster" {
            (src.to_string(), Device::B())
        } else if &src[0..1] == "%" {
            (src[1..].to_string(), Device::F(false))
        } else if &src[0..1] == "&" {
            (src[1..].to_string(), Device::C(BTreeMap::new()))
        } else {
            panic!();
        };
        devices.insert(device_name.clone(), device);
        graph.insert(device_name, dests);
    }
    println!("{:?}", devices);
    // Find reverse edges for C
    for (src, edges) in graph.clone() {
        for dest in edges {
            if let Some(x) = devices.get_mut(&dest) {
                if let Device::C(c) = x {
                    c.insert(src.clone(), false);
                }
            } else {
                devices.insert(dest.clone(), Device::B());
                graph.insert(dest.clone(), Vec::new());
                println!("Add Empty Device {}", dest);
            }
        }
    }
    (graph, devices)
}

type Pulse = (String, String, bool);
type PulseQueue = VecDeque<Pulse>;

fn process(pulse: Pulse, state: &mut State) -> Vec<Pulse> {
    let mut result = vec![];

    let (src, dest, signal) = pulse;
    let (graph, devices) = state;

    let next_dests = graph.get(&dest).unwrap();

    match devices.get_mut(&dest).unwrap() {
        Device::B() => {
            for next_dest in next_dests {
                result.push((dest.clone(), next_dest.clone(), signal));
            }
        }
        Device::C(c) => {
            *c.get_mut(&src).unwrap() = signal;
            let next_signal = !c.values().all(|b| *b);
            for next_dest in next_dests {
                result.push((dest.clone(), next_dest.clone(), next_signal));
            }
        }
        Device::F(f) => {
            if !signal {
                *f = !*f;
                for next_dest in next_dests {
                    result.push((dest.clone(), next_dest.clone(), *f));
                }
            }
        }
    }

    result
}

fn press(state: &mut State) -> (u64, u64) {
    let mut result = (0, 1);

    let mut pulse_queue = PulseQueue::new();
    pulse_queue.push_back(("button".to_string(), "broadcaster".to_string(), false));

    while let Some(pulse) = pulse_queue.pop_front() {
        let next_pulses = process(pulse, state);
        for next_pulse in next_pulses {
            // println!("{:?}", next_pulse);
            if next_pulse.2 {
                result.0 += 1;
            } else {
                result.1 += 1;
            }
            pulse_queue.push_back(next_pulse);
        }
    }
    result
}

const LIMIT: usize = 10000;

fn main_d20p1() {
    let mut state = parse();
    let mut cache = HashMap::<Devices, usize>::new();
    let mut cache_result = HashMap::<usize, (u64, u64)>::new();

    let mut lop = (0, 0);
    for j in 0..LIMIT {
        if let Some(i) = cache.get(&state.1) {
            println!("Loop detected : {i} - {j}");
            lop = (*i, j);
            break;
        } else {
            let entry = cache.entry(state.1.clone());
            let (high_signals, low_signals) = press(&mut state);
            entry.or_insert(j);
            cache_result.insert(j, (high_signals, low_signals));
            println!("Finished Press {j}");
        }
    }

    if lop == (0, 0) {
        let mut result = (0, 0);
        for (i, (a, b)) in cache_result {
            result.0 += a;
            result.1 += b;
        }
        println!("result = {:?}", result);
        println!("result = {:?}", result.0 * result.1);
        return;
    }

    // All below are for loop reduction

    let mut result = (0, 0);

    // Add things before the loop
    for i in 0..lop.0 {
        let tp = *cache_result.get(&i).unwrap();
        result.0 += tp.0;
        result.1 += tp.1;
    }

    // Add things in the loop
    let mut lop_result = (0, 0);
    for i in lop.0..lop.1 {
        let tp = *cache_result.get(&i).unwrap();
        lop_result.0 += tp.0;
        lop_result.1 += tp.1;
    }

    // Add all loops
    let mut idx = lop.0;
    let lop_len = lop.1 - lop.0;
    while idx + lop_len < LIMIT {
        result.0 += lop_result.0;
        result.1 += lop_result.1;
        idx += lop_len;
    }

    // Add anything left
    while idx < LIMIT {
        let i = (idx - lop.0) % lop_len;
        let tp = *cache_result.get(&i).unwrap();
        result.0 += tp.0;
        result.1 += tp.1;
        idx += 1;
    }

    println!("{:?}", result);
    println!("{:?}", result.0 * result.1);
}

fn main_d20p2() {
    let mut state = parse();

    let mut cnt = 0;

    loop {
        let mut pulse_queue = PulseQueue::new();
        pulse_queue.push_back(("button".to_string(), "broadcaster".to_string(), false));

        cnt += 1;
        println!("Button Cnt = {:?}", cnt);

        while let Some(pulse) = pulse_queue.pop_front() {
            let next_pulses = process(pulse, &mut state);
            for next_pulse in next_pulses {
                if next_pulse.1 == "rx" && !next_pulse.2 {
                    exit(0);
                }
                pulse_queue.push_back(next_pulse);
            }
        }
    }
}

fn main() {
    main_d20p2();
}
