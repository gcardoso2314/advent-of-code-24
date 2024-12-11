use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

fn move_dir(pos: (i32, i32), dir: (i32, i32), grid_h: usize, grid_w: usize) -> Option<(i32, i32)> {
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if new_pos.0 >= 0 && new_pos.0 < grid_h as i32 && new_pos.1 >= 0 && new_pos.1 < grid_w as i32 {
        Some(new_pos)
    } else {
        None
    }
}

fn score_trailhead(start: (i32, i32), grid: &Vec<Vec<u32>>) -> usize {
    let grid_w = grid[0].len();
    let grid_h = grid.len();
    let mut to_visit: Vec<(i32, i32)> = vec![start];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut peak_count = 0;

    while let Some(p) = to_visit.pop() {
        if visited.contains(&p) {
            continue;
        }

        visited.insert(p);
        let val = grid[p.0 as usize][p.1 as usize];
        if val == 9 {
            peak_count += 1
        } else {
            for dir in DIRECTIONS {
                if let Some(n) = move_dir(p, dir, grid_h, grid_w) {
                    if grid[n.0 as usize][n.1 as usize] == val + 1 {
                        to_visit.push(n);
                    }
                }
            }
        }
    }

    peak_count
}

fn rate_trailhead(start: (i32, i32), grid: &Vec<Vec<u32>>) -> usize {
    let grid_w = grid[0].len();
    let grid_h = grid.len();
    let mut to_visit: Vec<(i32, i32)> = vec![start];
    let mut paths = 0;

    while let Some(p) = to_visit.pop() {
        let val = grid[p.0 as usize][p.1 as usize];
        if val == 9 {
            paths += 1
        } else {
            for dir in DIRECTIONS {
                if let Some(n) = move_dir(p, dir, grid_h, grid_w) {
                    if grid[n.0 as usize][n.1 as usize] == val + 1 {
                        to_visit.push(n);
                    }
                }
            }
        }
    }

    paths
}

pub fn process_part_one(input: &str) -> usize {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let mut result = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 0 {
                result += score_trailhead((i as i32, j as i32), &grid)
            }
        }
    }

    result
}

pub fn process_part_two(input: &str) -> usize {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let mut result = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 0 {
                result += rate_trailhead((i as i32, j as i32), &grid)
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 36)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 81)
    }
}
