use std::env;
use std::process;
mod config;
use config::read_source;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source_code = read_source(&args).unwrap_or_else(|_|{
        println!("Invalid source code file provided.");
        process::exit(1);
    });
    println!("{}", source_code);

}
