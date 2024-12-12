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

fn run_blinks(stones: Vec<usize>, n: usize) -> usize {
    let mut num_counts: HashMap<usize, usize> = HashMap::new();
    for &stone in &stones {
        *num_counts.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..n {
        let mut updates: HashMap<usize, usize> = HashMap::new();
        for (num, count) in num_counts.iter() {
            let mapping = apply_rules(num);
            for &n in &mapping {
                updates
                    .entry(n)
                    .and_modify(|c| *c += count)
                    .or_insert(*count);
            }
        }
        num_counts = updates;
    }

    num_counts.values().sum()
}

pub fn process_part_one(input: &str) -> usize {
    let stones: Vec<usize> = input
        .trim()
        .split(" ")
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();

    run_blinks(stones, 25)
}

pub fn process_part_two(input: &str) -> usize {
    let stones: Vec<usize> = input
        .trim()
        .split(" ")
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();

    run_blinks(stones, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 55312)
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 81)
    }
}
