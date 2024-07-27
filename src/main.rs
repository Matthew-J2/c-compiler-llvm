use std::env;
use std::fs;
use std::io;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let source_code = read_source(&args).unwrap_or_else(|_|{
        println!("Invalid source code file provided.");
        process::exit(1);
    });
    println!("{}", source_code);

}

fn read_source(args: &[String]) -> Result<String, io::Error> {
    let source_file = args[1].clone();
    let source_code = fs::read_to_string(source_file)?;
    Ok(source_code)
}