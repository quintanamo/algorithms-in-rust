use std::env;
use std::fs;

fn main() {
    // get algorithm and file name from command line args
    let args: Vec<String> = env::args().collect();
    let algorithm: &String = &args[1];
    let input_file_path: &String = &args[2];

    println!("Selected algorithm: {algorithm}");
    println!("Input file path: {input_file_path}");

    // get the input file's contents and convert to a vector of numbers
    // input file's max number is 10,000 (14 bits), so we use u16
    let contents: String = fs::read_to_string(input_file_path)
        .expect("File not specified.");
    let numbers: Vec<u16> = contents.split_whitespace().map(|s: &str| s.parse().unwrap()).collect();

    println!("Numbers: {numbers:?}");
}
