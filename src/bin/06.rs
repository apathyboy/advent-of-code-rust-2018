use std::{
    cmp::{max, min},
    collections::VecDeque,
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
    let x = a.x - b.x;
    let y = a.y - b.y;

    (x.unsigned_abs() + y.unsigned_abs()) as usize
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
) -> Option<Vec<IVec2>> {
    let dirs = [
        IVec2::new(0, -1),
        IVec2::new(0, 1),
        IVec2::new(-1, 0),
        IVec2::new(1, 0),
    ];

    let mut area = Vec::new();
    let mut to_visit = VecDeque::from([*coord]);

    while to_visit.len() > 0 {
        let next = to_visit.pop_front()?;

        if !is_in_bounds(&next, upper_left, lower_right) {
            return None;
        }

        if area.contains(&next) {
            continue;
        }

        area.push(next);

        to_visit.extend(dirs.iter().map(|dir| *dir + next).filter(|test_pos| {
            let closest_coord = find_closest_coord(test_pos, map);

            closest_coord.is_some() && closest_coord.unwrap() == *coord
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

pub fn part_one(input: &str) -> Option<usize> {
    let coordinates = parse(input);
    let (upper_left, lower_right) = find_bounds(&coordinates);

    coordinates
        .iter()
        .filter(|&c| {
            //confirm there is at least one coordinate further in each direction
            coordinates.iter().any(|other| c.x < other.x)
                && coordinates.iter().any(|other| c.x > other.x)
                && coordinates.iter().any(|other| c.y < other.y)
                && coordinates.iter().any(|other| c.y > other.y)
        })
        .filter_map(|coord| {
            let area = find_area(coord, &coordinates, &upper_left, &lower_right);

            area.map(|a| a.len())
        })
        .max()
}

pub fn part_two(input: &str) -> Option<usize> {
    let coords = parse(input);
    let (upper_left, lower_right) = find_bounds(&coords);

    let mut area = Vec::new();

    for y in upper_left.y..=lower_right.y {
        for x in upper_left.x..=lower_right.x {
            let coord = IVec2::new(x, y);

            if coords.iter().map(|c| manhattan(c, &coord)).sum::<usize>() < 10000 {
                area.push(coord);
            }
        }
    }

    Some(area.len())
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
