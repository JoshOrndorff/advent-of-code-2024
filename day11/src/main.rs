// 194444 is wrong for part 1

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");
    // let input = std::fs::read_to_string("./example.txt").expect("input file should exist");

    let mut sequence: Vec<String> = input
        .trim()
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    for i in 1..=25 {
        // We don't want to be constantly resizing the vec, so allocate double the previous round.
        // This will always be enough and will almost always be too much.
        let mut next_gen = Vec::with_capacity(sequence.len() * 2);

        for rock in sequence {
            if rock == "0" {
                next_gen.push(String::from("1"));
            } else if rock.len() % 2 == 0 {
                let l = rock.len() / 2;
                next_gen.push(rock[..l].to_string());

                // The right half may have leading zeros.
                let fisrt_nonzero_in_right_half_index = rock[l..].find(|c| c != '0');
                let next_val = match fisrt_nonzero_in_right_half_index {
                    Some(i) => &rock[(l + i)..],
                    None => "0",
                }.to_string();
                next_gen.push(next_val);
            } else {
                let val = rock.parse::<usize>().unwrap();
                next_gen.push((val * 2024).to_string());
            }
        }
        sequence = next_gen;
        // println!("{:?}", sequence);

        println!("After blinking {i} times you have {} stones", sequence.len());
    }
}
