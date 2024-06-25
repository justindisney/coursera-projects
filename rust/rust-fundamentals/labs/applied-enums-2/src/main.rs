use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // The first argument is the size that was used to call the program.
    // Must use quotes to read this as a single argument
    // execute the script using this command: cargo run -- "24 mb"
    println!("Size is: {}.", args[1]);

    // split args[1] on spaces to get the size and the unit
    let size_and_unit: Vec<&str> = args[1].split(" ").collect();

    // check that size is a valid number
    let size: u64 = match size_and_unit[0].parse() {
        Ok(size) => size,
        Err(_) => {
            println!("Invalid size: {}", size_and_unit[0]);
            return;
        }
    };

    // convert unit to a full string
    let unit: &str = match size_and_unit[1] {
        "bytes" => "bytes",
        "kb" => "kilobytes",
        "mb" => "megabytes",
        "gb" => "gigabytes",
        "tb" => "terabytes",
        _ => {
            println!("Invalid unit: {}", size_and_unit[1]);
            return;
        }
    };

    // convert size to bytes
    let size_in_bytes: u64 = match unit {
        "bytes" => size,
        "kilobytes" => size * 1000,
        "megabytes" => size * 1000 * 1000,
        "gigabytes" => size * 1000 * 1000 * 1000,
        "terabytes" => size * 1000 * 1000 * 1000 * 1000,
        _ => {
            println!("Invalid unit: {}", unit);
            return;
        }
    };

    println!("Bytes: {}", size_in_bytes);
    println!("Kilobytes: {}", size_in_bytes as f64 / 1024.0);
    println!("Megabytes: {}", size_in_bytes as f64 / (1024.0 * 1024.0));
    println!("Gigabytes: {}", size_in_bytes as f64 / (1024.0 * 1024.0 * 1024.0));
    println!("Terabytes: {}", size_in_bytes as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0));

    // Declare first variable binding with name "shadow_num"
    let shadow_num = 5;

    // Declare second variable binding, shadows existing variable "shadow_num" 
    let shadow_num = shadow_num + 5; 

    // Declare third variable binding, shadows second binding of variable "shadow_num"
    let shadow_num = shadow_num * 2; 

    println!("The number is {}.", shadow_num);
}
