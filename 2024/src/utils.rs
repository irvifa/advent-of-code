use std::env;

pub struct Args {
    pub file_path: String,
}

impl Args {
    pub fn parse() -> Self {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            eprintln!("Usage: {} <input file>", args[0]);
            std::process::exit(1);
        }
        Args {
            file_path: args[1].clone(),
        }
    }
}
