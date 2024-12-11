use std::collections::HashSet;

type Coord = (usize, usize);

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");
    // let input = std::fs::read_to_string(".example.txt").expect("input file should exist");
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

    // Find the trailheads (Coulda done this in the same iteration above, but I think this is clearer)
    let mut trailheads: HashSet<Coord> = HashSet::new();
    for row in 0..height {
        for col in 0..width {
            if map[row][col] == 0 {
                trailheads.insert((row, col));
            }
        }
    }

    println!("Found {} trailheads.", trailheads.len());

    // Part 1: For each trailhead, calculate its score, and add them up.
    let mut sum_of_scores = 0usize;
    let mut sum_of_ranks = 0usize;

    for trailhead in trailheads {
        let (reachable, trail_count) = reachable_niners(&trailhead, &map);
        sum_of_scores += reachable.len();
        sum_of_ranks += trail_count;
    }

    println!("Sum of trailheads' scores is {sum_of_scores}.");
    println!("Sum of trailheads' ranks is {sum_of_ranks}.");
}

/// Given a starting point on the map, calculates
/// 1. The set of all the reachable niners
/// 2. The total number of unique trails to niners.
///
/// This is a recursive algorithm.

fn reachable_niners(&(row, col): &Coord, map: &[Vec<usize>]) -> (HashSet<Coord>, usize) {
    // The terminating case is when the starting position is, itself a niner.
    if map[row][col] == 9 {
        return (HashSet::from([(row, col)]), 1);
    }

    // Figure out which neighbors are viable
    let viable_neighbors = neighbors(row, col, map).filter(|(neighbor_row, neighbor_col)| {
        map[*neighbor_row][*neighbor_col] == map[row][col] + 1
    });

    // Recursive call.
    // The niners reachable from this point are the union of the niners reachable from all of this point's immediately reachable neighbor
    // And the number of unique trails is the sum of the unique trails
    let mut reachable = HashSet::new();
    let mut trail_count = 0usize;
    for n in viable_neighbors {
        let (neighbor_reachable, neighbor_trail_count) = reachable_niners(&n, map);
        reachable.extend(neighbor_reachable);
        trail_count += neighbor_trail_count;
    }

    (reachable, trail_count)
}

fn neighbors(row: usize, col: usize, map: &[Vec<usize>]) -> impl Iterator<Item = Coord> {
    let mut neighbors = Vec::new();

    if row > 0 {
        neighbors.push((row - 1, col));
    }
    if row < map.len() - 1 {
        neighbors.push((row + 1, col));
    }
    if col > 0 {
        neighbors.push((row, col - 1));
    }
    if col < map[0].len() - 1 {
        neighbors.push((row, col + 1));
    }

    neighbors.into_iter()
}
