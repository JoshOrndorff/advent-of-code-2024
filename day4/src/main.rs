const TARGET: &'static [char] = &['X', 'M', 'A', 'S'];
const TARGET_REVERSE: &'static [char] = &['S', 'A', 'M', 'X'];

fn main() {
    //HOW the fuck am I supposed to reverse this?
    // let mut target_reverse: &mut[char] = &mut *TARGET;
    // target_reverse.reverse();
    // let target_reverse = target_reverse; // No longer mutable

    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");

    let word_search: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = word_search.len();
    let height = word_search[0].len();

    println! {"The grid is {width} X {height}"};

    // Loop through the word search looking for forward and backward occurrences
    let mut forward_counter = 0;
    let mut backward_counter = 0;
    for row in 0..height {
        for start_col in 0..width - TARGET.len() + 1 {
            let sample = &word_search[row][start_col..start_col + TARGET.len()];
            if sample == TARGET {
                forward_counter += 1;
            }
            if sample == TARGET_REVERSE {
                backward_counter += 1;
            }
        }
    }

    println!("forward: {forward_counter}");
    println!("backward: {backward_counter}");

    // Loop through again looking for upward and downward occurrences
    let mut up_counter = 0;
    let mut down_counter = 0;
    for start_row in 0..height - TARGET.len() + 1 {
        for col in 0..width {
            let sample = [
                word_search[start_row][col],
                word_search[start_row + 1][col],
                word_search[start_row + 2][col],
                word_search[start_row + 3][col],
            ];
            if sample == TARGET {
                down_counter += 1;
            }
            if sample == TARGET_REVERSE {
                up_counter += 1;
            }
        }
    }

    println!("up: {up_counter}");
    println!("down: {down_counter}");

    // Downhill diags
    let mut downhill_forward_counter = 0;
    let mut downhill_backward_counter = 0;
    for start_row in 0..height - TARGET.len() + 1 {
        for start_col in 0..width - TARGET.len() + 1 {
            let sample = [
                word_search[start_row][start_col],
                word_search[start_row + 1][start_col + 1],
                word_search[start_row + 2][start_col + 2],
                word_search[start_row + 3][start_col + 3],
            ];
            if sample == TARGET {
                downhill_forward_counter += 1;
            }
            if sample == TARGET_REVERSE {
                downhill_backward_counter += 1;
            }
        }
    }

    println!("downhill forward: {downhill_forward_counter}");
    println!("downhill backward: {downhill_backward_counter}");

    // Uphill diags
    let mut uphill_forward_counter = 0;
    let mut uphill_backward_counter = 0;
    for start_row in TARGET.len() - 1..height {
        for start_col in 0..width - TARGET.len() + 1 {
            let sample = [
                word_search[start_row][start_col],
                word_search[start_row - 1][start_col + 1],
                word_search[start_row - 2][start_col + 2],
                word_search[start_row - 3][start_col + 3],
            ];
            if sample == TARGET {
                uphill_forward_counter += 1;
            }
            if sample == TARGET_REVERSE {
                uphill_backward_counter += 1;
            }
        }
    }

    println!("uphill forward: {uphill_forward_counter}");
    println!("uphill backward: {uphill_backward_counter}");

    // Part 1 results
    let total = forward_counter
        + backward_counter
        + up_counter
        + down_counter
        + downhill_forward_counter
        + downhill_backward_counter
        + uphill_forward_counter
        + uphill_backward_counter;
    println!("Total XMAS occurrences: {total}");

    // Part 2

    let mut xmas_counter = 0;
    for row in 1..width - 1 {
        for col in 1..height - 1 {
            if word_search[row][col] == 'A' {
                let up_left = word_search[row - 1][col - 1];
                let up_right = word_search[row - 1][col + 1];
                let down_left = word_search[row + 1][col - 1];
                let down_right = word_search[row + 1][col + 1];

                match (up_left, down_right, up_right, down_left) {
                    ('M', 'S', 'M', 'S') => xmas_counter += 1,
                    ('M', 'S', 'S', 'M') => xmas_counter += 1,
                    ('S', 'M', 'M', 'S') => xmas_counter += 1,
                    ('S', 'M', 'S', 'M') => xmas_counter += 1,
                    _ => (),
                }
            }
        }
    }
    println!("Total X-MAS occurrences: {xmas_counter}");
}
