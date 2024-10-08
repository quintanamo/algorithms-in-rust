use std::env;
use std::fs;
mod sorting;

fn main() {
    // get algorithm and file names from command line args
    let args: Vec<String> = env::args().collect();
    let algorithm: &String = &args[1];
    let input_file_path: &String = &args[2];
    let output_file_path: &String = &args[3];

    println!("Selected algorithm: {algorithm}");
    println!("Input file path: {input_file_path}");

    // get the input file's contents and convert to a vector of numbers
    // input file's max number is 10,000 (14 bits), so we use i32
    let contents: String = fs::read_to_string(input_file_path)
        .expect("File not specified.");
    let numbers: Vec<i32> = contents.split_whitespace().map(|s: &str| s.parse().unwrap()).collect();
    let numbers_len: usize = numbers.len();

    let result: Vec<i32>;
    match algorithm.as_str() {
        "insertion-sort" => result = sorting::insertion_sort(numbers),
        "merge-sort" => result = sorting::merge_sort(numbers, 0, numbers_len - 1),
        "heap-sort" => result = sorting::heap_sort(numbers),
        _ => result = Vec::new()
    }

    if !result.is_empty() {
        let result_as_string = result.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join(" ");
        fs::write(output_file_path, result_as_string).expect("Unable to write output to file.");
    }
}
