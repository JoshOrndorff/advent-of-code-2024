fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");
    let l = input.len();
    let mut total: u32 = 0;
    let mut part_2: u32 = 0;
    let mut enabled = true;

    for i in 0..input.len() {
        if try_parse_do(&input[i..]) {
            println!("ENABLING");
            enabled = true;
            continue;
        }

        if l - i > 7 && try_parse_dont(&input[i..]) {
            println!("DISABLING");
            enabled = false;
            continue;
        }

        if let Some((x, y)) = starts_with_mul(&input[i..]) {
            total += x * y;
            if enabled {
                part_2 += x * y;
            }
            println!("total: {total}, part 2: {part_2}");
        }
    }
}

// returns the two numbers being multiplied
fn starts_with_mul(input: &str) -> Option<(u32, u32)> {
    if &input[0..4] != "mul(" {
        return None;
    }

    let Some((x, x_len)) = try_parse_number(&input[4..]) else {
        return None;
    };

    if &input[x_len + 4..x_len + 5] != "," {
        return None;
    }

    let Some((y, y_len)) = try_parse_number(&input[x_len + 5..]) else {
        return None;
    };

    if &input[x_len + y_len + 5..x_len + y_len + 6] != ")" {
        return None;
    }

    Some((x, y))
}

fn try_parse_number(input: &str) -> Option<(u32, usize)> {
    if !input.chars().next().unwrap().is_digit(10) {
        return None;
    }

    let mut x: u32 = 0;
    let mut num_digits: usize = 0;

    input.chars().take_while(|c| c.is_digit(10)).for_each(|c| {
        let val: u32 = c.to_digit(10).unwrap();
        num_digits += 1;
        x *= 10;
        x += val;
    });

    Some((x, num_digits))
}

fn try_parse_do(input: &str) -> bool {
    &input[0..4] == "do()"
}

fn try_parse_dont(input: &str) -> bool {
    &input[0..7] == "don't()"
}
