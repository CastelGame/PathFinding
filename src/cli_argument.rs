use std::collections::HashMap;
use std::env;
use std::process::exit;

#[derive(Debug)]
pub struct CliArguments {
    pub start: u32,
    pub goal: u32,
    pub file_location: String,
    pub start_neighbors: HashMap<u32, u32>,
    pub goal_neighbors: HashMap<u32, u32>,
}

impl CliArguments {
    fn new(args: Vec<String>) -> CliArguments {
        if args.len() < 4 {
            panic!("Not enough arguments, start end file_location");
        }
        let mut start_neighbors: HashMap<u32, u32> = HashMap::new();
        let mut goal_neighbors: HashMap<u32, u32> = HashMap::new();
        if args.len() > 4 {
            start_neighbors = serde_json::from_str::<HashMap<u32, u32>>(&args[4]).unwrap()
        }
        if args.len() > 5 {
            goal_neighbors = serde_json::from_str::<HashMap<u32, u32>>(&args[5]).unwrap()
        }
        CliArguments {
            start: args[1].parse::<u32>().unwrap(),
            goal: args[2].parse::<u32>().unwrap(),
            file_location: args[3].clone(),
            start_neighbors,
            goal_neighbors,
        }
    }
}

pub fn get() -> CliArguments {
    let _args: Vec<String> = env::args().collect();

    let _arg: CliArguments = CliArguments::new(_args);
    return _arg;
}
