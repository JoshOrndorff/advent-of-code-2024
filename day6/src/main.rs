use std::{collections::HashSet, hash::Hash};

type Map = Vec<Vec<char>>;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
use Direction::*;

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");

    let map: Map = input.lines().map(|line| line.chars().collect()).collect();
    let height = map.len();
    let width = map[0].len();

    // Figure out where we're starting
    let mut start_x: usize = 0;
    let mut start_y: usize = 0;
    for row in 0..height {
        for col in 0..width {
            if map[row][col] == '^' {
                start_y = row;
                start_x = col;
            }
        }
    }
    // Make them no longer mutable.
    // This uses the type system to ensure that we never accidentally overwrite the original starting location.
    let start_x = start_x;
    let start_y = start_y;

    // Solve part 1
    let part_1 = how_many_steps_until_guard_wanders_off_map(start_x, start_y, &map)
        .expect("Guard should wander off map from original starting location.");
    println!("Part 1: {} steps before wandering off the map.", part_1);

    // Solve part 2
    let mut number_of_locations_where_an_additional_obstacle_causes_a_loop = 0;
    for row in 0..height {
        for col in 0..width {
            // If the space in question was originally free, try putting an obstacle there and see if it creates a loop.
            if map[row][col] == '.' {
                println!("Adding obstacle at {row}, {col}");
                let mut modified_map = map.clone();
                modified_map[row][col] = '#'; // Sneaky elves
                if how_many_steps_until_guard_wanders_off_map(start_x, start_y, &modified_map)
                    .is_none()
                {
                    number_of_locations_where_an_additional_obstacle_causes_a_loop += 1;
                }
            }
        }
    }

    println!(
        "There are {} locations where a single additional obstacle would cause a loop.",
        number_of_locations_where_an_additional_obstacle_causes_a_loop
    );
}

fn how_many_steps_until_guard_wanders_off_map(
    start_x: usize,
    start_y: usize,
    map: &Map,
) -> Option<usize> {
    // Keep track of the places we've visited and the direction we were facing when we were there.
    // Useful to detect loops and count steps before wandering off.
    // So far only the starting position.
    let mut visited = HashSet::from([(start_x, start_y, Up)]);

    // Declare local mutable state tracking variables.
    let mut x = start_x;
    let mut y = start_y;
    let mut facing = Up;

    // Now just run the algorithm as described
    loop {
        let (nextx, nexty) = match facing {
            // just let it wrap around here. index max_value will be out of bounds just like 0 would have been.
            Up => (x, y - 1),
            Right => (x + 1, y),
            Down => (x, y + 1),
            Left => (x - 1, y),
        };

        if let Some(row) = map.get(nexty) {
            if let Some(c) = row.get(nextx) {
                if c == &'#' {
                    facing = match facing {
                        Up => Right,
                        Right => Down,
                        Down => Left,
                        Left => Up,
                    }
                } else {
                    x = nextx;
                    y = nexty;
                }
            } else {
                break;
            }
        } else {
            break;
        }

        // Check if we are in a cycle
        if visited.contains(&(x, y, facing)) {
            return None;
        }

        visited.insert((x, y, facing));
    }

    println!("gonna return; no cycle.");

    // Calculate the unique number of locations by using a temporary hashset (only needed for part 1)
    Some(
        visited
            .iter()
            .map(|(x, y, _)| (x, y))
            .collect::<HashSet<_>>()
            .len(),
    )
}
