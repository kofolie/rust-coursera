use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("My path is {}.", args[1]);
    let link = args[1].clone();

    let file = File::open(link);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}