fn parse_lists_sorted(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut lists = (Vec::new(), Vec::new());

    for line in input.split("\n") {
        // Most input files have a trailing empty line
        if line == "" {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();

        lists.0.push(parts[0].parse()
            .expect("Failed to parse first ID from input"));
        lists.1.push(parts[1].parse()
            .expect("Failed to parse first ID from input"));
    }

    lists.0.sort();
    lists.1.sort();

    lists
}

pub fn solve_first(input: &str) -> u32 {
    let lists = parse_lists_sorted(input);

    // Simply sum the distances between the ith smallest numbers
    let mut dist_sum = 0;
    for i in 0..lists.0.len() {
        dist_sum += (lists.0[i] - lists.1[i]).abs() as u32;
    }

    dist_sum
}

pub fn solve_second(input: &str) -> i32 {
    let lists = parse_lists_sorted(input);

    // For each number in the left, similarity is its value times appearances in right
    // We avoid recalculating similarity unless the number changes
    let mut current_number = 0;
    let mut current_similarity = 0;
    let mut total_similarity = 0;
    for i in 0..lists.0.len() {
        if lists.0[i] != current_number {
            current_number = lists.0[i];
            current_similarity = 0;

            // The partition point gives us the start of appearances, and the list is
            //   sorted so we can loop to find last appearance and calc similarity
            let mut possible_pos = lists.1.partition_point(|&x| x < current_number);
            while possible_pos < lists.1.len() && lists.1[possible_pos] == current_number {
                current_similarity += current_number;
                possible_pos += 1;
            }
        }

        total_similarity += current_similarity;
    }

    total_similarity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_solution() {
        assert_eq!(solve_first("3 4\n4 3\n2 5\n1 3\n3 9\n3 3"), 11);
    }

    #[test]
    fn second_solution() {
        assert_eq!(solve_second("3 4\n4 3\n2 5\n1 3\n3 9\n3 3"), 31);
    }
}
