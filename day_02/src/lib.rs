fn is_report_safe(nums: &Vec<i32>) -> bool {
    if nums.is_sorted() | nums.iter().rev().is_sorted() {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        sorted_nums.windows(2).all(|p| {
            p[1] - p[0] > 0 && p[1] - p[0] < 4
        })
    } else {
        false
    }
}

pub fn process_part_one(input: &str) -> usize {
    input.lines().filter(|&line| {
        let nums: Vec<i32> = line.split(" ").map(|n| n.parse().unwrap()).collect();
        is_report_safe(&nums)
    }).count()
}

pub fn process_part_two(input: &str) -> usize {
    input.lines().filter(|&line| {
        let nums: Vec<i32> = line.split(" ").map(|n| n.parse().unwrap()).collect();
        if is_report_safe(&nums) {
            true
        } else {
            // Check if leaving any out will be safe
            for i in 0..nums.len() {
                let filter_nums: Vec<i32> = nums.iter().enumerate().filter_map(|(idx, v)| {
                    if idx != i {Some(*v)} else {None}
                }).collect();
                if is_report_safe(&filter_nums) {
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
