use std::collections::{HashMap, HashSet};

fn draw_grid(positions: Vec<(i32, i32)>, grid_w: i32, grid_h: i32) {
    for i in 0..grid_h {
        let mut row: Vec<&str> = Vec::new();
        for j in 0..grid_w {
            if positions.contains(&(j, i)) {
                row.push("X")
            } else {
                row.push(".")
            }
        }
        println!("{}\n", row.join(""))
    }
}

pub fn process_part_one(input: &str, grid_w: i32, grid_h: i32) -> usize {
    let mut quadrant_map: HashMap<(i32, i32), usize> =
        HashMap::from([((0, 0), 0), ((0, 1), 0), ((1, 0), 0), ((1, 1), 0)]);

    input.lines().for_each(|line| {
        let (pos, velocity) = line.split_once(" ").unwrap();
        let (pos_x, pos_y): (i32, i32) = pos[2..]
            .split_once(",")
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();
        let (vel_x, vel_y): (i32, i32) = velocity[2..]
            .split_once(",")
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();

        let (final_pos_x, final_pos_y) = (
            (pos_x + vel_x * 100).rem_euclid(grid_w),
            (pos_y + vel_y * 100).rem_euclid(grid_h),
        );

        if !(final_pos_x == grid_w / 2 || final_pos_y == grid_h / 2) {
            let quadrant_x: i32 = if final_pos_x < grid_w / 2 { 0 } else { 1 };
            let quadrant_y: i32 = if final_pos_y < grid_h / 2 { 0 } else { 1 };
            quadrant_map
                .entry((quadrant_x, quadrant_y))
                .and_modify(|c| *c += 1);
        }
    });

    quadrant_map.values().product()
}

pub fn process_part_two(input: &str, grid_w: i32, grid_h: i32) -> i64 {
    let robots: Vec<((i32, i32), (i32, i32))> = input
        .lines()
        .map(|line| {
            let (pos, velocity) = line.split_once(" ").unwrap();
            let pos: (i32, i32) = pos[2..]
                .split_once(",")
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();
            let vel: (i32, i32) = velocity[2..]
                .split_once(",")
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();
            (pos, vel)
        })
        .collect();
    'outer: for s in 0..10000 {
        let positions: Vec<(i32, i32)> = robots
            .iter()
            .map(|((pos_x, pos_y), (vel_x, vel_y))| {
                let (final_pos_x, final_pos_y) = (
                    (pos_x + vel_x * s).rem_euclid(grid_w),
                    (pos_y + vel_y * s).rem_euclid(grid_h),
                );

                (final_pos_x, final_pos_y)
            })
            .collect();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut to_visit: Vec<(i32, i32)> = vec![];
        let mut connected_components: Vec<Vec<(i32, i32)>> = vec![];
        for &pos in positions.iter() {
            if !visited.contains(&pos) {
                let mut component: Vec<(i32, i32)> = vec![];
                to_visit.push(pos);
                while let Some(x) = to_visit.pop() {
                    if !visited.contains(&x) {
                        component.push(x);
                        visited.insert(x);
                        [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().for_each(|d| {
                            let n = (x.0 + d.0, x.1 + d.1);
                            if positions.contains(&n) {
                                to_visit.push(n)
                            }
                        })
                    }
                }
                if component.len() > 100 {
                    draw_grid(positions, grid_w, grid_h);
                    println!("{}", s);
                    break 'outer;
                }
                connected_components.push(component);
            }
        }
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT, 11, 7), 12)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT, 11, 7), 10)
    }
}
