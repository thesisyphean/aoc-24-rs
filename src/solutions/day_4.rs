use regex::Regex;

// TODO: Generate the regexes automatically based on the word and the width
const FIRST_REGEXES: [&str; 8] = [
    "^XMAS",
    "^SAMX",
    "(?s)^X.{140}M.{140}A.{140}S",
    "(?s)^S.{140}A.{140}M.{140}X",
    "(?s)^X.{141}M.{141}A.{141}S",
    "(?s)^S.{141}A.{141}M.{141}X",
    "(?s)^X.{139}M.{139}A.{139}S",
    "(?s)^S.{139}A.{139}M.{139}X",
];

const SECOND_REGEXES: [&str; 4] = [
    "(?s)^M.S.{139}A.{139}M.S",
    "(?s)^M.M.{139}A.{139}S.S",
    "(?s)^S.M.{139}A.{139}S.M",
    "(?s)^S.S.{139}A.{139}M.M",
];

fn count_overlapping_matches(regexes: &[&str], input: &str) -> usize {
    let mut words = 0;

    let res: Vec<_> = regexes.iter()
        .map(|r| Regex::new(r).unwrap())
        .collect();

    for i in 0..input.len() {
        for regex in res.iter() {
            if regex.find(&input[i..]).is_some() {
                words += 1;
            }
        }
    }

    words
}

pub fn solve_first(input: &str) -> usize {
    count_overlapping_matches(&FIRST_REGEXES, input)
}

pub fn solve_second(input: &str) -> usize {
    count_overlapping_matches(&SECOND_REGEXES, input)
}

/* TODO: Include these when the regexes can be calculated automatically
#[cfg(test)]
mod tests {
    use super::*;

    // TODO: This test won't work with the different regexes
    const TEST_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn first_solution() {
        assert_eq!(solve_first(TEST_INPUT), 18);
    }

    #[test]
    fn second_solution() {
        assert_eq!(solve_second(TEST_INPUT), 9);
    }
}
*/
