use std::io;
use regex::Regex;

struct Robot {
    px: usize,
    py: usize,
    vx: usize,
    vy: usize,
}

struct Grid {
    width: usize,
    height: usize,
    robots: Vec<Robot>,
    cells: Vec<Vec<u32>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            robots: Vec::new(),
            cells: vec![vec![0; width]; height],
        }
    }

    fn parse_robots(&mut self, input: &str) {
        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

        for (_, [px, py, vx, vy]) in re.captures_iter(input).map(|c| c.extract()) {
            let px = px.parse().unwrap();
            let py = py.parse().unwrap();
            let mut vx: i32 = vx.parse().unwrap();
            let mut vy: i32 = vy.parse().unwrap();
            if vx < 0 { vx = vx % self.width as i32 + self.width as i32 };
            if vy < 0 { vy = vy % self.height as i32 + self.height as i32 };

            self.robots.push(Robot {
                px, py,
                vx: vx as usize,
                vy: vy as usize,
            });

            self.cells[py][px] += 1;
        }
    }

    fn iterate_once(&mut self) {
        for robot in self.robots.iter_mut() {
            let (opx, opy) = (robot.px, robot.py);
            robot.px = (robot.px + robot.vx) % self.width;
            robot.py = (robot.py + robot.vy) % self.height;

            self.cells[opy][opx] -= 1;
            self.cells[robot.py][robot.px] += 1;
        }
    }

    fn iterate_robots(&mut self, seconds: usize) {
        for robot in self.robots.iter_mut() {
            let (opx, opy) = (robot.px, robot.py);
            robot.px = (robot.px + robot.vx * seconds) % self.width;
            robot.py = (robot.py + robot.vy * seconds) % self.height;

            self.cells[opy][opx] -= 1;
            self.cells[robot.py][robot.px] += 1;
        }
    }

    fn count_robots_in_quadrants(&self) -> [u32; 4] {
        let midx = self.width / 2;
        let midy = self.height / 2;
        let mut quadrants = [0, 0, 0, 0];

        for robot in self.robots.iter() {
            if robot.px > midx && robot.py < midy {
                quadrants[0] += 1;
            } else if robot.px < midx && robot.py < midy {
                quadrants[1] += 1;
            } else if robot.px < midx && robot.py > midy {
                quadrants[2] += 1;
            } else if robot.px > midx && robot.py > midy {
                quadrants[3] += 1;
            }
        }

        quadrants
    }

    fn connected_inner_cells(&self) -> usize {
        self.cells.iter()
            .map(|r| r.windows(3)
                .filter(|w| w[1] > 0 && (w[0] > 0 || w[2] > 0))
                .count())
            .sum()
    }

    fn print_cells(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                let num_robots = self.cells[r][c];
                if num_robots == 0 {
                    print!(".");
                } else if num_robots < 10 {
                    print!("{num_robots}");
                } else {
                    print!("X");
                }
            }
            println!();
        }
    }
}

pub fn solve_first(input: &str) -> u32 {
    let mut grid = Grid::new(101, 103);
    grid.parse_robots(input);

    grid.iterate_robots(100);
    grid.count_robots_in_quadrants().iter().product()
}

pub fn solve_second(input: &str) -> i32 {
    let mut grid = Grid::new(101, 103);
    let grid_size = grid.width * grid.height;
    grid.parse_robots(input);

    let mut input_string = String::new();
    let stdin = io::stdin();

    // I originally assumed that the number of seconds would be much larger and had much tighter
    // bounds. It turns out that not many seconds are needed, and iterating once works
    let connected_limit = (grid_size as f64 * 0.01) as usize;
    println!("Enter y when the christmas tree appears");
    for s in 0..10_000 {
        if grid.connected_inner_cells() > connected_limit {
            println!("Seconds passed: {s}");
            grid.print_cells();

            stdin.read_line(&mut input_string).expect("Failed to read input");
            if input_string == "y\n" {
                return s;
            }
            input_string.clear();
        }

        grid.iterate_once();
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn first_solution() {
        let mut grid = Grid::new(11, 7);
        grid.parse_robots(TEST_INPUT);
        grid.iterate_robots(100);
        assert_eq!(grid.count_robots_in_quadrants().iter().product::<u32>(), 12);
    }
}
