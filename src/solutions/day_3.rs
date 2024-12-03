use regex::Regex;

pub fn solve_first(input: &str) -> u32 {
    // Regex to find and capture muls in the input
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;
    for line in input.split("\n") {
        for (_, [first, second]) in regex.captures_iter(line).map(|c| c.extract()) {
            // We know the format is correct from the previous regex
            sum += first.parse::<u32>().unwrap() * second.parse::<u32>().unwrap();
        }
    }

    sum
}

pub fn solve_second(input: &str) -> u32 {
    // Regex to find and capture functions in the input
    let regex = Regex::new(r"(do\(\))|(don't\(\))|(mul\(\d{1,3},\d{1,3}\))").unwrap();
    // Regex to capture the numbers in a mul function
    let mul_regex = Regex::new(r"(\d{1,3}),(\d{1,3})").unwrap();

    let mut sum = 0;
    let mut multiplying = true;
    for line in input.split("\n") {
        for (_, [function]) in regex.captures_iter(line).map(|c| c.extract()) {
            if &function[0..3] == "mul" {
                if multiplying {
                    // We know the format is correct from previous regex
                    let (_, [first, second]) = mul_regex.captures(function).unwrap().extract();
                    sum += first.parse::<u32>().unwrap() * second.parse::<u32>().unwrap();
                }
            } else {
                // do() or don't()
                multiplying = function == "do()";
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_solution() {
        assert_eq!(solve_first("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 161);
    }

    #[test]
    fn second_solution() {
        assert_eq!(solve_second("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"), 48);
    }
}
