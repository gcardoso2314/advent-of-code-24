use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::collections::hash_map::HashMap;

fn parse_edge(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32, tag("|"), u32)(input)
}

fn parse_update(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), u32)(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<(u32, u32)>, Vec<Vec<u32>>)> {
    let (input, edges) = separated_list1(newline, parse_edge)(input)?;
    let (input, _) = many1(newline)(input)?;
    let (input, updates) = separated_list1(newline, parse_update)(input)?;
    Ok((input, (edges, updates)))
}

pub fn process_part_one(input: &str) -> u32 {
    let (_, (edges, updates)) = parse_input(input).unwrap();
    updates
        .iter()
        .filter_map(|update| {
            let valid_order = edges.iter().all(|(a, b)| {
                let Some(pos_a) = update.iter().position(|x| x == a) else {
                    return true;
                };
                let Some(pos_b) = update.iter().position(|x| x == b) else {
                    return true;
                };
                pos_a < pos_b
            });
            if valid_order {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn order_update(update: &Vec<u32>, edges: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut indegrees: HashMap<u32, usize> = update.iter().map(|v| (*v, 0)).collect();
    let mut edge_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for (a, b) in edges.iter() {
        if update.contains(a) && update.contains(b) {
            edge_map
                .entry(*a)
                .and_modify(|v| v.push(*b))
                .or_insert(vec![*b]);
            indegrees.entry(*b).and_modify(|v| *v += 1);
        }
    }

    let mut ordered_update: Vec<u32> = Vec::new();
    while indegrees.len() > 0 {
        let sources: Vec<u32> = indegrees
            .iter()
            .filter_map(|(a, b)| if *b == 0 { Some(*a) } else { None })
            .collect();
        for a in sources {
            ordered_update.push(a);
            indegrees.remove(&a);
            if let Some(neighbors) = edge_map.get(&a) {
                for x in neighbors {
                    indegrees.entry(*x).and_modify(|v| *v -= 1);
                }
            }
        }
    }

    ordered_update
}

pub fn process_part_two(input: &str) -> u32 {
    let (_, (edges, updates)) = parse_input(input).unwrap();
    updates
        .iter()
        .filter_map(|update| {
            let valid_order = edges.iter().all(|(a, b)| {
                let Some(pos_a) = update.iter().position(|x| x == a) else {
                    return true;
                };
                let Some(pos_b) = update.iter().position(|x| x == b) else {
                    return true;
                };
                pos_a < pos_b
            });
            if !valid_order {
                let ordered_update = order_update(update, &edges);
                Some(ordered_update[ordered_update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 143)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 123)
    }
}
