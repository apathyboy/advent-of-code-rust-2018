use itertools::Itertools;
use pathfinding::prelude::bfs;
use std::collections::HashMap;

use glam::IVec2;

advent_of_code::solution!(22);

fn build_region_map(depth: usize, target: &IVec2) -> HashMap<IVec2, usize> {
    let mut erosion_level_map: HashMap<IVec2, usize> = HashMap::new();
    let mut region_map: HashMap<IVec2, usize> = HashMap::new();

    for y in 0..=target.y {
        for x in 0..=target.x {
            let geologic_index = if x == 0 && y == 0 {
                0
            } else if x == target.x && y == target.y {
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

    let region_map = build_region_map(depth, &target);

    Some(region_map.values().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (depth, target) = parse(input)?;

    let region_map = build_region_map(depth, &target);

    let start = (IVec2::new(0, 0), 1);
    let goal = (target, 1);
    let goal_alt = (target, 2);

    let result = bfs(
        &start,
        |&(pos, tool)| {
            let mut next = vec![];
            for &new_tool in &[0, 1, 2] {
                if new_tool != tool && region_map[&pos] != new_tool {
                    next.push((pos, new_tool));
                }
            }

            for &dir in &[
                IVec2::new(0, 1),
                IVec2::new(1, 0),
                IVec2::new(0, -1),
                IVec2::new(-1, 0),
            ] {
                let new_pos = pos + dir;
                if new_pos.x < 0 || new_pos.y < 0 {
                    continue;
                }
                if region_map.contains_key(&new_pos) && region_map[&new_pos] != tool {
                    next.push((new_pos, tool));
                }
            }
            next
        },
        |&cur| cur == goal || cur == goal_alt,
    )?;

    dbg!(&result);

    let tool_changes = result.iter().map(|(_, tool)| tool).unique().count() * 7;
    if result.last().unwrap().1 == 1 {
        Some(result.len() - 1 + tool_changes)
    } else {
        Some(result.len() - 1 + tool_changes + 7)
    }
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
