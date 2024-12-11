use rayon::prelude::*;
use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, u64},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

static OPERATORS: [Operator; 3] = [Operator::Add, Operator::Mul, Operator::Combine];

enum Operator {
    Add,
    Mul,
    Combine,
}

impl Operator {
    fn apply(&self, x: &u64, y: &u64) -> u64 {
        match self {
            Operator::Add => x + y,
            Operator::Mul => x * y,
            Operator::Combine => format!("{x}{y}").parse().unwrap(),
        }
    }
}

fn check_valid_equation(
    test_value: u64,
    current_value: u64,
    idx: usize,
    nums: &[u64],
    op: &Operator,
) -> bool {
    if current_value > test_value {
        return false;
    }

    let num = nums[idx];
    let val: u64 = op.apply(&current_value, &num);

    if idx + 1 == nums.len() {
        return val == test_value;
    }

    OPERATORS
        .iter()
        .any(|op| check_valid_equation(test_value, val, idx + 1, nums, op))
}

fn parse_input(input: &[u8]) -> IResult<&[u8], Vec<(u64, Vec<u64>)>> {
    separated_list1(
        newline,
        separated_pair(u64, tag(": "), separated_list1(tag(" "), u64)),
    )(input)
}

fn process_part_two(input: &[u8]) -> u64 {
    let (_, equations) = parse_input(input).unwrap();
    equations
        .par_iter()
        .map(|(test_value, nums)| {
            let valid = OPERATORS
                .iter()
                .any(|op| check_valid_equation(*test_value, nums[0], 1, &nums, op));

            match valid {
                true => test_value,
                false => &0,
            }
        })
        .sum()
}

fn main() {
    let start = std::time::Instant::now();
    let input = fs::read("input.txt").expect("error loading the input file");
    println!("{}", process_part_two(&input));
    println!("{:.2?}", start.elapsed());
}
