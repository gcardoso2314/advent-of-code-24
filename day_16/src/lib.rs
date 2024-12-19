use std::collections::{BinaryHeap, HashMap, HashSet};

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

#[derive(Debug, Eq, Hash)]
struct Node {
    pos: (i32, i32),
    f_score: i32,
    g_score: i32,
    dir: Direction,
    path: Vec<((i32, i32), Direction)>,
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

fn get_start_and_end(
    grid: &Vec<Vec<Block>>,
    grid_h: usize,
    grid_w: usize,
) -> ((i32, i32), (i32, i32)) {
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

    (start_pos, end_pos)
}

fn find_best_paths(grid: Vec<Vec<Block>>) -> (HashSet<(i32, i32)>, i32) {
    let grid_h = grid.len();
    let grid_w = grid[0].len();

    let (start_pos, end_pos) = get_start_and_end(&grid, grid_h, grid_w);

    let mut to_visit: BinaryHeap<Node> = BinaryHeap::from([Node {
        pos: start_pos,
        f_score: -l1_distance(&start_pos, &end_pos),
        g_score: 0,
        dir: Direction::East,
        path: vec![(start_pos, Direction::East)],
    }]);
    let mut g_scores: HashMap<((i32, i32), Direction), i32> = HashMap::new();
    let mut best_tiles: HashSet<(i32, i32)> = HashSet::new();
    let mut lowest_score = 0;
    while let Some(node) = to_visit.pop() {
        if node.pos == end_pos {
            if lowest_score == 0 {
                lowest_score = -node.g_score;
            }

            best_tiles.extend(node.path.iter().map(|&(n, _)| n));
        } else {
            if let Some(g) = g_scores.get(&(node.pos, node.dir)) {
                if node.g_score < *g {
                    // No point exploring this further since we've visited with lower score
                    continue;
                }
            }
            g_scores.insert((node.pos, node.dir), node.g_score);

            let spos = node.dir.move_straight(&node.pos);
            if spos.0 >= 0 && spos.0 < grid_h as i32 && spos.1 >= 0 && spos.1 < grid_w as i32 {
                match grid[spos.0 as usize][spos.1 as usize] {
                    Block::FreeSpace | Block::End | Block::Start => {
                        let g_score = node.g_score - 1;
                        if lowest_score > 0 && -g_score > lowest_score {
                            continue;
                        }
                        let mut path = node.path.clone();
                        path.push((spos, node.dir));
                        to_visit.push(Node {
                            pos: spos,
                            f_score: g_score - l1_distance(&spos, &end_pos),
                            g_score,
                            dir: node.dir,
                            path,
                        })
                    }
                    _ => (),
                }
            }

            let rdir = node.dir.rotate_right();
            match grid[node.pos.0 as usize][node.pos.1 as usize] {
                Block::FreeSpace | Block::End | Block::Start => {
                    let g_score = node.g_score - 1000;
                    if lowest_score > 0 && -g_score > lowest_score {
                        continue;
                    }
                    let mut path = node.path.clone();
                    path.push((node.pos, rdir));
                    to_visit.push(Node {
                        pos: node.pos,
                        f_score: g_score - l1_distance(&node.pos, &end_pos),
                        g_score,
                        dir: rdir,
                        path,
                    })
                }
                _ => (),
            }

            let ldir = node.dir.rotate_left();
            match grid[node.pos.0 as usize][node.pos.1 as usize] {
                Block::FreeSpace | Block::End | Block::Start => {
                    let g_score = node.g_score - 1000;
                    if lowest_score > 0 && -g_score > lowest_score {
                        continue;
                    }
                    let mut path = node.path.clone();
                    path.push((node.pos, ldir));
                    to_visit.push(Node {
                        pos: node.pos,
                        f_score: g_score - l1_distance(&node.pos, &end_pos),
                        g_score,
                        dir: ldir,
                        path,
                    })
                }
                _ => (),
            }
        }
    }

    (best_tiles, lowest_score)
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

    let best_paths = find_best_paths(grid);
    best_paths.1
}

pub fn process_part_two(input: &str) -> usize {
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

    let best_paths = find_best_paths(grid);
    best_paths.0.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 7036)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 45)
    }
}
