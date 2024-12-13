use std::collections::HashSet;

struct Region {
    plots: HashSet<(usize, usize)>,
    entry: (usize, usize),
    plant_type: char,
}

fn main() {
    // let input = std::fs::read_to_string("./input.txt").expect("input file should exist");
    let input = std::fs::read_to_string("./example.txt").expect("input file should exist");

    let garden = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = garden.len();
    let width = garden[0].len();

    // Figure out which region each plot is in by iterating over each one and looking at its neighbors.
    let mut explored = HashSet::new();
    let mut total_standard_price = 0usize;
    let mut total_bulk_price = 0usize;
    for row in 0..height {
        for col in 0..width {
            if explored.contains(&(row, col)) {
                continue;
            }
            let region = Region {
                plots: discover_region_recursively(
                    (row, col),
                    width,
                    height,
                    &mut explored,
                    &garden,
                ),
                entry: (row, col),
                plant_type: garden[row][col],
            };

            // Calculate the perimeter
            let perimeter = calculate_perimeter(&region, width, height);
            let sides = calculate_sides(&region, width, height);
            let area = region.plots.len();

            let standard_price = perimeter * area;
            let bulk_price = sides * area;

            total_standard_price += standard_price;
            total_bulk_price += bulk_price;

            println!("A region of {} plants with area: {area}, perimiter: {perimeter}, and sides: {sides}", region.plant_type);
        }
    }

    println!("total standard price is {total_standard_price}");
    println!("total bulk price is {total_bulk_price}");
}

fn calculate_perimeter(region: &Region, width: usize, height: usize) -> usize {
    let mut perimeter = 0usize;
    for (row, col) in &region.plots {
        let neighbors = get_neighbors(*row, *col, width, height);
        perimeter += 4 - neighbors.len();
        for neighbor in neighbors {
            if !region.plots.contains(&neighbor) {
                perimeter += 1;
            }
        }
    }
    perimeter
}

use Facing::*;
#[derive(Clone, Copy)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    fn turn_left(&self) -> Self {
        match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
}

fn straight_ahead(
    row: usize,
    col: usize,
    width: usize,
    height: usize,
    facing: Facing,
    region: &Region,
) -> Option<(usize, usize)> {
    match facing {
        Up => {
            if row > 0 && region.plots.contains(&(row - 1, col)) {
                Some((row - 1, col))
            } else {
                None
            }
        }
        Down => {
            if row < height - 1 && region.plots.contains(&(row + 1, col)) {
                Some((row + 1, col))
            } else {
                None
            }
        }
        Left => {
            if col > 0 && region.plots.contains(&(row, col - 1)) {
                Some((row, col - 1))
            } else {
                None
            }
        }
        Right => {
            if col < width - 1 && region.plots.contains(&(row, col + 1)) {
                Some((row, col + 1))
            } else {
                None
            }
        }
    }
}

fn to_my_left(
    row: usize,
    col: usize,
    width: usize,
    height: usize,
    facing: Facing,
    region: &Region,
) -> Option<(usize, usize)> {
    match facing {
        Up => {
            if col > 0 && region.plots.contains(&(row, col - 1)) {
                Some((row, col - 1))
            } else {
                None
            }
        }
        Down => {
            if col < width - 1 && region.plots.contains(&(row, col + 1)) {
                Some((row, col + 1))
            } else {
                None
            }
        }
        Left => {
            if row < height - 1 && region.plots.contains(&(row + 1, col)) {
                Some((row + 1, col))
            } else {
                None
            }
        }
        Right => {
            if row > 0 && region.plots.contains(&(row - 1, col)) {
                Some((row - 1, col))
            } else {
                None
            }
        }
    }
}

fn to_my_right(
    row: usize,
    col: usize,
    width: usize,
    height: usize,
    facing: Facing,
    region: &Region,
) -> Option<(usize, usize)> {
    match facing {
        Up => {
            if col < width - 1 && region.plots.contains(&(row, col + 1)) {
                Some((row, col + 1))
            } else {
                None
            }
        }
        Down => {
            if col > 0 && region.plots.contains(&(row, col - 1)) {
                Some((row, col - 1))
            } else {
                None
            }
        }
        Left => {
            if row > 0 && region.plots.contains(&(row - 1, col)) {
                Some((row - 1, col))
            } else {
                None
            }
        }
        Right => {
            if row < height - 1 && region.plots.contains(&(row + 1, col)) {
                Some((row + 1, col))
            } else {
                None
            }
        }
    }
}

// We start at the entry point facing right with our left hand on the fence.
// The entry is guaranteed to be an (not the) upper left corner.
fn calculate_sides(region: &Region, width: usize, height: usize) -> usize {
    let mut sides = 0usize;
    let mut facing = Right;

    let (mut row, mut col) = region.entry;
    loop {
        if to_my_left(row, col, width, height, facing, region).is_some() {
            sides += 1;
            facing = facing.turn_left();
        } else if straight_ahead(row, col, width, height, facing, region).is_some() {
            // There is nothing to do here except step forward afterwards
            // but it is important to have this elif clause because we don't want to turn right
            // if we could have gone straight.
        } else if to_my_right(row, col, width, height, facing, region).is_some() {
            sides += 1;
            facing = facing.turn_right();
        } else {
            panic!("We weren't able to go left right or forward. Something is fucked.")
        }

        (row, col) = straight_ahead(row, col, width, height, facing, region).unwrap();

        // I guess this is how you do a do-while-loop in Rust
        if (row, col) == region.entry {
            break;
        }
    }

    sides
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
