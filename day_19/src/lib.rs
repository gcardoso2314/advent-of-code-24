use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::{many1, separated_list1},
    IResult,
};

use std::collections::HashMap;

fn parse_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (input, towels) = separated_list1(tag(", "), alpha1)(input)?;
    let (input, _) = many1(newline)(input)?;
    let (input, combos) = separated_list1(newline, alpha1)(input)?;

    Ok((input, (towels, combos)))
}

fn match_patterns(combo: &str, towels: &Vec<&str>, memo: &mut HashMap<String, usize>) -> usize {
    // Check if the result is already memoized
    if let Some(&cached_result) = memo.get(combo) {
        return cached_result;
    }

    let possible_towels: Vec<&str> = towels
        .iter()
        .filter_map(|&towel| {
            if combo.contains(towel) {
                Some(towel)
            } else {
                None
            }
        })
        .collect();

    if possible_towels.is_empty() {
        memo.insert(combo.to_string(), 0);
        return 0;
    }

    let result: usize = possible_towels
        .iter()
        .map(|&pat| {
            if combo == pat {
                1
            } else if !combo.starts_with(pat) {
                0
            } else {
                match_patterns(&combo[pat.len()..], &possible_towels, memo)
            }
        })
        .sum();

    // Memoize the result
    memo.insert(combo.to_string(), result);
    result
}

pub fn process_part_one(input: &str) -> usize {
    let (_, (towels, combos)) = parse_input(input).unwrap();

    // Create a memoization map
    let mut memo = HashMap::new();

    combos
        .iter()
        .filter(|&combo| match_patterns(combo, &towels, &mut memo) > 0)
        .count()
}

pub fn process_part_two(input: &str) -> usize {
    let (_, (towels, combos)) = parse_input(input).unwrap();

    // Create a memoization map
    let mut memo = HashMap::new();

    combos
        .iter()
        .map(|&combo| match_patterns(combo, &towels, &mut memo))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 6);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 16)
    }
}
