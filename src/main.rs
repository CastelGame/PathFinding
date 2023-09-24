// use serde::{Deserialize, Serialize};

use pathfinding::prelude::dijkstra;
use std::{collections::HashMap, fs};

fn get_map(path: String) -> HashMap<u32, HashMap<u32, u32>> {
    let map = fs::read_to_string(path).unwrap();
    serde_json::from_str::<HashMap<u32, HashMap<u32, u32>>>(&map).unwrap()
}

fn successors(&_n: &u32, map: HashMap<u32, u32>) -> Vec<(u32, usize)> {
    let mut successors: Vec<(u32, usize)> = Vec::new();
    for (k, v) in map.iter() {
        successors.push((*k, *v as usize));
    }
    successors
}

fn main() {
    let map: HashMap<u32, HashMap<u32, u32>> =
        get_map("/home/geeksesi/public_html/Castel/Castel/.maps/1.json".to_string());

    // print!("{:?}", map[&55]);
    static GOAL: u32 = 2111;
    let reachables = dijkstra(&55, |p: &u32| successors(p, map[p].clone()), |&p| p == GOAL);
    print!("Path to {:?} is ", reachables);
}
