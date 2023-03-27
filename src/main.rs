use std::env;
use minigrep::Config;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(69);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    config.run().unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(69);
    });
}



