mod cli_argument;

use pathfinding::prelude::dijkstra;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Debug)]
struct Output {
    path: Vec<u32>,
    distance: usize,
}

fn main() {
    let args: cli_argument::CliArguments = cli_argument::get();

    let map: HashMap<u32, HashMap<u32, u32>> = get_map(args.file_location);

    let reachable: (Vec<u32>, usize) = dijkstra(
        &args.start,
        |p: &u32| successors(p, map[p].clone()),
        |&p| p == args.goal,
    )
    .unwrap();

    let output: Output = Output {
        path: reachable.0,
        distance: reachable.1,
    };

    print!("{}", serde_json::to_string(&output).unwrap());
}
