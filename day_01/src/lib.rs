use nom::{
    character::complete::{newline, space1, u32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::hash_map::HashMap;
use std::iter::zip;

fn parse_input(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, pairs) = separated_list1(newline, separated_pair(u32, space1, u32))(input)?;
    Ok((input, pairs.into_iter().unzip()))
}

pub fn process_part_one(input: &str) -> u32 {
    let (_, (mut list1, mut list2)) = parse_input(input).unwrap();
    list1.sort_unstable();
    list2.sort_unstable();

    zip(list1, list2).map(|(a, b)| a.abs_diff(b)).sum()
}

pub fn process_part_two(input: &str) -> u32 {
    let (_, (list1, list2)) = parse_input(input).unwrap();

    // Create a hashmap of counts in list2
    let loc_id_counts = list2.iter().fold(HashMap::new(), |mut counts, &a| {
        *counts.entry(a).or_insert(0) += 1;
        counts
    });

    list1
        .iter()
        .map(|a| a * loc_id_counts.get(a).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 11)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 31)
    }
}
