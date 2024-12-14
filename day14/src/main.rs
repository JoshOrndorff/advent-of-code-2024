/// The key idea here is that there mare a lot of separable bits of motion.
/// Most importantly, the motion of each individual guard is independent from every other guard.
/// Second, the horizontal and vertical components of motion are independent even for a single guard.
///
/// The next idea is that there will be cycles. In the worst case the motion will repeat after WIDTH
/// steps for horizontal motion and HEIGHT steps for vertical. In general, it is possible that motion
/// will repeat in fewer steps. For example if the step size is half the width, then motion would cycle
/// after only two steps.
/// HOWEVER, both the example board (7X11) and the real board (101X103) have prime dimensions. This
/// guarantees that we will always be in the longest-possible-cycle case.
// Man, I really thought part two was gonna say run it for a billion seconds.
// But it did not. Maybe I should have just simulated a hundred steps instead of
// doing all the modular math and separating the independent parts.
use std::collections::HashSet;

use sscanf::sscanf;

/// A single bathroom guard with its position and velocity.
struct Guard {
    row: isize,
    col: isize,
    v_y: isize,
    v_x: isize,
}

impl Guard {
    /// Create a new guard from a string slice (a line of input most likely)
    fn from_string(s: &str) -> Self {
        let (x, y, vx, vy) = sscanf!(s, "p={},{} v={},{}", isize, isize, isize, isize)
            .expect("guard strings should parse");
        Self {
            row: y,
            col: x,
            v_y: vy,
            v_x: vx,
        }
    }

    /// Calculate the position of the guard after n seconds.
    ///
    /// The algorithm calculates the horizontal and vertical components independently
    /// and takes advantage of cycles.
    fn position_after(&self, n: isize, width: isize, height: isize) -> (isize, isize) {
        let final_col = modulus(self.col + n * self.v_x, width);
        let final_row = modulus(self.row + n * self.v_y, height);
        (final_row, final_col)
    }

    /// Calculates which quadrant the guard is in after n seconds.
    /// I will adopt the convention that Q0 is the top right, Q1, is the top left and so on counterclockwise.
    /// The final result is independent of this convention.
    fn quadrant_after(&self, n: isize, width: isize, height: isize) -> Option<usize> {
        let horizontal_midpoint = width / 2;
        let vertical_midpoint = height / 2;
        let (row, col) = self.position_after(n, width, height);

        if row < vertical_midpoint && col > horizontal_midpoint {
            Some(0)
        } else if row < vertical_midpoint && col < horizontal_midpoint {
            Some(1)
        } else if row > vertical_midpoint && col < horizontal_midpoint {
            Some(2)
        } else if row > vertical_midpoint && col > horizontal_midpoint {
            Some(3)
        } else {
            println!("Guard at position ({row}, {col})is not in any quadrant.");
            None
        }
    }
}

/// TIL That the remainder and modulus are not the same for negative numbers.
/// https://www.reddit.com/r/rust/comments/r1rmv5/rust_says_207_6_when_it_is_1/
fn modulus(n: isize, base: isize) -> isize {
    let remainder = n % base;
    (remainder + base) % base
}

fn main() {
    let (input, width, height) = (
        std::fs::read_to_string("./input.txt").expect("input file should exist"),
        101isize,
        103isize,
    );
    // let (input, width, height) = (
    //     std::fs::read_to_string("./example.txt").expect("input file should exist"),
    //     11isize,
    //     7isize,
    // );

    let guards = input.lines().map(|l| Guard::from_string(l));

    // Part 1
    let quadrants = guards
        .clone()
        .filter_map(|g| g.quadrant_after(100, width, height));
    let mut quadrant_count: Vec<usize> = Vec::from([0, 0, 0, 0]);
    for q in quadrants {
        quadrant_count[q] += 1;
    }

    let safety_factor = quadrant_count.iter().product::<usize>();
    println!("Safety factor is {safety_factor}");

    // Print the example board after 100 steps to make sure our print_board
    // function is working properly. It seems it is working properly because it
    // matches the example output stated in the problem.
    let positions = guards
        .clone()
        .clone()
        .map(|g| g.position_after(100, width, height))
        .collect::<HashSet<_>>();
    print_board(&positions, width, height);

    // Part 2 - There can be at most 101*103 unique board configs. I'll
    // print them all out (to a file) and look for the tree in my editor.
    for n in 0..(101 * 103) {
        println!("After {n} steps (press enter to continue):");
        let guard_positions = guards
            .clone()
            .map(|g| g.position_after(n, width, height))
            .collect::<HashSet<_>>();
        print_board(&guard_positions, width, height);

        // std::thread::sleep(std::time::Duration::from_millis(90));
    }
}

fn print_board(guard_positions: &HashSet<(isize, isize)>, width: isize, height: isize) {
    for row in 0..height {
        for col in 0..width {
            if guard_positions.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[test]
fn simple_move() {
    let g = Guard::from_string(&"p=0,4 v=3,-3");
    let (final_y, final_x) = g.position_after(1, 11, 7);

    assert_eq!(final_x, 3);
    assert_eq!(final_y, 1);
}

#[test]
fn wrap_off_top() {
    let g = Guard::from_string(&"p=0,4 v=3,-3");
    let (final_y, final_x) = g.position_after(2, 11, 7);

    assert_eq!(final_x, 6);
    assert_eq!(final_y, 5);
}

#[test]
fn wrap_off_top_and_right() {
    let g = Guard::from_string(&"p=0,4 v=3,-3");
    let (final_y, final_x) = g.position_after(4, 11, 7);

    assert_eq!(final_x, 1);
    assert_eq!(final_y, 6);
}

#[test]
fn g1_returns_to_start_after_width_times_height_seconds() {
    let g = Guard::from_string(&"p=0,4 v=3,-3");
    let (final_y, final_x) = g.position_after(77, 11, 7);

    assert_eq!(final_x, g.col);
    assert_eq!(final_y, g.row);
}

#[test]
fn g2_returns_to_start_after_width_times_height_seconds() {
    let g = Guard::from_string(&"p=6,3 v=-1,-3");
    let (final_y, final_x) = g.position_after(77, 11, 7);

    assert_eq!(final_x, g.col);
    assert_eq!(final_y, g.row);
}
