mod cli_argument;

use pathfinding::prelude::dijkstra;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

fn get_map(path: String) -> HashMap<u32, HashMap<u32, u32>> {
    let map = fs::read_to_string(path).unwrap();
    serde_json::from_str::<HashMap<u32, HashMap<u32, u32>>>(&map).unwrap()
}

fn add_new_tile_neighbors(
    mut map: HashMap<u32, HashMap<u32, u32>>,
    start: u32,
    goal: u32,
    start_neighbors: HashMap<u32, u32>,
    end_neighbors: HashMap<u32, u32>,
) -> HashMap<u32, HashMap<u32, u32>> {
    if !map.contains_key(&start) && start_neighbors.is_empty() {
        panic!("Start tile not in map and no start neighbors provided")
    }
    if !map.contains_key(&goal) && end_neighbors.is_empty() {
        panic!("Goal tile not in map and no goal neighbors provided")
    }

    if !start_neighbors.is_empty() {
        map.insert(start, start_neighbors);
    }
    if !end_neighbors.is_empty() {
        map.insert(goal, end_neighbors.clone());
        for (k, v) in end_neighbors.iter() {
            map.get_mut(k).unwrap().insert(goal, *v);
        }
    }

    map
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

    let mut map: HashMap<u32, HashMap<u32, u32>> = get_map(args.file_location);

    map = add_new_tile_neighbors(
        map,
        args.start.clone(),
        args.goal.clone(),
        args.start_neighbors.clone(),
        args.goal_neighbors,
    );

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
