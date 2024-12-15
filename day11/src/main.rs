// The big problem with my current caching is that it only caches the result for the exact requested number of generations.
// This is true whether we are returning the actual nth generation or just the size of that generation.
// I would need to cache all the intermediate generations too.
// For example. If I encounter an N with 17 generations to go, I will need to calculate each of the intermediate generations.
// So I _should_ cache the result for N after 1 generation, N after 2 gen..., n after 17 generations.
//
// One way to implement this is to return a vector with the result for all the intermediate generations.
// So in the example above I would get back [n after 1 gen, n after 2 gen, ..., n after 17 gen]. Then I can cache them all

use std::collections::{HashMap, VecDeque};

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");
    // let input = std::fs::read_to_string("./example.txt").expect("input file should exist");

    let starting_sequence = input
        .trim()
        .split(' ')
        // .map(|s| s.to_string())
        .collect::<Vec<_>>();

    // Solve part 1 by the direct expansion method
    println!(
        "Part 1 via direct expansion: {}",
        direct_expansion(25, &starting_sequence).len()
    );

    let mut cache = HashMap::<String, VecDeque<usize>>::new();

    ////// CAREFUL!!! The indices are off by one from what I expected. IDK why.
    ////// But once I figured it out it was easy enough to change it and get that delicious star.

    // Solve part 1 by the recursive expansion method
    println!(
        "Part 1 via recursive expansion: {}",
        recursive_expansion_of_sequence(25, &starting_sequence, &mut cache)[24]
    );

    // Solve part 2 by recursive expansion with a cache.
    println!(
        "Part 2 via cached recursive expansion: {}",
        recursive_expansion_of_sequence(75, &starting_sequence, &mut cache)[74]
    );
}

/// Expands a single rock.
///
/// This helper is only used in the direct expansion technique.
/// Similar, but not identical logic is repeated in the recursive method.
fn expand_single(rock: &str) -> Vec<String> {
    if rock == "0" {
        vec![String::from("1")]
    } else if rock.len() % 2 == 0 {
        let mut next_gen = Vec::new();

        // The left half is straight forward
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
/// This is good enough to do 25 generations for part 1 but not to do 75 for part 2
fn direct_expansion(generations: usize, starting_sequence: &[&str]) -> Vec<String> {
    let mut sequence = starting_sequence
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

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

/// Solves the rock problem in a depth first way fully expanding the first rock in the starting
/// sequence before moving on to the second.
/// This allows caching.
///
/// This function returns the number of rocks in each generation up to the one given.
/// This allows better caching.
fn recursive_expansion(
    generations: usize,
    rock: &str,
    cache: &mut HashMap<String, VecDeque<usize>>,
) -> VecDeque<usize> {
    // The terminating case is when we are asked for zero generations.
    // We return a vec length 1 saying this generation is size 1.
    // println!("Entering Single requesting {generations} generations");
    if generations == 0 {
        // println!("Ending Single at terminating case");
        return [1].into();
    }

    // Check if the call we are making is already in the cache.
    if let Some(cached_results) = cache.get(rock) {
        // So far we know that we have _something cached for this type of rock, but we don't know if it is enough generations.
        if cached_results.len() > generations {
            // Now we have a proper cache hit.
            // println!("CACHE HIT  for rock {rock}  after    {generations} generations.");
            return cached_results.clone();
        } else {
            // println!(
            //     " Partial  for rock {rock}  after    {generations} generations. We only had {}",
            //     cached_results.len()
            // );
        }
    } else {
        // println!("Total MISS for rock {rock} requested {generations} generations.");
    }

    // So now we do the real calculating
    let mut next_several_generations;
    if rock == "0" {
        let daughter = "1";
        next_several_generations = recursive_expansion(generations - 1, daughter, cache);
        next_several_generations.push_front(1);
    } else if rock.len() % 2 == 0 {
        // The left half is straight forward
        let l = rock.len() / 2;
        let daughter_left = &rock[..l];

        // The right half may have leading zeros.
        let first_nonzero_in_right_half_index = rock[l..].find(|c| c != '0');
        let daughter_right = match first_nonzero_in_right_half_index {
            Some(i) => &rock[(l + i)..],
            None => "0",
        };

        next_several_generations = recursive_expansion_of_sequence(
            generations - 1,
            &[&daughter_left, &daughter_right],
            cache,
        );
        next_several_generations.push_front(2);
    } else {
        let daughter = (rock.parse::<usize>().unwrap() * 2024).to_string();
        next_several_generations = recursive_expansion(generations - 1, &daughter, cache);
        next_several_generations.push_front(1);
    }

    // I think we need to do some zipping somewhere. around here or in a branch above.
    cache.insert(rock.into(), next_several_generations.clone());

    next_several_generations
}

/// Helper that calls the recursive expansion method for sequences.
/// This is useful for the original sequence and for the case where a large rock is split into two.
fn recursive_expansion_of_sequence(
    generations: usize,
    rock_sequence: &[&str],
    cache: &mut HashMap<String, VecDeque<usize>>,
) -> VecDeque<usize> {
    // println!("Entering sequence of length {} requesting {generations} generations", rock_sequence.len());
    let generations_by_rock = rock_sequence
        .iter()
        .map(|&rock| {
            let generations = recursive_expansion(generations, rock, cache);
            // println!("rock in sequence yielded {} length list", generations.len());
            generations
        })
        .collect::<Vec<_>>();

    // This is the thing we will return. It has a slot for each requested generation (including zero)
    // We initialize the slots to zero.
    let mut zipped = VecDeque::with_capacity(generations + 1);
    // println!("zipped length")
    for _ in 0..=generations {
        zipped.push_back(0);
    }

    // No go through and zip the other generations.
    for rock_index in 0..rock_sequence.len() {
        for generation in 0..=generations {
            zipped[generation] += generations_by_rock[rock_index][generation];
        }
    }

    zipped
}
