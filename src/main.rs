use clap::Parser;
use clap::builder::TypedValueParser;
use std::path::PathBuf;

mod solutions;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// The day to solve
    #[arg(
        short,
        long,
        default_value_t = 1,
        value_parser = clap::builder::PossibleValuesParser::new(["1", "2", "3", "5", "7"])
            .map(|s| s.parse::<u8>().unwrap()),
    )]
    day: u8,

    /// The path to the input file
    #[arg(short, long, default_value = "input/day_1.txt")]
    file_path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let input = read_input(args.file_path);
    let (first, second) = match args.day {
        1 => (solutions::day_1::solve_first(&input).to_string(),
              solutions::day_1::solve_second(&input).to_string()),
        2 => (solutions::day_2::solve_first(&input).to_string(),
              solutions::day_2::solve_second(&input).to_string()),
        3 => (solutions::day_3::solve_first(&input).to_string(),
              solutions::day_3::solve_second(&input).to_string()),
        5 => (solutions::day_5::solve_first(&input).to_string(),
              solutions::day_5::solve_second(&input).to_string()),
        7 => (solutions::day_7::solve_first(&input).to_string(),
              solutions::day_7::solve_second(&input).to_string()),
        _ => panic!("Unsolved day {} allowed by arg parser", args.day),
    };

    println!("First solution: {first}");
    println!("Second solution: {second}");
}

fn read_input(file_path: PathBuf) -> String {
    std::fs::read_to_string(file_path)
        .expect("Failed to read input file")
}
