use regex::Regex;

// This is used as a limit for how different two f64s can be
const EPSILON: f64 = 0.00001;

fn is_integer(float: f64) -> bool {
    (float - float.round()).abs() < EPSILON
}

struct ClawMachine {
    prize_x: f64,
    prize_y: f64,
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
}

impl ClawMachine {
    fn parse(px: &str, py: &str, ax: &str, ay: &str, bx: &str, by: &str) -> Self {
        ClawMachine {
            prize_x: px.parse().unwrap(),
            prize_y: py.parse().unwrap(),
            ax: ax.parse().unwrap(),
            ay: ay.parse().unwrap(),
            bx: bx.parse().unwrap(),
            by: by.parse().unwrap(),
        }
    }

    fn solve_for_presses(&self) -> Option<(u32, u32)> {
        let alpha = self.prize_y - self.ay * self.prize_x / self.ax;
        let beta = self.by - self.ay * self.bx / self.ax;

        let b_presses = alpha / beta;
        let a_presses = (self.prize_x - b_presses * self.bx) / self.ax;

        if !is_integer(a_presses) || !is_integer(b_presses) {
            return None;
        }

        // NOTE: This was a nasty error (without the `.round()`)
        Some((a_presses.round() as u32, b_presses.round() as u32))
    }

    fn adjust_for_error(&mut self) {
        self.prize_x += 10000000000000.0;
        self.prize_y += 10000000000000.0;
    }
}

pub fn solve_first(input: &str) -> u32 {
    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)")
        .unwrap();

    let mut ticket_sum = 0;
    for (_, [ax, ay, bx, by, prize_x, prize_y]) in re.captures_iter(input).map(|c| c.extract()) {
        let claw_machine = ClawMachine::parse(prize_x, prize_y, ax, ay, bx, by);

        if let Some((a_presses, b_presses)) = claw_machine.solve_for_presses() {
            ticket_sum += 3 * a_presses + b_presses;
        }
    }
    ticket_sum
}

pub fn solve_second(input: &str) -> u64 {
    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)")
        .unwrap();

    let mut ticket_sum = 0;
    for (_, [ax, ay, bx, by, prize_x, prize_y]) in re.captures_iter(input).map(|c| c.extract()) {
        let mut claw_machine = ClawMachine::parse(prize_x, prize_y, ax, ay, bx, by);
        claw_machine.adjust_for_error();

        if let Some((a_presses, b_presses)) = claw_machine.solve_for_presses() {
            ticket_sum += 3 * a_presses as u64 + b_presses as u64;
        }
    }
    ticket_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn first_solution() {
        assert_eq!(solve_first(TEST_INPUT), 480);
    }
}
