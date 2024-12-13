use std::collections::{HashMap, HashSet};

/// This is a hack to indicate when a plot has not yet been assigned to a region.
/// If there are more than this many regions, we're fucked. Or just need a bigger constant.
const UNASSIGNED: usize = 9999999;

fn main() {
    // let input = std::fs::read_to_string("./input.txt").expect("input file should exist");
    let input = std::fs::read_to_string("./example.txt").expect("input file should exist");

    // We make 2 2D lists. One represents the input as given.
    // The other is the region id assigned to the corresponding plot.
    let garden = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = garden.len();
    let width = garden[0].len();
    let mut regions = get_unassigned_2d(width, height);

    // Figure out which region each plot is in by iterating over each one and looking at its neighbors.
    let mut next_unused_plot_id = 0usize;
    for row in 0..height {
        for col in 0..width {
            let plot_type = garden[row][col];
            let maybe_same_plot_neighbor = get_neighbors(row, col, width, height).iter().find_map(
                |(neighbor_row, neighbor_col)| {
                    // They need to be of the same type AND already have a region.
                    if garden[*neighbor_row][*neighbor_col] == plot_type
                        && regions[*neighbor_row][*neighbor_col] != UNASSIGNED
                    {
                        Some(regions[*neighbor_row][*neighbor_col])
                    } else {
                        None
                    }
                },
            );
            if let Some(neighbor_region_id) = maybe_same_plot_neighbor {
                regions[row][col] = neighbor_region_id;
            } else {
                regions[row][col] = next_unused_plot_id;
                next_unused_plot_id += 1;
            }
        }
    }

    // For debugging purposes, print out the region ids (gets fucked up after they are multiple hexadigits long)
    for row in 0..height {
        for col in 0..width {
            let region_id = regions[row][col];
            // format in hex so it stays single character longer.
            print!("{:x}", region_id);
        }
        println!();
    }

    // Now we go through again sorting each plot into a map by region.
    // (Could have done this in the first iteration, but it reads better this way imo)
    let mut by_region: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for row in 0..height {
        for col in 0..width {
            let region_id = regions[row][col];
            by_region.entry(region_id).or_default().push((row, col));
        }
    }

    // Now we iterate through each region.
    // Finding the area is easy. For the perimeter, we loop through each plot in the region and sum up how many outside edges are present.
    let mut total_price = 0usize;
    for (_region_id, plots) in by_region {
        let area = plots.len();
        let mut perimeter = 0usize;
        let mut plot_type = ' ';
        for (row, col) in plots.clone() {
            plot_type = garden[row][col];
            for neighbor in get_neighbors(row, col, width, height) {
                if !plots.contains(&neighbor) {
                    perimeter += 1;
                }
            }
        }
        let price = perimeter * area;
        println!("A region of {plot_type} plants with price {area} x {perimeter} = {price}");
        total_price += price;
    }
    println!("total price is {total_price}");
}

fn get_neighbors(row: usize, col: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    // Look Up
    if row > 0 {
        neighbors.push((row - 1, col));
    }
    // Look left
    if col > 0 {
        neighbors.push((row, col - 1));
    }
    // Look down
    if row < height - 1 {
        neighbors.push((row + 1, col));
    }
    // Look right
    if col < width - 1 {
        neighbors.push((row, col + 1));
    }

    neighbors
}

/// This is an ugly way to not initialize a 2d vec.
fn get_unassigned_2d(width: usize, height: usize) -> Vec<Vec<usize>> {
    let template_row = [UNASSIGNED].repeat(width);
    let mut result = Vec::with_capacity(height);
    for _ in 0..height {
        result.push(template_row.clone());
    }
    result
}

#[test]
fn neighbors_corner() {
    assert_eq!(
        get_neighbors(0, 0, 10, 10).iter().collect::<HashSet<_>>(),
        [(0, 1), (1, 0)].iter().collect::<HashSet<_>>()
    );
}
