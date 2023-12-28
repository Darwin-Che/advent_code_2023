use std::{
    collections::{btree_set::Union, BTreeMap, HashMap},
    mem::{replace, swap},
    panic::PanicInfo,
};

use advent_code_2023::helper::*;
use itertools::Itertools;

type IdMap = BTreeMap<usize, String>;
type Edges = HashMap<usize, Vec<(usize, usize)>>;
type Graph = BTreeMap<usize, Edges>;

fn parse(file_name: &str) -> (IdMap, Graph) {
    let mut nodemap = BTreeMap::<String, usize>::new();
    let mut graph = Graph::new();

    let lines = read_file_line_vec(file_name);

    // for line in &lines {
    //     let arr = line.split(":").collect_vec();
    //     let src = arr[0].trim().to_string();
    //     let src_id = nodemap.len();
    //     nodemap.insert(src, src_id);
    //     graph.insert(src_id, BTreeMap::new());
    // }

    for line in lines {
        let arr = line.split(":").collect_vec();
        let src = arr[0].trim().to_string();
        let nodemap_len = nodemap.len();
        let src_id = *nodemap.entry(src.clone()).or_insert(nodemap_len);
        for dest_raw in arr[1].trim().split(' ') {
            let dest = dest_raw.trim().to_string();
            let nodemap_len = nodemap.len();
            let dest_id = *nodemap.entry(dest.clone()).or_insert(nodemap_len);
            graph
                .entry(src_id)
                .or_default()
                .entry(dest_id)
                .or_default()
                .push((src_id, dest_id));
            graph
                .entry(dest_id)
                .or_default()
                .entry(src_id)
                .or_default()
                .push((dest_id, src_id));
        }
    }

    let idmap: IdMap = nodemap.into_iter().map(|(k, v)| (v, k)).collect();

    (idmap, graph)
}

fn combine_nodes(uf: &mut UnionFind, mut s: usize, t: usize, s_edges: &mut Edges, t_edges: &Edges) {
    // println!("    update_nodes");
    // println!("    {:?}", s_edges);
    // println!("    {:?}", t_edges);
    uf.union(s, t);
    s = uf.root(s);
    let mut comb_edges = Edges::new();
    for (mut dest, mut arr) in s_edges.drain().chain(t_edges.clone().drain()) {
        dest = uf.root(dest);
        if dest == s {
            continue;
        }
        comb_edges.entry(dest).or_default().append(&mut arr);
    }

    swap(&mut comb_edges, s_edges);
    // println!("\t == {:?}", s_edges);
}

fn update_nodes(uf: &mut UnionFind, mut s: usize, s_edges: &mut Edges) {
    s = uf.root(s);
    let mut update_edges = Edges::new();
    for (mut dest, mut arr) in s_edges.drain() {
        dest = uf.root(dest);
        if dest == s {
            continue;
        }
        update_edges.entry(dest).or_default().append(&mut arr);
    }

    swap(&mut update_edges, s_edges);
}

fn choose_next(_uf: &UnionFind, edge: &Edges) -> Option<usize> {
    // println!("  choose_next {:?}", edge);
    edge.iter()
        .max_by(|a, b| a.1.len().cmp(&b.1.len()))
        .map(|a| *a.0)
}

fn reduce_phase(graph: &Graph, mut uf: UnionFind) -> (usize, usize, Vec<(usize, usize)>) {
    let mut s = 0;
    let mut s_edges = graph.get(&s).unwrap().clone();
    let mut last_cut = (0, 0, vec![]);

    while let Some(t) = choose_next(&uf, &s_edges) {
        last_cut = (s, t, s_edges.get(&t).unwrap().clone());
        // println!("  phase_merge {s} {t}");
        combine_nodes(&mut uf, s, t, &mut s_edges, graph.get(&t).unwrap());
        s = t;
    }

    last_cut
}

fn main_d25p1() {
    let (mut idmap, mut graph) = parse("input/d25p1.txt");

    // println!("{:?}", idmap);
    // println!("{:?}", graph);

    let n = graph.len();
    let mut uf = UnionFind::new(n);

    let mut mincut = (usize::MAX, 0, 0); // (val, s, t)
    let mut result = 0;

    for _ in 0..n - 1 {
        // Try to reduce graph from id = 0
        // return (s, t) and Vec<(usize, usize)> as the s-t cut
        let (s, t, cut) = reduce_phase(&graph, uf.clone());
        println!("reduce_phase {:?} {:?} {:?}", s, t, cut);

        if cut.len() < mincut.0 {
            println!("{:?} == {:?}", cut.len(), cut);
            mincut = (cut.len(), s, t);
            println!("size = {}", uf.size(t));
            result = (n as u32 - uf.size(t)) * uf.size(t);
        }

        let t_edges = replace(graph.get_mut(&t).unwrap(), Edges::new());

        combine_nodes(&mut uf, s, t, graph.get_mut(&s).unwrap(), &t_edges);
        update_nodes(&mut uf, 0, graph.get_mut(&0).unwrap());
    }

    println!("result = {result}");
}

fn main() {
    main_d25p1();
}
