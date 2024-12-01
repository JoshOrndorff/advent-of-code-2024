fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");

    let mut left_col = Vec::new();
    let mut right_col = Vec::new();

    for line in input.lines() {
        let l: i128 = line[..5].parse().expect("left column parses");
        let r: i128 = line[8..].parse().expect("right column parses");

        left_col.push(l);
        right_col.push(r);
    }

    // Sort each column individually.
    left_col.sort();
    right_col.sort();

    // Part 1
    let total_distance: i128 = left_col
        .iter()
        .zip(&right_col)
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("{total_distance}");

    // Part 2
    // This is ripe for optimizations, but I'll just do
    // the boring old O(n^2) way because this is day one.

    let similarity_score: i128 = left_col
        .iter()
        .map(|l| l * right_col.iter().filter(|r| r == &l).count() as i128)
        .sum();

    println!("{similarity_score}");
}
