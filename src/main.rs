use jwalk::{Error, WalkDir};
use std::env::args;
use std::process;
use regex::Regex;

fn main() -> Result<(), Error> {
    let my_args: Vec<String> = args().collect();

    let config = Finder::new(&my_args);
    let regex_pattern = Regex::new(&config.filename).unwrap();
    for entry in WalkDir::new(&config.path).sort(true) {
        let my_path = entry?.path();
        if regex_pattern.is_match(&my_path.to_str().unwrap()){
        println!("{}", &my_path.display());}
    }
    Ok(())
}

struct Finder {
    path: String,
    filename: String,
}

impl Finder {
    fn new(args: &[String]) -> Finder {
        if args.len() != 3 {
            eprintln!("hint!: command path filename");
            process::exit(1);
        }
        let path = args[1].clone();
        let filename = args[2].clone();

        Finder { path, filename }
    }
}
