// A fast approximation of a base 10 logarithm, floored
fn fast_log10(mut n: u64) -> u32 {
    let mut p = 0;

    while n >= 10 {
        n /= 10;
        p += 1;
    }

    p
}

struct Equation {
    test_value: u64,
    // This stores the numbers in the opposite order to that given
    numbers: Vec<u64>,
    can_concat: bool,
}

impl Equation {
    fn parse(line: &str) -> Self {
        let parts: Vec<_> = line.split(": ").collect();
        // TODO: Is there a way to parse and reverse in one step?
        let numbers: Vec<_> = parts[1].split(" ")
            .map(|n| n.parse().unwrap())
            .collect();

        Equation {
            test_value: parts[0].parse().unwrap(),
            numbers: numbers.into_iter().rev().collect(),
            can_concat: false,
        }
    }

    fn possible_by_add(&mut self, first_number: u64, last: usize) -> bool {
        self.numbers[last] += first_number;
        let possible = self.compute_possibility();
        self.numbers[last] -= first_number;
        possible
    }

    fn possible_by_mul(&mut self, first_number: u64, last: usize) -> bool {
        self.numbers[last] *= first_number;
        let possible = self.compute_possibility();
        self.numbers[last] /= first_number;
        possible
    }

    fn possible_by_con(&mut self, first_number: u64, last: usize) -> bool {
        let concat_diff = first_number * 10u64.pow(fast_log10(self.numbers[last]) + 1);
        self.numbers[last] += concat_diff;
        let possible = self.compute_possibility();
        self.numbers[last] -= concat_diff;
        possible
    }

    // Recursively check for possibility by trying different operators
    fn compute_possibility(&mut self) -> bool {
        // Not necessary with perfect input, but good practice
        if self.numbers.len() == 0 {
            return false;
        }

        if self.numbers.len() == 1 {
            return self.numbers[0] == self.test_value;
        }

        let first_number = self.numbers.pop().unwrap();
        let last = self.numbers.len() - 1;

        // We can cut early as all of the operations lead to increasing values
        // (assuming non-negative input)
        if first_number > self.test_value {
            self.numbers.push(first_number);
            return false;
        }

        let mut possible = self.possible_by_add(first_number, last) ||
            self.possible_by_mul(first_number, last);

        if self.can_concat {
            possible |= self.possible_by_con(first_number, last);
        }

        self.numbers.push(first_number);

        possible
    }
}

fn parse_equations(input: &str, concat: bool) -> Vec<Equation> {
    let mut equations: Vec<_> = input.split("\n")
        .filter(|l| l.len() > 0)
        .map(Equation::parse)
        .collect();

    if concat {
        for equation in &mut equations {
            equation.can_concat = true;
        }
    }

    equations
}

pub fn solve_first(input: &str) -> u64 {
    let equations = parse_equations(input, false);

    let mut calibration_result = 0;
    for mut equation in equations {
        if equation.compute_possibility() {
            calibration_result += equation.test_value;
        }
    }
    calibration_result
}

pub fn solve_second(input: &str) -> u64 {
    // Can concatenate now
    let equations = parse_equations(input, true);

    let mut calibration_result = 0;
    for mut equation in equations {
        if equation.compute_possibility() {
            calibration_result += equation.test_value;
        }
    }
    calibration_result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn first_solution() {
        assert_eq!(solve_first(TEST_INPUT), 3749);
    }

    #[test]
    fn second_solution() {
        assert_eq!(solve_second(TEST_INPUT), 11387);
    }

    #[test]
    fn basic_concat() {
        let mut equation = Equation::parse("156: 15 6");
        equation.can_concat = true;
        assert_eq!(equation.compute_possibility(), true);
    }
}
