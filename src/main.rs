// TODO:
// 1. Get commandline args
// 2. Use a switch statement
// 3. Use a logger
// 4. Add colours

use std::fs;

mod solutions;

fn main() {
    let file_path = "input/day_1.txt";
    let input = read_input(file_path);

    // This can take ownership of the input because we don't need it again
    // Also, we can assume that a solution might need to own the input
    solutions::day_1::solve_first(input);
}

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("Failed to read input file")
}
