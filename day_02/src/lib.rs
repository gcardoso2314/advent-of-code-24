fn is_level_safe(nums: &mut Vec<i32>) -> bool {
    if nums.is_sorted() | nums.iter().rev().is_sorted() {
        nums.sort();
        nums.windows(2).all(|p| {
            p[1] - p[0] > 0 && p[1] - p[0] < 4
        })
    } else {
        false
    }
}

pub fn process_part_one(input: &str) -> usize {
    input.lines().filter(|&line| {
        let mut nums: Vec<i32> = line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect();
        is_level_safe(&mut nums)
    }).count()
}

pub fn process_part_two(input: &str) -> usize {
    input.lines().filter(|&line| {
        let mut nums: Vec<i32> = line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect();
        if is_level_safe(&mut nums) {
            true
        } else {
            // Check if leaving any out will be safe
            for i in 0..nums.len() {
                let mut filter_nums = nums.iter().enumerate().filter_map(|(idx, v)| {
                    if idx != i {Some(*v)} else {None}
                }).collect();
                if is_level_safe(&mut filter_nums) {
                    return true;
                }
            }
            false
        }
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 2)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 4)
    }
}
