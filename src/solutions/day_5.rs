use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

type Rules = HashMap<u32, Vec<u32>>;
type Update = Vec<u32>;
type Set = HashSet<u32>;

fn parse_ordering_rules(input: &str) -> Rules {
    let mut ordering_rules = HashMap::new();

    for line in input.split("\n") {
        // A|B means B relies on A, i.e. value before key in this case
        let mut parts = line.split("|").map(|p| p.parse().unwrap());
        let (constraint, page) = (parts.next().unwrap(), parts.next().unwrap());

        let constraints = ordering_rules.entry(page).or_insert(Vec::new());
        (*constraints).push(constraint);
    }

    ordering_rules
}

fn parse_updates(input: &str) -> Vec<Update> {
    input.split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| l.split(",")
            .map(|p| p.parse().unwrap())
            .collect())
        .collect()
}

fn valid_placement(ordering: &Rules, update: &Update, set: &Set, i: usize) -> bool {
    if let Some(constraints) = ordering.get(&update[i]) {
        for previous_page in constraints {
            // If the constraint is not satisfied, the placement is invalid
            if set.contains(&previous_page) &&
               update[..i].iter().position(|p| p == previous_page).is_none() {
                return false;
            }
        }
    }

    return true;
}

fn parse_updates_split(ordering: &Rules, input: &str) -> (Vec<Update>, Vec<Update>) {
    let updates = parse_updates(input);
    let (mut valid_updates, mut invalid_updates) = (Vec::new(), Vec::new());

    'outer: for update in updates {
        let update_set: Set = HashSet::from_iter(update.iter().cloned());

        for i in 0..update.len() {
            if !valid_placement(ordering, &update, &update_set, i) {
                invalid_updates.push(update);
                continue 'outer;
            }
        }

        valid_updates.push(update);
    }

    (valid_updates, invalid_updates)
}

pub fn solve_first(input: &str) -> u32 {
    let split_point = input.find("\n\n").unwrap();
    let ordering_rules = parse_ordering_rules(&input[..split_point]);
    let (valid_updates, _) = parse_updates_split(&ordering_rules, &input[split_point+2..]);

    let mut middle_sum = 0;
    for update in valid_updates {
        middle_sum += update[update.len() / 2];
    }
    middle_sum
}

pub fn solve_second(input: &str) -> u32 {
    let split_point = input.find("\n\n").unwrap();
    let ordering_rules = parse_ordering_rules(&input[..split_point]);
    let (_, invalid_updates) = parse_updates_split(&ordering_rules, &input[split_point+2..]);

    let mut middle_sum = 0;
    for mut update in invalid_updates {
        let update_set: Set = HashSet::from_iter(update.iter().cloned());
        let mut i = 0;
        let mut j = 1;

        // This will reach a valid ordering, guaranteed by the problem
        while i < update.len() {
            if valid_placement(&ordering_rules, &update, &update_set, i) {
                // The ordering so far works, move on and reset j
                i += 1;
                j = 1;
            } else {
                // The ordering does not work, try a new element to add
                let t = update[i];
                update[i] = update[i + j];
                update[i + j] = t;
                // Swap with the next element if the current one also fails
                j += 1;
            }
        }

        middle_sum += update[update.len() / 2];
    }
    middle_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn first_solution() {
        assert_eq!(solve_first(TEST_INPUT), 143);
    }

    #[test]
    fn second_solution() {
        assert_eq!(solve_second(TEST_INPUT), 123);
    }
}
