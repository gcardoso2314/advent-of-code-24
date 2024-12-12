use std::collections::HashMap;

fn apply_rules(num: &usize) -> Vec<usize> {
    if num == &0 {
        vec![1]
    } else if (num.ilog10() + 1) % 2 == 0 {
        let n_digits = num.ilog10() + 1;
        let left = num / 10u64.pow(n_digits / 2) as usize;
        let right = num % (left * 10u64.pow(n_digits / 2) as usize);
        vec![left, right]
    } else {
        vec![num * 2024]
    }
}

pub fn process_part_one(input: &str) -> usize {
    let mut stones: Vec<usize> = input
        .trim()
        .split(" ")
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();

    for _ in 0..25 {
        stones = stones.iter().flat_map(|num| apply_rules(num)).collect();
    }

    stones.len()
}

pub fn process_part_two(input: &str) -> usize {
    let stones: Vec<usize> = input
        .trim()
        .split(" ")
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();

    let mut rule_mappings: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut num_counts: HashMap<usize, usize> = HashMap::new();
    for &stone in &stones {
        *num_counts.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..75 {
        let mut updates: HashMap<usize, usize> = HashMap::new();
        for (num, count) in num_counts.iter() {
            if let Some(mapping) = rule_mappings.get(num) {
                for &n in mapping {
                    updates
                        .entry(n)
                        .and_modify(|c| *c += count)
                        .or_insert(*count);
                }
            } else {
                let mapping = apply_rules(num);
                for &n in &mapping {
                    updates
                        .entry(n)
                        .and_modify(|c| *c += count)
                        .or_insert(*count);
                }
                rule_mappings.insert(*num, mapping);
            }
        }
        num_counts = updates;
    }

    num_counts.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_two(TEST_INPUT), 55312)
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 81)
    }
}
