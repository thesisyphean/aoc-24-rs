type Report = Vec<i32>;

fn parse_reports(input: &str) -> Vec<Report> {
    input.split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| l.split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect())
        .collect()
}

fn valid_level_change(level_a: i32, level_b: i32, dir: i32) -> bool {
    let change = level_a - level_b;
    let current_dir = change.signum();
    let diff = change.abs();

    // Direction stays the same and diff the right size
    current_dir == dir && diff >= 1 && diff <= 3
}

fn safe_report(report: &Report) -> bool {
    if report.len() < 2 {
        return true;
    }

    // These indices are fine because of the previous check
    let dir = (report[0] - report[1]).signum();

    for window in report.windows(2) {
        if !valid_level_change(window[0], window[1], dir) {
            return false;
        }
    }

    true
}

pub fn solve_first(input: &str) -> u32 {
    let reports = parse_reports(input);
    let mut safe_reports = 0;

    for report in reports {
        if safe_report(&report) {
            safe_reports += 1;
        }
    }

    safe_reports
}

pub fn solve_second(input: &str) -> u32 {
    let reports = parse_reports(input);
    let mut safe_reports = 0;

    'outer: for report in reports {
        if safe_report(&report) {
            safe_reports += 1;
            continue;
        }

        // Check if any deletion leads to a safe report
        for i in 0..report.len() {
            let new_report = [&report[0..i], &report[i+1..]].concat();
            if safe_report(&new_report) {
                safe_reports += 1;
                continue 'outer;
            }
        }
    }

    safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn first_solution() {
        assert_eq!(solve_first(TEST_INPUT), 2);
    }

    #[test]
    fn second_solution() {
        assert_eq!(solve_second(TEST_INPUT), 4);
    }
}
