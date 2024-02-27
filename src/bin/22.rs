use pathfinding::prelude::{astar, Matrix};
use std::collections::HashMap;

use glam::IVec2;

advent_of_code::solution!(22);

fn build_region_map(depth: usize, bounds: &IVec2, target: &IVec2) -> HashMap<IVec2, usize> {
    let mut erosion_level_map: HashMap<IVec2, usize> = HashMap::new();
    let mut region_map: HashMap<IVec2, usize> = HashMap::new();

    for y in 0..=bounds.y {
        for x in 0..=bounds.x {
            let geologic_index = if (x == 0 && y == 0) || (x == target.x && y == target.y) {
                0
            } else if y == 0 {
                x as usize * 16807
            } else if x == 0 {
                y as usize * 48271
            } else {
                erosion_level_map.get(&IVec2::new(x - 1, y)).unwrap()
                    * erosion_level_map.get(&IVec2::new(x, y - 1)).unwrap()
            };

            let erosion_level = (geologic_index + depth).rem_euclid(20183);

            erosion_level_map.insert(IVec2::new(x, y), erosion_level);

            region_map.insert(IVec2::new(x, y), erosion_level.rem_euclid(3));
        }
    }

    region_map
}

fn parse(input: &str) -> Option<(usize, IVec2)> {
    let mut lines = input.lines();
    let depth = lines.next()?.split_whitespace().nth(1)?.parse().ok()?;
    let mut parts = lines.next()?.split_whitespace().nth(1)?.split(',');
    let x = parts.next()?.parse().ok()?;
    let y = parts.next()?.parse().ok()?;
    Some((depth, IVec2::new(x, y)))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (depth, target) = parse(input)?;

    let region_map = build_region_map(depth, &target, &target);

    Some(region_map.values().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (depth, target) = parse(input)?;

    let corner = target + IVec2::new(100, 100);

    let region_map = build_region_map(depth, &corner, &target);

    let mut matrix = Matrix::new(corner.x as usize + 1, corner.y as usize + 1, 0);

    for y in 0..=corner.y as usize {
        for x in 0..=corner.x as usize {
            matrix[(x, y)] = match region_map.get(&IVec2::new(x as i32, y as i32)) {
                Some(0) => 0,
                Some(1) => 1,
                Some(2) => 2,
                _ => panic!("Unknown region type"),
            };
        }
    }

    const NEITHER: usize = 1;
    const TORCH: usize = 2;
    const GEAR: usize = 4;

    const ALLOWED: [usize; 3] = [TORCH + GEAR, NEITHER + GEAR, NEITHER + TORCH];

    astar(
        &((0, 0), TORCH),
        |&((x, y), eq)| {
            matrix
                .neighbours((x, y), false)
                .filter(|&(nx, ny)| ALLOWED[matrix[(nx, ny)]] & eq == eq)
                .map(|(nx, ny)| (((nx, ny), eq), 1))
                .chain(std::iter::once((((x, y), ALLOWED[matrix[(x, y)]] - eq), 7)))
                .collect::<Vec<_>>()
        },
        |&((x, y), _)| x.abs_diff(target.x as usize) + y.abs_diff(target.y as usize),
        |&((x, y), eq)| x == target.x as usize && y == target.y as usize && eq == TORCH,
    )
    .map(|(_, cost)| cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
