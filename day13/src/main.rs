// part 2: 77779103984288 is too high

use sscanf::sscanf;

struct Machine {
    ax: isize,
    ay: isize,
    bx: isize,
    by: isize,
    px: isize,
    py: isize,
}

impl Machine {
    fn from_string(s: &str) -> Self {
        let lines = s.lines().collect::<Vec<_>>();
        let (ax, ay) = sscanf!(lines[0], "Button A: X+{}, Y+{}", isize, isize,)
            .expect("machine input line 1 should parse");
        let (bx, by) = sscanf!(lines[1], "Button B: X+{}, Y+{}", isize, isize,)
            .expect("machine input line 2 should parse");
        let (px, py) = sscanf!(lines[2], "Prize: X={}, Y={}", isize, isize,)
            .expect("machine input line 3 should parse");

        Self {
            ax,
            ay,
            bx,
            by,
            px,
            py,
        }
    }

    /// Calculate the cost to win this machine if it is indeed possible, or none if it is impossible.
    ///
    /// Our approach is based on matrix inversion. Each machine describes a system of equations with two variables and two unknowns.
    ///
    /// a * ax + b * bx = px
    /// a * ay + b * by = py
    ///
    /// Or as a matrix equation
    ///
    ///   +--+     +--+   +--+
    /// a |ax| + b |bx| = |px|
    ///   |ay|     |by|   |py|
    ///   +--+     +--+   +--+
    ///
    ///  +-----+ +-+   +--+
    ///  |ax bx| |a| = |px|
    ///  |ay by| |b|   |py|
    ///  +-----+ +-+   +--+
    ///
    /// This system can be solved with the method Mrs McDuffie taught us in 10th grade.
    /// where you expand matricies into more and more smaller matricies.
    ///
    /// But I did it first by substitution, so let's see if that works out.
    ///
    /// We know that b = (py * ax - px * ay) / (by * ax - bx * ay).
    /// Obviously the denominator needs to be non zero which means that ax * by != bx * ay
    /// If those are equal, then we can't reach the prize. This sounds familiar from calculating inverse matricies.
    /// If they are unequal we can assume the prize is reachable.
    ///
    /// Once we have b, then we calculate a as a = (px - b * bx) / ax
    fn cost_to_win(&self) -> Option<isize> {
        // Calculate for coefficients, and watch out for cases where the machine is not solvable
        let b_numerator = self.py * self.ax - self.px * self.ay;
        let b_denominator = self.by * self.ax - self.bx * self.ay;
        if b_denominator == 0 {
            return None;
        }
        if b_numerator % b_denominator != 0 {
            return None;
        }
        let b = b_numerator / b_denominator;
        let a = (self.px - b * self.bx) / self.ax;

        // Return the cost of pressing those buttons
        let cost = 3 * a + b;
        println!("Machine is winnable in {a} A presses and {b} B presses. with a cost of {cost}");
        Some(cost)
    }
}
fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");
    // let input = std::fs::read_to_string("./example.txt").expect("input file should exist");

    let part_1_cost = input
        .split("\n\n")
        .map(|l| Machine::from_string(l).cost_to_win().unwrap_or_default())
        .sum::<isize>();

    let part_2_cost = input
        .split("\n\n")
        .map(|l| {
            let mut m = Machine::from_string(l);
            m.px += 10000000000000;
            m.py += 10000000000000;
            m.cost_to_win().unwrap_or_default()
        })
        .sum::<isize>();

    println! {"Total cost to win all possible prizes (part 1): {part_1_cost}"};
    println! {"Total cost to win all possible prizes (part 2): {part_2_cost}"};
}
