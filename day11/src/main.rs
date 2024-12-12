use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");
    // let input = std::fs::read_to_string("./example.txt").expect("input file should exist");

    let starting_sequence: Vec<String> = input
        .trim()
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    // Solve part 1 by the direct expansion method
    println!(
        "Part 1 via direct expansion: {}",
        direct_expansion(25, &starting_sequence).len()
    );

    // Solve part 1 by the recursive expansion method
    // println!(
    //     "Part 1 via recursive expansion: {}",
    //     recursive_expansion(25, &starting_sequence).len()
    // );

    // Solve part 2 by recursive expansion with a cache.
    let mut cache = HashMap::<(usize, Vec<String>), Vec<String>>::new();
    println!(
        "Part 2 via cached recursive expansion: {}",
        recursive_expansion(75, &starting_sequence, &mut cache).len()
    );
}

fn expand_single(rock: &str) -> Vec<String> {
    if rock == "0" {
        vec![String::from("1")]
    } else if rock.len() % 2 == 0 {
        let mut next_gen = Vec::new();

        let l = rock.len() / 2;
        next_gen.push(rock[..l].to_string());

        // The right half may have leading zeros.
        let first_nonzero_in_right_half_index = rock[l..].find(|c| c != '0');
        let next_val = match first_nonzero_in_right_half_index {
            Some(i) => &rock[(l + i)..],
            None => "0",
        }
        .to_string();
        next_gen.push(next_val);

        next_gen
    } else {
        vec![(rock.parse::<usize>().unwrap() * 2024).to_string()]
    }
}

/// Solves the rock problem by directly expanding the sequence of rocks generation by generation.
/// This is good enough to do 25 generations for part 1 but to do 75 for part 2
fn direct_expansion(generations: usize, starting_sequence: &Vec<String>) -> Vec<String> {
    let mut sequence = starting_sequence.clone();

    for _ in 1..=generations {
        // We don't want to be constantly resizing the vec, so allocate double the previous round.
        // This will always be enough and will almost always be too much.
        let mut next_gen = Vec::with_capacity(sequence.len() * 2);

        for rock in sequence {
            next_gen.extend(expand_single(&rock));
        }
        sequence = next_gen;
    }
    sequence
}

// IDEA: Maybe return an iterator instead...

/// Solves the rock problem in a depth first way fully expanding the first rock in the starting
/// sequence before moving on to the second.
/// This allows caching.
fn recursive_expansion(
    generations: usize,
    starting_sequence: &Vec<String>,
    cache: &mut HashMap<(usize, Vec<String>), Vec<String>>,
) -> Vec<String> {
    // The terminating case is when we are asked for zero generations. Then we just return the rocks we were given.
    if generations == 0 {
        return starting_sequence.clone();
    }

    // Check if the call we are making is already in the cache.
    if let Some(cached_result) = cache.get(&(generations, starting_sequence.clone())) {
        println!("CACHE HIT BABY!!!! {:?}", starting_sequence);
        return cached_result.clone();
    }

    println!("cache miss {:?}", starting_sequence);

    let mut fully_expanded = Vec::new();

    for rock in starting_sequence {
        let single_rock_expansion =
            recursive_expansion(generations - 1, &expand_single(rock), cache);
        fully_expanded.extend(single_rock_expansion);
    }

    cache.insert(
        (generations, starting_sequence.clone()),
        fully_expanded.clone(),
    );

    fully_expanded
}
