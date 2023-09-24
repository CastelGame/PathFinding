// use serde::{Deserialize, Serialize};

use pathfinding::prelude::dijkstra;
use std::{collections::HashMap, fs};

fn get_map(path: String) -> HashMap<u32, HashMap<u32, u32>> {
    let map = fs::read_to_string(path).unwrap();
    serde_json::from_str::<HashMap<u32, HashMap<u32, u32>>>(&map).unwrap()
}

fn successors(&n: &u32) -> Vec<(u32, usize)> {
    let map: HashMap<u32, HashMap<u32, u32>> =
        get_map("/home/geeksesi/public_html/Castel/Castel/.maps/1.json".to_string());
    let mut successors: Vec<(u32, usize)> = Vec::new();
    for (k, v) in map[&n].iter() {
        successors.push((*k, *v as usize));
    }
    successors
}

fn main() {
    // print!("{:?}", map[&55]);
    static GOAL: u32 = 2111;
    let reachables = dijkstra(&55, successors, |&p| p == GOAL);
    print!("Path to {:?} is ", reachables);
}
