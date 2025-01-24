use std::collections::HashMap;
use std::cell::RefCell;

struct TowelSpecifications<'a> {
    towels: Vec<&'a str>,
    designs: Vec<&'a str>,
    cache: RefCell<HashMap<&'a str, usize>>,
}

impl<'a> TowelSpecifications<'a> {
    fn count_possible_designs(&self) -> usize {
        self.designs.iter()
            .filter(|&d| self.count_arrangements(d) > 0)
            .count()
    }

    fn count_design_arrangements(&self) -> usize {
        self.designs.iter()
            .map(|&d| self.count_arrangements(d))
            .sum()
    }

    fn count_arrangements(&self, design: &'a str) -> usize {
        if let Some(&count) = self.cache.borrow().get(design) {
            return count;
        }

        if design.is_empty() {
            return 1;
        }

        let mut ways = 0;
        for &towel in self.towels.iter() {
            if towel.len() <= design.len() && towel == &design[..towel.len()] {
                ways += self.count_arrangements(&design[towel.len()..]);
            }
        }

        self.cache.borrow_mut().insert(design, ways);

        ways
    }
}

impl<'a> From<&'a str> for TowelSpecifications<'a> {
    fn from(input: &'a str) -> Self {
        let parts: Vec<_> = input
            .split("\n\n")
            .map(|p| p.trim())
            .collect();

        TowelSpecifications {
            towels: parts[0].split(", ").collect(),
            designs: parts[1].split_whitespace().collect(),
            cache: RefCell::new(HashMap::new()),
        }
    }
}

pub fn solve_first(input: &str) -> usize {
    let specifications = TowelSpecifications::from(input);
    specifications.count_possible_designs()
}

pub fn solve_second(input: &str) -> usize {
    let specifications = TowelSpecifications::from(input);
    specifications.count_design_arrangements()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn first_solution() {
        assert_eq!(solve_first(TEST_INPUT), 6);
    }

    #[test]
    fn second_solution() {
        assert_eq!(solve_second(TEST_INPUT), 16);
    }
}
