use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

// I'm mapping from the right input column to a set of all items that were matched with it.
// Conceptually: When checking an update and you try to print a given page, you look it up in this map.
// The set you find is a set of pages that mut not come after it in the update.
type Deps = HashMap<u32, HashSet<u32>>;

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");

    let mut parts = input.split("\n\n");

    // Set up a mapping of the dependencies and parse them
    let mut deps: Deps = HashMap::new();
    for dep_str in parts.next().expect("input formatting").lines() {
        let parts = dep_str
            .split('|')
            .map(|s| s.parse::<u32>().expect("numbers parse in dependencies"))
            .collect::<Vec<_>>();
        deps.entry(parts[1]).or_default().insert(parts[0]);
    }
    // println!("{:#?}", deps);

    // Parse the updates and separate the correctly ordered ones from the incorrect ones
    let (correctly_ordered, incorrectly_ordered): (Vec<Vec<u32>>, Vec<Vec<u32>>) = parts
        .next()
        .expect("input formatting 2")
        .lines()
        .map(|update_str| {
            update_str
                .split(',')
                .map(|s| s.parse::<u32>().expect("numbers parse in updates"))
                .collect::<Vec<_>>()
        })
        .partition(|update| is_correctly_ordered(update, &deps));
    // println!("{:?}", updates);

    // Solve part 1
    let part_1: u32 = correctly_ordered
        .iter()
        .map(|update| middle_number(update))
        .sum();
    println!("{part_1}");

    // Solve part 2
    let part_2: u32 = incorrectly_ordered
        .iter()
        .map(|original| middle_number(&ordered_clone(original, &deps)))
        .sum();

    println!("{part_2}");
}

/// Given a list of numbers, this function returns the one in the middle position.
fn middle_number(nums: &[u32]) -> u32 {
    nums[nums.len() / 2]
}

fn is_correctly_ordered(update: &[u32], deps: &Deps) -> bool {
    for i in 0..update.len() - 1 {
        let forbidden_after = deps.get(&update[i]).expect(
            "nothing forbidden after... I'll have to figure out how to return an empty set here.",
        );
        if update[i + 1..]
            .iter()
            .any(|later_page| forbidden_after.contains(later_page))
        {
            return false;
        }
    }

    true
}

fn ordered_clone(original: &[u32], deps: &Deps) -> Vec<u32> {
    let mut fixed = Vec::from(original);

    fixed.sort_by(|a, b| compare_pages(a, b, deps));

    fixed
}

fn compare_pages(a: &u32, b: &u32, deps: &Deps) -> Ordering {
    let a_deps = deps.get(&a).expect("get a deps");
    let b_deps = deps.get(&b).expect("get b deps");

    if a_deps.contains(&b) {
        return Ordering::Less;
    }

    if b_deps.contains(&a) {
        return Ordering::Greater;
    }

    println!("Didn't expect to end up here !!!!!!!!!!!!!!!!!!!");
    Ordering::Equal
}
