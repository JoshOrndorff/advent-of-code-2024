// 989 is too low

use std::collections::{HashMap, HashSet};

type Coord = (i32, i32);

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let height = map.len();
    let width = map[0].len();

    // Two mappings to track the locations of antennas and antinodes. Each mapping is from the antenna type to a set of coordinates.
    // The antinode rules change for part two so the antinodes are tracked separately for each part.
    let mut antennas: HashMap<char, HashSet<Coord>> = HashMap::new();
    let mut antinodes1: HashMap<char, HashSet<Coord>> = HashMap::new();
    let mut antinodes2: HashMap<char, HashSet<Coord>> = HashMap::new();

    // Go through the entire map looking for antennas and when one is found:
    // Calculate and catalog antinodes formed with previously known antennas
    // Catalog the newly-found antenna itself
    for row in 0..height {
        for col in 0..width {
            let c = map[row][col];
            if c == '.' {
                continue;
            }

            println!("Found antenna of type {c} at {row},{col}");

            // When we discover a new antenna, we first iterate through all the previously-known
            // antennas with the same id, calculating the antinode locations formed.
            //
            // If this is the first antenna with its id that has been found, the following loop
            // executed zero times.
            // The calculations for the antinode coordinates do not depend which antenna is on top or bottom
            // or whether they are in a TopLeft-BottomRight or TopRight-BottomLeft configuration.
            // See how-to-calculate-antinode-locations for an exhaustive investigation of every possible case.
            let new_row = row as i32;
            let new_col = col as i32;
            for (old_row, old_col) in antennas.get(&c).unwrap_or(&HashSet::new()) {
                println!("  Existing antenna of type {c} at {old_row},{old_col}");

                // Part 1 rules
                let s = antinodes1.entry(c).or_default();
                s.extend(part_1_antinodes(
                    (*old_row, *old_col),
                    (new_row, new_col),
                    width,
                    height,
                ));

                // Part 2 rules
                let s = antinodes2.entry(c).or_default();
                s.extend(part_2_antinodes(
                    (*old_row, *old_col),
                    (new_row, new_col),
                    width,
                    height,
                ));
            }

            // Now we can insert the freshly discovered antenna into the antennas mapping.
            antennas
                .entry(c)
                .or_default()
                .insert((row as i32, col as i32));
        }
    }

    // Count the unique locations by part 1 rules
    let deduped_antinode_locations = antinodes1.values().flatten().collect::<HashSet<_>>().len();
    println!("Part 1: {deduped_antinode_locations}");

    // Count the unique locations by part 1 rules
    let deduped_antinode_locations = antinodes2.values().flatten().collect::<HashSet<_>>().len();
    println!("Part 2: {deduped_antinode_locations}");
}

fn part_1_antinodes(
    (old_row, old_col): Coord,
    (new_row, new_col): Coord,
    w: usize,
    h: usize,
) -> Vec<Coord> {
    let mut locations = Vec::new();

    // The antinode closer to the new antenna
    let antinode_new_row = 2 * new_row - old_row;
    let antinode_new_col = 2 * new_col - old_col;
    if in_map((antinode_new_row, antinode_new_col), w, h) {
        locations.push((antinode_new_row, antinode_new_col));
    }

    // The antinode closer to the old antenna
    let antinode_old_row = 2 * old_row - new_row;
    let antinode_old_col = 2 * old_col - new_col;
    if in_map((antinode_old_row, antinode_old_col), w, h) {
        locations.push((antinode_old_row, antinode_old_col));
    }

    locations
}

// The only difference is the loop really.
fn part_2_antinodes(
    (old_row, old_col): Coord,
    (new_row, new_col): Coord,
    w: usize,
    h: usize,
) -> Vec<Coord> {
    let mut locations = Vec::new();

    // The antinodes closer to the new antenna
    for n in 0.. {
        let antinode_new_row = (n + 1) * new_row - n * old_row;
        let antinode_new_col = (n + 1) * new_col - n * old_col;
        if in_map((antinode_new_row, antinode_new_col), w, h) {
            locations.push((antinode_new_row, antinode_new_col));
        } else {
            break;
        }
    }

    // The antinodes closer to the old antenna
    for n in 0.. {
        let antinode_old_row = (n + 1) * old_row - n * new_row;
        let antinode_old_col = (n + 1) * old_col - n * new_col;
        if in_map((antinode_old_row, antinode_old_col), w, h) {
            locations.push((antinode_old_row, antinode_old_col));
        } else {
            break;
        }
    }

    locations
}

fn in_map((row, col): Coord, w: usize, h: usize) -> bool {
    row >= 0 && col >= 0 && row < h as i32 && col < w as i32
}
