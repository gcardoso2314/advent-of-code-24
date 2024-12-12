use itertools::*;
use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

struct Grid {
    plants: Vec<Vec<char>>,
}

impl Grid {
    fn grid_w(&self) -> i32 {
        self.plants[0].len() as i32
    }

    fn grid_h(&self) -> i32 {
        self.plants.len() as i32
    }

    fn get_neighbours(&self, pos: (i32, i32)) -> (Vec<(i32, i32)>, usize) {
        let mut neighbours: Vec<(i32, i32)> = Vec::new();
        let plant = &self.plants[pos.0 as usize][pos.1 as usize];
        for dir in DIRECTIONS.iter() {
            let (nrow, ncol) = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
            if nrow >= 0 && nrow < self.grid_h() && ncol >= 0 && ncol < self.grid_w() {
                let nplant = &self.plants[nrow as usize][ncol as usize];
                if plant == nplant {
                    neighbours.push((nrow, ncol));
                }
            }
        }

        let corners = match neighbours.len() {
            0 => 4,
            1 => 2,
            2 => {
                // If 2 neighbours then could have 0, 1, 2 corners depending on diagonals
                let (n1, n2) = (neighbours[0], neighbours[1]);
                if (n1.0 == n2.0 && n1.0 == pos.0) || (n1.1 == n2.1 && n1.1 == pos.1) {
                    0
                } else if pos.0 == n1.0 {
                    let diag = (n2.0, n1.1);
                    1 + (&self.plants[diag.0 as usize][diag.1 as usize] != plant) as usize
                } else {
                    let diag = (n1.0, n2.1);
                    1 + (&self.plants[diag.0 as usize][diag.1 as usize] != plant) as usize
                }
            }
            3 => {
                // If 3 neighbours then could have 0, 1, 2 corners depending on diagonals
                let (n1, n2, n3) = (neighbours[0], neighbours[1], neighbours[2]);
                let odd_one_out = if n1.0 == n2.0 || n1.1 == n2.1 {
                    n3
                } else if n2.0 == n3.0 || n2.1 == n3.1 {
                    n1
                } else {
                    n2
                };

                let (diag1, diag2) = if pos.0 == odd_one_out.0 {
                    (
                        (odd_one_out.0 - 1, odd_one_out.1),
                        (odd_one_out.0 + 1, odd_one_out.1),
                    )
                } else {
                    (
                        (odd_one_out.0, odd_one_out.1 - 1),
                        (odd_one_out.0, odd_one_out.1 + 1),
                    )
                };

                [diag1, diag2]
                    .iter()
                    .filter(|&&(x, y)| &self.plants[x as usize][y as usize] != plant)
                    .count()
            }
            4 => {
                // As many corners as there are plants of different types in diagonals
                let diags = [(1, 1), (-1, 1), (1, -1), (-1, -1)];
                let mut corners = 0;
                for diag in diags {
                    let (r, c) = (pos.0 + diag.0, pos.1 + diag.1);
                    if &self.plants[r as usize][c as usize] != plant {
                        corners += 1;
                    }
                }
                corners
            }
            _ => panic!("Shouldn't be more than 4 neighbours"),
        };

        (neighbours, corners)
    }

    fn dfs(&self, start: (i32, i32), visited: &mut HashSet<(i32, i32)>) -> (usize, usize, usize) {
        let mut perimeter = 0;
        let mut area = 0;
        let mut sides = 0;
        let mut to_visit = vec![start];

        while let Some(pos) = to_visit.pop() {
            if visited.contains(&pos) {
                continue;
            } else {
                area += 1;
                visited.insert(pos);

                let (nb, corners) = self.get_neighbours(pos);
                perimeter += 4 - nb.len();
                sides += corners;
                to_visit.extend(nb);
            }
        }

        (perimeter, area, sides)
    }
}

pub fn process_part_one(input: &str) -> usize {
    let grid: Grid = Grid {
        plants: input
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect(),
    };

    let to_visit: Vec<(i32, i32)> =
        Vec::from_iter((0..grid.grid_h()).cartesian_product(0..grid.grid_w()));
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut result = 0;
    for pos in to_visit.iter() {
        if visited.contains(pos) {
            continue;
        } else {
            let (p, a, _) = grid.dfs(*pos, &mut visited);
            result += p * a;
        }
    }
    result
}

pub fn process_part_two(input: &str) -> usize {
    let grid: Grid = Grid {
        plants: input
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect(),
    };

    let to_visit: Vec<(i32, i32)> =
        Vec::from_iter((0..grid.grid_h()).cartesian_product(0..grid.grid_w()));
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut result = 0;
    for pos in to_visit.iter() {
        if visited.contains(pos) {
            continue;
        } else {
            let (_, a, s) = grid.dfs(*pos, &mut visited);
            result += s * a;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 1930)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 1206)
    }
}
