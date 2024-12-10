// Part 1: 613 is too high.

use std::collections::{HashMap, HashSet};

type Coord = (i32, i32);

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let height = map.len();
    let width = map[0].len();

    // Two mappings to track the locations of antennas and antinodes. Each mapping is from the antenna type to a set of coordinates.
    let mut antennas: HashMap<char, HashSet<Coord>> = HashMap::new();
    let mut antinodes: HashMap<char, HashSet<Coord>> = HashMap::new();

    // Go through the entire map looking for antennas and when one is found:
    // Calculate and catalog antinodes formed with previously known antennas
    // Catalog the newly-found antenna itself
    for row in 0..height {
        for col in 0..width {
            let c = map[row][col];
            if c == '.' {
                continue;
            }

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
                // The antinode closer to the new antenna
                let antinode_new_row = 2 * new_row - old_row;
                let antinode_new_col = 2 * new_col - old_col;
                antinodes
                    .entry(c)
                    .or_default()
                    .insert((antinode_new_row, antinode_new_col));

                // The antinode closer to the old antenna
                let antinode_old_row = 2 * old_row - new_row;
                let antinode_old_col = 2 * old_col - new_col;
                antinodes
                    .entry(c)
                    .or_default()
                    .insert((antinode_old_row, antinode_old_col));
            }

            // Now we can insert the freshly discovered antenna into the antennas mapping.
            antennas
                .entry(c)
                .or_default()
                .insert((row as i32, col as i32));
        }
    }

    // To solve part 1, we just have to count how many coordinates are in the antinodes map.
    // Let's first count within an id, then sum all ids.
    let part_1: usize = antinodes.values().map(|coords| coords.len()).sum();
    println!("{part_1}");

    // I'm curious is there are any locations that are antinodes for two different types of antennas.
    // So let's dedupe first and see if we get the same result
    let deduped_antinode_locations = antinodes.values().flatten().collect::<HashSet<_>>().len();
    println!("{deduped_antinode_locations}");
}
