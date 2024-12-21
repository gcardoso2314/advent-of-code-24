use std::collections::{BinaryHeap, HashSet};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
struct Grid {
    size: usize,
    corrupted: Vec<(i32, i32)>,
}

#[derive(Debug, Eq, Hash)]
struct Node {
    pos: (i32, i32),
    g_score: i32,
    f_score: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f_score.cmp(&other.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Grid {
    fn new(size: usize, corrupted_bytes: &[(i32, i32)]) -> Self {
        Grid {
            size,
            corrupted: Vec::from(corrupted_bytes),
        }
    }

    fn valid_move(&self, pos: &(i32, i32)) -> bool {
        let in_grid =
            pos.0 >= 0 && pos.0 < self.size as i32 && pos.1 >= 0 && pos.1 < self.size as i32;
        let not_corrupted = !self.corrupted.contains(pos);
        in_grid && not_corrupted
    }

    fn find_shortest_path(&self) -> i32 {
        let end_pos: (i32, i32) = (self.size as i32 - 1, self.size as i32 - 1);
        let mut to_visit: BinaryHeap<Node> = BinaryHeap::from([Node {
            pos: (0, 0),
            f_score: -l1_distance(&(0, 0), &end_pos),
            g_score: 0,
        }]);

        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        while let Some(node) = to_visit.pop() {
            if node.pos == end_pos {
                return -node.g_score;
            }
            if !visited.contains(&node.pos) {
                visited.insert(node.pos);
                DIRECTIONS.iter().for_each(|d| {
                    let new_pos = (node.pos.0 + d.0, node.pos.1 + d.1);
                    if self.valid_move(&new_pos) {
                        to_visit.push(Node {
                            pos: new_pos,
                            g_score: node.g_score - 1,
                            f_score: node.g_score - 1 - l1_distance(&new_pos, &end_pos),
                        })
                    }
                });
            }
        }

        panic!("END POSITION NOT FOUND!!")
    }

    fn is_end_reachable(&self) -> bool {
        let start_pos: (i32, i32) = (0, 0);
        let end_pos: (i32, i32) = (self.size as i32 - 1, self.size as i32 - 1);
        let mut to_visit: BinaryHeap<Node> = BinaryHeap::from([Node {
            pos: start_pos,
            f_score: -l1_distance(&start_pos, &end_pos),
            g_score: 0,
        }]);
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        while let Some(node) = to_visit.pop() {
            if node.pos == end_pos {
                return true;
            }
            if !visited.contains(&node.pos) {
                visited.insert(node.pos);
                DIRECTIONS.iter().for_each(|d| {
                    let new_pos = (node.pos.0 + d.0, node.pos.1 + d.1);
                    if self.valid_move(&new_pos) {
                        to_visit.push(Node {
                            pos: new_pos,
                            g_score: node.g_score - 1,
                            f_score: node.g_score - 1 - l1_distance(&new_pos, &end_pos),
                        })
                    }
                });
            }
        }

        false
    }
}

fn l1_distance(pos: &(i32, i32), end: &(i32, i32)) -> i32 {
    (pos.0 - end.0).abs() + (pos.1 - end.1).abs()
}

pub fn process_part_one(input: &str, grid_size: usize, n_corrupt: usize) -> i32 {
    let corrupted_bytes: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            line.split_once(",")
                .map(|n| (n.0.parse().unwrap(), n.1.parse().unwrap()))
                .unwrap()
        })
        .collect();

    let grid = Grid::new(grid_size, &corrupted_bytes[0..n_corrupt]);

    grid.find_shortest_path()
}

pub fn process_part_two(input: &str, grid_size: usize) -> String {
    let corrupted_bytes: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            line.split_once(",")
                .map(|n| (n.0.parse().unwrap(), n.1.parse().unwrap()))
                .unwrap()
        })
        .collect();

    for n_corrupt in 0..corrupted_bytes.len() {
        println!("Running for {n_corrupt} bytes");
        let grid = Grid::new(grid_size, &corrupted_bytes[0..n_corrupt]);
        if !grid.is_end_reachable() {
            return format!(
                "{},{}",
                corrupted_bytes[n_corrupt - 1].0,
                corrupted_bytes[n_corrupt - 1].1
            );
        }
    }

    panic!("RESULT NOT FOUND, SOMETHING WENT WRONG")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT, 7, 12), 22);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT, 7), "6,1")
    }
}
