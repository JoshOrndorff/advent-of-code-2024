// This is my first one that solved the example input but not my real input.
// It is because I was calculating the incorrect number of digits for numbers that were exact powers of ten.

use std::{collections::VecDeque, str::FromStr};

#[derive(Debug)]
struct Test {
    target: u64,
    operands: VecDeque<u64>,
}

impl FromStr for Test {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon_index = s.find(':').expect("colon");
        let target = s[..colon_index].parse().expect("target should parse");
        let operands = s[colon_index + 2..]
            .split(' ')
            .map(|n| n.parse().expect("operands should parse"))
            .collect();
        Ok(Self { target, operands })
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");

    let tests = input
        .lines()
        .map(|l| Test::from_str(l).expect("test should parse"));

    // println!("{:#?}", tests.clone().collect::<Vec<_>>());

    // Solve part 1
    let calibration_result: u64 = tests
        .clone()
        .filter(|t| can_work_add_multiply(t, false))
        .map(|t| t.target)
        .sum();
    println!("{calibration_result}");

    // Solve part 2
    let calibration_result: u64 = tests
        .filter(|t| can_work_add_multiply(t, true))
        .map(|t| t.target)
        .sum();
    println!("{calibration_result}");
}

fn can_work_add_multiply(Test { target, operands }: &Test, allow_concat: bool) -> bool {
    if operands.len() == 1 {
        return operands[0] == *target;
    }

    let mut add_operands = operands.clone();
    let a = add_operands.pop_front().unwrap();
    let b = add_operands.pop_front().unwrap();
    add_operands.push_front(a + b);

    let mut mul_operands = operands.clone();
    let a = mul_operands.pop_front().unwrap();
    let b = mul_operands.pop_front().unwrap();
    mul_operands.push_front(a * b);

    let mut cat_operands = operands.clone();
    let a = cat_operands.pop_front().unwrap();
    let b = cat_operands.pop_front().unwrap();
    cat_operands.push_front(concat(a, b));

    can_work_add_multiply(
        &Test {
            target: *target,
            operands: add_operands,
        },
        allow_concat,
    ) || can_work_add_multiply(
        &Test {
            target: *target,
            operands: mul_operands,
        },
        allow_concat,
    ) || (allow_concat
        && can_work_add_multiply(
            &Test {
                target: *target,
                operands: cat_operands,
            },
            allow_concat,
        ))
}

fn concat(a: u64, b: u64) -> u64 {
    let mut num_digits_in_b = 0u64;

    while b >= 10u64.pow(num_digits_in_b as u32) {
        num_digits_in_b += 1;
    }

    // println!("There are {num_digits_in_b} digits in {b}");

    a * 10u64.pow(num_digits_in_b as u32) + b
}

#[test]
fn one_ninety() {
    let t = Test {
        target: 190,
        operands: vec![10, 19].into(),
    };

    assert!(can_work_add_multiply(&t, false))
}

#[test]
fn fifteen_concat_six() {
    assert_eq!(concat(15, 6), 156)
}

#[test]
fn fifteen_concat_one() {
    assert_eq!(concat(15, 1), 151)
}

#[test]
fn fifteen_concat_ten() {
    assert_eq!(concat(15, 10), 1510)
}
