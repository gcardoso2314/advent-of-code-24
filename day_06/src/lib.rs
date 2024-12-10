use std::collections::{hash_map::HashMap, hash_set::HashSet};

#[derive(Debug, Clone)]
struct Grid {
    row_obstacles: HashMap<usize, Vec<usize>>,
    col_obstacles: HashMap<usize, Vec<usize>>,
    height: usize,
    width: usize,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point {
    row: i32,
    col: i32,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Grid {
    fn from(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let mut row_obstacles: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut col_obstacles: HashMap<usize, Vec<usize>> = HashMap::new();
        lines.iter().enumerate().for_each(|(i, line)| {
            line.chars().enumerate().for_each(|(j, c)| {
                if c == '#' {
                    row_obstacles
                        .entry(i)
                        .and_modify(|v| v.push(j))
                        .or_insert(vec![j]);
                    col_obstacles
                        .entry(j)
                        .and_modify(|v| v.push(i))
                        .or_insert(vec![i]);
                }
            })
        });
        Grid {
            row_obstacles,
            col_obstacles,
            height: lines.len(),
            width: lines[0].len(),
        }
    }

    fn find_next_obstacle(&self, pos: &Point, dir: &Direction) -> Option<Point> {
        match dir {
            Direction::Up => {
                let obstacles = self.col_obstacles.get(&(pos.col as usize))?;
                let obs_row = obstacles
                    .iter()
                    .filter_map(|&i| if i < pos.row as usize { Some(i) } else { None })
                    .max()?;
                Some(Point {
                    row: obs_row as i32,
                    col: pos.col,
                })
            }
            Direction::Down => {
                let obstacles = self.col_obstacles.get(&(pos.col as usize))?;
                let obs_row = obstacles
                    .iter()
                    .filter_map(|&i| if i > pos.row as usize { Some(i) } else { None })
                    .min()?;
                Some(Point {
                    row: obs_row as i32,
                    col: pos.col,
                })
            }
            Direction::Left => {
                let obstacles = self.row_obstacles.get(&(pos.row as usize))?;
                let obs_col = obstacles
                    .iter()
                    .filter_map(|&i| if i < pos.col as usize { Some(i) } else { None })
                    .max()?;
                Some(Point {
                    row: pos.row,
                    col: obs_col as i32,
                })
            }
            Direction::Right => {
                let obstacles = self.row_obstacles.get(&(pos.row as usize))?;
                let obs_col = obstacles
                    .iter()
                    .filter_map(|&i| if i > pos.col as usize { Some(i) } else { None })
                    .min()?;
                Some(Point {
                    row: pos.row,
                    col: obs_col as i32,
                })
            }
        }
    }

    fn set_obstacle(&mut self, pos: &Point) {
        self.row_obstacles
            .entry(pos.row as usize)
            .and_modify(|v| v.push(pos.col as usize))
            .or_insert(vec![pos.col as usize]);
        self.col_obstacles
            .entry(pos.col as usize)
            .and_modify(|v| v.push(pos.row as usize))
            .or_insert(vec![pos.row as usize]);
    }
}

pub fn process_part_one(input: &str) -> usize {
    let grid = Grid::from(input);
    let mut start_pos = Point { row: 0, col: 0 };
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '^' {
                start_pos = Point {
                    row: i as i32,
                    col: j as i32,
                }
            }
        })
    });

    let mut visited: HashSet<Point> = HashSet::new();
    let mut pos = start_pos;
    let mut dir: Direction = Direction::Up;

    loop {
        if let Some(obs) = grid.find_next_obstacle(&pos, &dir) {
            match dir {
                Direction::Left => {
                    for i in obs.col + 1..=pos.col {
                        visited.insert(Point {
                            row: pos.row,
                            col: i,
                        });
                    }
                    pos = Point {
                        row: pos.row,
                        col: obs.col + 1,
                    }
                }
                Direction::Right => {
                    for i in pos.col..obs.col {
                        visited.insert(Point {
                            row: pos.row,
                            col: i,
                        });
                    }
                    pos = Point {
                        row: pos.row,
                        col: obs.col - 1,
                    }
                }
                Direction::Up => {
                    for i in obs.row + 1..=pos.row {
                        visited.insert(Point {
                            row: i,
                            col: pos.col,
                        });
                    }
                    pos = Point {
                        row: obs.row + 1,
                        col: pos.col,
                    }
                }
                Direction::Down => {
                    for i in pos.row..obs.row {
                        visited.insert(Point {
                            row: i,
                            col: pos.col,
                        });
                    }
                    pos = Point {
                        row: obs.row - 1,
                        col: pos.col,
                    }
                }
            }
            dir = dir.rotate();
        } else {
            // Leaving the grid, visit the remaining cells
            match dir {
                Direction::Left => {
                    for i in 0..=pos.col {
                        visited.insert(Point {
                            row: pos.row,
                            col: i,
                        });
                    }
                }
                Direction::Right => {
                    for i in pos.col..grid.width as i32 {
                        visited.insert(Point {
                            row: pos.row,
                            col: i,
                        });
                    }
                }
                Direction::Up => {
                    for i in 0..=pos.row {
                        visited.insert(Point {
                            row: i,
                            col: pos.col,
                        });
                    }
                }
                Direction::Down => {
                    for i in pos.row..grid.height as i32 {
                        visited.insert(Point {
                            row: i,
                            col: pos.col,
                        });
                    }
                }
            }
            break;
        }
    }

    visited.len()
}

fn test_cycle(start_pos: Point, new_obstacle: Point, mut grid: Grid) -> bool {
    grid.set_obstacle(&new_obstacle);
    let mut dir = Direction::Up;
    let mut pos = start_pos;
    let mut visited: HashSet<(Point, Direction)> = HashSet::new();

    loop {
        if let Some(obs) = grid.find_next_obstacle(&pos, &dir) {
            if visited.contains(&(obs, dir)) {
                return true;
            }
            visited.insert((obs, dir));
            pos = match dir {
                Direction::Left => Point {
                    row: pos.row,
                    col: obs.col + 1,
                },
                Direction::Right => Point {
                    row: pos.row,
                    col: obs.col - 1,
                },
                Direction::Up => Point {
                    row: obs.row + 1,
                    col: pos.col,
                },
                Direction::Down => Point {
                    row: obs.row - 1,
                    col: pos.col,
                },
            };
            dir = dir.rotate();
        } else {
            return false;
        }
    }
}

pub fn process_part_two(input: &str) -> usize {
    let grid = Grid::from(input);
    let mut start_pos = Point { row: 0, col: 0 };
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '^' {
                start_pos = Point {
                    row: i as i32,
                    col: j as i32,
                }
            }
        })
    });

    let mut new_obstacles: HashSet<Point> = HashSet::new();
    let mut pos = start_pos;
    let mut dir: Direction = Direction::Up;

    loop {
        if let Some(obs) = grid.find_next_obstacle(&pos, &dir) {
            match dir {
                Direction::Left => {
                    for i in obs.col + 1..=pos.col {
                        let new_obstacle = Point {
                            row: pos.row,
                            col: i,
                        };
                        if test_cycle(start_pos, new_obstacle, grid.clone()) {
                            new_obstacles.insert(new_obstacle);
                        }
                    }
                    pos = Point {
                        row: pos.row,
                        col: obs.col + 1,
                    }
                }
                Direction::Right => {
                    for i in pos.col..obs.col {
                        let new_obstacle = Point {
                            row: pos.row,
                            col: i,
                        };
                        if test_cycle(start_pos, new_obstacle, grid.clone()) {
                            new_obstacles.insert(new_obstacle);
                        }
                    }
                    pos = Point {
                        row: pos.row,
                        col: obs.col - 1,
                    }
                }
                Direction::Up => {
                    for i in obs.row + 1..=pos.row {
                        let new_obstacle = Point {
                            row: i,
                            col: pos.col,
                        };
                        if test_cycle(start_pos, new_obstacle, grid.clone()) {
                            new_obstacles.insert(new_obstacle);
                        }
                    }
                    pos = Point {
                        row: obs.row + 1,
                        col: pos.col,
                    }
                }
                Direction::Down => {
                    for i in pos.row..obs.row {
                        let new_obstacle = Point {
                            row: i,
                            col: pos.col,
                        };
                        if test_cycle(start_pos, new_obstacle, grid.clone()) {
                            new_obstacles.insert(new_obstacle);
                        }
                    }
                    pos = Point {
                        row: obs.row - 1,
                        col: pos.col,
                    }
                }
            }
            dir = dir.rotate();
        } else {
            // Leaving the grid, visit the remaining cells
            match dir {
                Direction::Left => {
                    for i in 0..=pos.col {
                        let new_obstacle = Point {
                            row: pos.row,
                            col: i,
                        };
                        if test_cycle(start_pos, new_obstacle, grid.clone()) {
                            new_obstacles.insert(new_obstacle);
                        }
                    }
                }
                Direction::Right => {
                    for i in pos.col..grid.width as i32 {
                        let new_obstacle = Point {
                            row: pos.row,
                            col: i,
                        };
                        if test_cycle(start_pos, new_obstacle, grid.clone()) {
                            new_obstacles.insert(new_obstacle);
                        }
                    }
                }
                Direction::Up => {
                    for i in 0..=pos.row {
                        let new_obstacle = Point {
                            row: i,
                            col: pos.col,
                        };
                        if test_cycle(start_pos, new_obstacle, grid.clone()) {
                            new_obstacles.insert(new_obstacle);
                        }
                    }
                }
                Direction::Down => {
                    for i in pos.row..grid.height as i32 {
                        let new_obstacle = Point {
                            row: i,
                            col: pos.col,
                        };
                        if test_cycle(start_pos, new_obstacle, grid.clone()) {
                            new_obstacles.insert(new_obstacle);
                        }
                    }
                }
            }
            break;
        }
    }

    new_obstacles.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 41)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 6)
    }
}
