use dotenvy::dotenv;
use std::env;

fn main() {
    // load .env file
    dotenv().expect(".env file not found.");
    // insert a environment variable
    let string_val: String = env::var("STRING").expect("KEY not found in .env file.");
    let integer_val: i32 = env::var("INTEGER")
        .expect("KEY not found in .env file.")
        .parse::<i32>()
        .expect("Error parsing INTEGER");
    let float_val: f64 = env::var("FLOAT")
        .expect("KEY not found in .env file.")
        .parse::<f64>()
        .expect("Error parsing FLOAT");
    let boolean_val: bool = env::var("BOOLEAN")
        .expect("KEY not found in .env file.")
        .parse::<bool>()
        .expect("Error parsing BOOLEAN");
    println!("STRING: {}", string_val);
    println!("INTEGER: {}", integer_val);
    println!("FLOAT: {}", float_val);
    println!("BOOLEAN: {}", boolean_val);
    //insert enironment variables
    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }
}
