struct Cell {
    value: char,
    counted: bool,
}

impl Cell {
    fn new(value: char) -> Self {
        Cell {
            value,
            counted: false,
        }
    }
}

struct Farm {
    grid: Vec<Vec<Cell>>,
}

impl Farm {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<_>> = input.split_whitespace()
            .map(|l| l.chars().map(Cell::new).collect())
            .collect();

        debug_assert!(!grid.is_empty());

        Farm { grid }
    }

    fn sum_priced_regions(&mut self) -> u32 {
        let mut price_sum = 0;

        for r in 0..self.grid.len() {
            for c in 0..self.grid[0].len() {
                let (area, perimeter) = self.price_check(r, c, self.grid[r][c].value);
                price_sum += area * perimeter;
            }
        }

        price_sum
    }

    fn price_check(&mut self, r: usize, c: usize, source: char) -> (u32, u32) {
        if self.grid[r][c].value != source {
            return (0, 1);
        }

        if self.grid[r][c].counted {
            return (0, 0);
        }

        self.grid[r][c].counted = true;
        
        let mut area = 1;
        let mut new_positions = Vec::new();
        if r > 0 { new_positions.push((r - 1, c)) };
        if c > 0 { new_positions.push((r, c - 1)) };
        if r < self.grid.len() - 1 { new_positions.push((r + 1, c)) };
        if c < self.grid[0].len() - 1 { new_positions.push((r, c + 1)) };
        let mut perimeter = 4 - new_positions.len() as u32;

        for position in new_positions {
            let (new_area, new_perimeter) = self.price_check(position.0, position.1, source);
            area += new_area;
            perimeter += new_perimeter;
        }

        (area, perimeter)
    }
}

pub fn solve_first(input: &str) -> u32 {
    let mut farm = Farm::new(input);
    farm.sum_priced_regions()
}

pub fn solve_second(_input: &str) -> i32 {
    // TODO
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
AAAA
BBCD
BBCC
EEEC
";

    #[test]
    fn first_solution() {
        assert_eq!(solve_first(TEST_INPUT), 140);
    }

    #[test]
    fn second_solution() {
        assert_eq!(solve_second(TEST_INPUT), -1);
    }
}
