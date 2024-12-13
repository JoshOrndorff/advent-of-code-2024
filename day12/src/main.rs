use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");
    // let input = std::fs::read_to_string("./example.txt").expect("input file should exist");

    let garden = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = garden.len();
    let width = garden[0].len();

    // Figure out which region each plot is in by iterating over each one and looking at its neighbors.
    let mut explored = HashSet::new();
    let mut total_price = 0usize;
    for row in 0..height {
        for col in 0..width {
            if explored.contains(&(row, col)) {
                continue;
            }
            print!("A region of {} plants", garden[row][col]);
            let region =
                discover_region_recursively((row, col), width, height, &mut explored, &garden);

            // Calculate the perimeter
            let mut perimeter = 0usize;
            for (row, col) in region.clone() {
                let neighbors = get_neighbors(row, col, width, height);
                perimeter += 4 - neighbors.len();
                for neighbor in neighbors {
                    if !region.contains(&neighbor) {
                        perimeter += 1;
                    }
                }
            }

            let area = region.len();
            let price = perimeter * area;
            println!("plants with price {area} x {perimeter} = {price}");
            total_price += price;
        }
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

// Given a starting point, this function discovers all the adjacent tiles that belong to
// to the same region. It records this information in two ways.
// 1. It records the region in the region mapping.
// 2. It marks the cell as explored ad gives it a region id in the regions grid.
fn discover_region_recursively(
    (row, col): (usize, usize),
    width: usize,
    height: usize,
    explored: &mut HashSet<(usize, usize)>,
    garden: &Vec<Vec<char>>,
) -> HashSet<(usize, usize)> {
    // If this cell is already explored, there is nothing to do. Simply return.
    if explored.contains(&(row, col)) {
        return HashSet::new();
    }

    // Immediately mark this plot as explored to prevent infinite recursion
    explored.insert((row, col));

    let mut all_discovered_in_region = HashSet::new();
    all_discovered_in_region.insert((row, col));
    for neighbor in get_neighbors(row, col, width, height) {
        if garden[row][col] == garden[neighbor.0][neighbor.1] {
            all_discovered_in_region.extend(discover_region_recursively(
                neighbor, width, height, explored, garden,
            ));
        }
    }

    all_discovered_in_region
}

#[test]
fn neighbors_corner() {
    assert_eq!(
        get_neighbors(0, 0, 10, 10).iter().collect::<HashSet<_>>(),
        [(0, 1), (1, 0)].iter().collect::<HashSet<_>>()
    );
}
