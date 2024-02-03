use std::{
    cmp::{max, min},
    collections::{HashSet, VecDeque},
};

use glam::IVec2;
use itertools::Itertools;

advent_of_code::solution!(6);

fn parse_point(s: &str) -> Option<IVec2> {
    let (x, y) = s.split_once(", ")?;

    let x_fromstr = x.parse().ok()?;
    let y_fromstr = y.parse().ok()?;

    Some(IVec2::new(x_fromstr, y_fromstr))
}

fn parse(input: &str) -> Vec<IVec2> {
    input.lines().filter_map(parse_point).collect()
}

fn manhattan(a: &IVec2, b: &IVec2) -> usize {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as usize
}

fn find_closest_coord(coord: &IVec2, map: &[IVec2]) -> Option<IVec2> {
    let closest = map
        .iter()
        .map(|c| (c, manhattan(c, coord)))
        .min_set_by(|(_, d1), (_, d2)| d1.cmp(d2));

    if closest.len() != 1 {
        return None;
    }

    Some(*closest[0].0)
}

fn is_in_bounds(coord: &IVec2, upper_left: &IVec2, lower_right: &IVec2) -> bool {
    coord.x > upper_left.x
        && coord.x < lower_right.x
        && coord.y > upper_left.y
        && coord.y < lower_right.y
}

fn find_area(
    coord: &IVec2,
    map: &[IVec2],
    upper_left: &IVec2,
    lower_right: &IVec2,
) -> Option<HashSet<IVec2>> {
    let dirs = [
        IVec2::new(0, -1),
        IVec2::new(0, 1),
        IVec2::new(-1, 0),
        IVec2::new(1, 0),
    ];

    let mut area = HashSet::new();
    let mut to_visit = VecDeque::from([*coord]);

    while let Some(current) = to_visit.pop_front() {
        if !is_in_bounds(&current, upper_left, lower_right) {
            return None;
        }

        if !area.insert(current) {
            continue;
        }

        to_visit.extend(dirs.iter().map(|&dir| dir + current).filter(|&next_coord| {
            find_closest_coord(&next_coord, map).map_or(false, |closest| closest == *coord)
        }));
    }

    Some(area)
}

fn find_bounds(map: &[IVec2]) -> (IVec2, IVec2) {
    let mut upper_left = IVec2::MAX;
    let mut lower_right = IVec2::MIN;

    for coord in map.iter() {
        upper_left.x = min(upper_left.x, coord.x);
        upper_left.y = min(upper_left.y, coord.y);

        lower_right.x = max(lower_right.x, coord.x);
        lower_right.y = max(lower_right.y, coord.y);
    }

    (upper_left, lower_right)
}

fn find_safe_region_size(
    coords: &[IVec2],
    upper_left: &IVec2,
    lower_right: &IVec2,
    constraint: usize,
) -> usize {
    (upper_left.y..=lower_right.y)
        .flat_map(|y| {
            (upper_left.x..=lower_right.x)
                .map(move |x| IVec2::new(x, y))
                .filter(|coord| {
                    // Check if the total Manhattan distance to all points is less than 10000
                    coords.iter().map(|c| manhattan(c, coord)).sum::<usize>() < constraint
                })
        })
        .count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let coordinates = parse(input);
    let (upper_left, lower_right) = find_bounds(&coordinates);

    coordinates
        .iter()
        .filter(|&c| {
            !(c.x == upper_left.x
                || c.x == lower_right.x
                || c.y == upper_left.y
                || c.y == lower_right.y)
        })
        .filter_map(|coord| {
            find_area(coord, &coordinates, &upper_left, &lower_right).map(|a| a.len())
        })
        .max()
}

pub fn part_two(input: &str) -> Option<usize> {
    let coords = parse(input);
    let (upper_left, lower_right) = find_bounds(&coords);

    Some(find_safe_region_size(
        &coords,
        &upper_left,
        &lower_right,
        10000,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(17));
    }

    #[test]
    fn test_find_safe_region_size() {
        let coords = parse(&advent_of_code::template::read_file("examples", DAY));
        let (upper_left, lower_right) = find_bounds(&coords);
        let result = find_safe_region_size(&coords, &upper_left, &lower_right, 32);
        assert_eq!(result, 16);
    }
}
