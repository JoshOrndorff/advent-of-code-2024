use std::collections::HashSet;

type Coord = (usize, usize);

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");
    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let height = map.len();
    let width = map[0].len();

    // Find the trailheads and niners
    let mut trailheads: HashSet<Coord> = HashSet::new();
    let mut niners: HashSet<Coord> = HashSet::new();
    for row in 0..height {
        for col in 0..width {
            match map[row][col] {
                0 => trailheads.insert((row, col)),
                9 => niners.insert((row, col)),
                _ => false, // Just do nothing
            };
        }
    }

    // Part 1: For each trailhead, calculate its score, and add them up.
    let sum_of_scores: usize = trailheads.iter().map(|trailhead| score_path(trailhead, &map)).sum();

    println!("sum of trailheads' scores is {sum_of_scores}");
}

// Given a starting point on the map, counts how many...
fn score_path((mut row, mut col): &Coord, map: &[Vec<usize>]) -> usize {
    todo!()
}
