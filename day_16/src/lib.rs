use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn move_straight(&self, pos: &(i32, i32)) -> (i32, i32) {
        match self {
            Direction::North => (pos.0 + -1, pos.1 + 0),
            Direction::East => (pos.0 + 0, pos.1 + 1),
            Direction::South => (pos.0 + 1, pos.1 + 0),
            Direction::West => (pos.0 + 0, pos.1 + -1),
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn rotate_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Block {
    Wall,
    FreeSpace,
    Start,
    End,
}

#[derive(Debug, Eq, Hash, Clone, Copy)]
struct Node {
    pos: (i32, i32),
    f_score: i32,
    g_score: i32,
    dir: Direction,
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
        self.pos == other.pos && self.dir == other.dir
    }
}

fn l1_distance(pos: &(i32, i32), end: &(i32, i32)) -> i32 {
    (pos.0 - end.0).abs() + (pos.1 - end.1).abs()
}

pub fn process_part_one(input: &str) -> i32 {
    let grid: Vec<Vec<Block>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Block::Wall,
                    '.' => Block::FreeSpace,
                    'S' => Block::Start,
                    'E' => Block::End,
                    _ => panic!("Unrecognised character"),
                })
                .collect()
        })
        .collect();

    let grid_h = grid.len();
    let grid_w = grid[0].len();

    let mut start_pos: (i32, i32) = (0, 0);
    let mut end_pos: (i32, i32) = (0, 0);
    for i in 0..grid_h {
        for j in 0..grid_w {
            if grid[i][j] == Block::Start {
                start_pos = (i as i32, j as i32);
            } else if grid[i][j] == Block::End {
                end_pos = (i as i32, j as i32);
            }
        }
    }

    let mut to_visit: BinaryHeap<Node> = BinaryHeap::from([Node {
        pos: start_pos,
        f_score: -l1_distance(&start_pos, &end_pos),
        g_score: 0,
        dir: Direction::East,
    }]);
    let mut visited: HashSet<((i32, i32), Direction)> = HashSet::new();
    while let Some(node) = to_visit.pop() {
        if node.pos == end_pos {
            return -node.g_score;
        }
        if !visited.contains(&(node.pos, node.dir)) {
            visited.insert((node.pos, node.dir));

            let spos = node.dir.move_straight(&node.pos);
            if spos.0 >= 0 && spos.0 < grid_h as i32 && spos.1 >= 0 && spos.1 < grid_w as i32 {
                match grid[spos.0 as usize][spos.1 as usize] {
                    Block::FreeSpace | Block::End | Block::Start => {
                        let g_score = node.g_score - 1;
                        to_visit.push(Node {
                            pos: spos,
                            f_score: g_score - l1_distance(&spos, &end_pos),
                            g_score,
                            dir: node.dir,
                        })
                    }
                    _ => (),
                }
            }

            let rdir = node.dir.rotate_right();
            match grid[node.pos.0 as usize][node.pos.1 as usize] {
                Block::FreeSpace | Block::End | Block::Start => {
                    let g_score = node.g_score - 1000;
                    to_visit.push(Node {
                        pos: node.pos,
                        f_score: g_score - l1_distance(&node.pos, &end_pos),
                        g_score,
                        dir: rdir,
                    })
                }
                _ => (),
            }

            let ldir = node.dir.rotate_left();
            match grid[node.pos.0 as usize][node.pos.1 as usize] {
                Block::FreeSpace | Block::End | Block::Start => {
                    let g_score = node.g_score - 1000;
                    to_visit.push(Node {
                        pos: node.pos,
                        f_score: g_score - l1_distance(&node.pos, &end_pos),
                        g_score,
                        dir: ldir,
                    })
                }
                _ => (),
            }
        }
    }

    panic!("NEVER HIT THE END!!!")
}

pub fn process_part_two(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 11048)
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 9021)
    }
}
