use rayon::prelude::*;

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
            Operator::Combine => x * 10u64.pow(y.ilog10() + 1) + y,
        }
    }
}

fn parse_row(input: &[u8]) -> IResult<&[u8], (u64, Vec<u64>)> {
    separated_pair(u64, tag(": "), separated_list1(tag(" "), u64))(input)
}

fn parse_input(input: &[u8]) -> IResult<&[u8], Vec<(u64, Vec<u64>)>> {
    separated_list1(newline, parse_row)(input)
}

fn check_valid_equation(
    test_value: u64,
    current_value: u64,
    idx: usize,
    nums: &[u64],
    op: &Operator,
    available_ops: &[Operator],
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
        .any(|op| check_valid_equation(test_value, val, idx + 1, nums, op, available_ops))
}

pub fn process_part_one(input: &[u8]) -> u64 {
    let (_, equations) = parse_input(input).unwrap();
    let available_ops = [Operator::Add, Operator::Mul];
    equations
        .iter()
        .map(|(test_value, nums)| {
            let valid = available_ops
                .iter()
                .any(|op| check_valid_equation(*test_value, nums[0], 1, &nums, op, &available_ops));
            match valid {
                true => test_value,
                false => &0,
            }
        })
        .sum()
}

pub fn process_part_two(input: &[u8]) -> u64 {
    let (_, equations) = parse_input(input).unwrap();
    let available_ops = [Operator::Add, Operator::Mul, Operator::Combine];
    equations
        .par_iter()
        .map(|(test_value, nums)| {
            let valid = available_ops
                .iter()
                .any(|op| check_valid_equation(*test_value, nums[0], 1, &nums, op, &available_ops));

            match valid {
                true => test_value,
                false => &0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &[u8] = b"190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20";

    #[test]
    #[ignore]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 3749)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 11387)
    }
}
