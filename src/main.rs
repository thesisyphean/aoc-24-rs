use std::fs;

mod solutions;

fn main() {
    let file_path = "input/day_5.txt";
    let input = read_input(file_path);

    println!("First Solution: {}", solutions::day_5::solve_first(&input));
    println!("Second Solution: {}", solutions::day_5::solve_second(&input));
}

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("Failed to read input file")
}
