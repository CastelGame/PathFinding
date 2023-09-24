use std::env;

#[derive(Debug)]
pub struct CliArguments {
    pub start: u32,
    pub goal: u32,
    pub file_location: String,
}

impl CliArguments {
    fn new(args: Vec<String>) -> CliArguments {
        if args.len() < 4 {
            panic!("Not enough arguments, start end file_location");
        }

        CliArguments {
            start: args[1].parse::<u32>().unwrap(),
            goal: args[2].parse::<u32>().unwrap(),
            file_location: args[3].clone(),
        }
    }
}

pub fn get() -> CliArguments {
    let _args: Vec<String> = env::args().collect();

    let _arg: CliArguments = CliArguments::new(_args);
    return _arg;
}
