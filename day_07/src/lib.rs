use rayon::prelude::*;

use nom::{
    bytes::complete::tag, character::complete::u64, multi::separated_list1,
    sequence::separated_pair, IResult,
};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Combine,
}

impl Operator {
    fn inverse_apply(&self, x: &u64, y: &u64) -> Option<u64> {
        match self {
            Operator::Add => x.checked_sub(*y),
            Operator::Mul => x.checked_div(*y),
            Operator::Combine => Some(x / 10u64.pow(y.ilog10() + 1)),
        }
    }
}

fn parse_row(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    separated_pair(u64, tag(": "), separated_list1(tag(" "), u64))(input)
}

#[inline]
fn is_combineable(test_value: u64, num: u64) -> bool {
    if test_value.checked_sub(num).is_some() {
        (test_value - num) % 10u64.pow(num.ilog(10) + 1) == 0
    } else {
        false
    }
}

fn check_equation(
    test_value: u64,
    idx: usize,
    nums: &[u64],
    op: &Operator,
    available_ops: &[Operator],
) -> bool {
    let num = nums[idx];

    let mut valid: bool = false;

    match op {
        Operator::Mul => {
            if test_value % num != 0 {
                return false;
            }
        }
        Operator::Combine => {
            if !is_combineable(test_value, num) {
                return false;
            }
        }
        Operator::Add => {}
    }

    if let Some(val) = op.inverse_apply(&test_value, &num) {
        if idx == 0 {
            return val == 0;
        } else {
            valid |= available_ops
                .iter()
                .any(|op| check_equation(val, idx - 1, nums, op, available_ops));
        }
    }

    valid
}

pub fn process_part_one(input: &str) -> u64 {
    let available_ops = [Operator::Add, Operator::Mul];
    input
        .par_lines()
        .map(|line| {
            let (_, (test_value, nums)) = parse_row(line).unwrap();
            let valid = available_ops
                .iter()
                .any(|op| check_equation(test_value, nums.len() - 1, &nums, op, &available_ops));

            match valid {
                true => test_value,
                false => 0,
            }
        })
        .sum()
}

pub fn process_part_two(input: &str) -> u64 {
    let available_ops = [Operator::Add, Operator::Mul, Operator::Combine];
    input
        .par_lines()
        .map(|line| {
            let (_, (test_value, nums)) = parse_row(line).unwrap();
            let valid = available_ops
                .iter()
                .any(|op| check_equation(test_value, nums.len() - 1, &nums, op, &available_ops));

            match valid {
                true => test_value,
                false => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_is_combineable() {
        assert_eq!(is_combineable(12345, 345), true);
        assert_eq!(is_combineable(1847, 7), true);
        assert_eq!(is_combineable(1789, 23), false);
        assert_eq!(is_combineable(76, 6), true);
        assert_eq!(is_combineable(320, 20), true);
    }

    #[test]
    fn test_inverse_apply() {
        let op = Operator::Combine;
        assert_eq!(op.inverse_apply(&12345, &345), Some(12));
        assert_eq!(op.inverse_apply(&5, &5), Some(0));
        assert_eq!(op.inverse_apply(&96, &6), Some(9));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 3749)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 11387)
    }
}
