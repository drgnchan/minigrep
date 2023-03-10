use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    println!("query is {}, filename is {}", config.query, config.filename)
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    Config{
        query: args[1].clone(),
        filename: args[2].clone(),
    }
}
