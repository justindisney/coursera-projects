use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("My path is {}.", args[1]);

    let file = File::open(args[1].clone());
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                std::io::ErrorKind::PermissionDenied => {
                    panic!("Permission denied: {}", error)
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

    // open file for writing; use match for error handling
    let mut file = match File::create("output.txt") {
        Ok(file) => file,
        Err(error) => {
            panic!("Error creating file: {}", error)
        }
    };

    // write to file
    match file.write_all(b"Hello, World!") {
        Ok(_) => println!("Successfully wrote to file"),
        Err(error) => {
            panic!("Error writing to file: {}", error)
        }
    }


}
