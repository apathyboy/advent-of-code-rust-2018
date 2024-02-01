use glam::IVec2;
use itertools::Itertools;
use std::cmp::{max, min};

advent_of_code::solution!(3);

#[derive(Debug, PartialEq)]
struct Rectangle {
    pos: IVec2,
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            pos: IVec2::new(x, y),
            width,
            height,
        }
    }
}

fn parse_rectangle(line: &str) -> Option<Rectangle> {
    let (_, rect) = line.split_once(" @ ")?;
    let (pos, dimensions) = rect.split_once(": ")?;

    let (x, y) = pos.split_once(',')?;
    let (width, height) = dimensions.split_once('x')?;

    Some(Rectangle::new(
        x.parse().ok()?,
        y.parse().ok()?,
        width.parse().ok()?,
        height.parse().ok()?,
    ))
}

fn parse(input: &str) -> Vec<Rectangle> {
    input.lines().filter_map(parse_rectangle).collect()
}

fn overlap_rectangle(a: &Rectangle, b: &Rectangle) -> Option<Rectangle> {
    if a.pos.x < b.pos.x + b.width
        && a.pos.y < b.pos.y + b.height
        && b.pos.x < a.pos.x + a.width
        && b.pos.y < a.pos.y + a.height
    {
        let x = max(a.pos.x, b.pos.x);
        let y = max(a.pos.y, b.pos.y);
        let w = min(a.pos.x + a.width, b.pos.x + b.width) - x;
        let h = min(a.pos.y + a.height, b.pos.y + b.height) - y;

        Some(Rectangle::new(x, y, w, h))
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let rects = parse(input);

    let overlapped_squares = rects
        .iter()
        .combinations(2)
        .filter_map(|combo| overlap_rectangle(combo[0], combo[1]))
        .flat_map(|overlap| {
            (overlap.pos.y..overlap.pos.y + overlap.height).flat_map(move |y| {
                (overlap.pos.x..overlap.pos.x + overlap.width).map(move |x| IVec2::new(x, y))
            })
        })
        .unique()
        .count();

    Some(overlapped_squares)
}

pub fn part_two(input: &str) -> Option<usize> {
    let rects = parse(input);

    let (id, _) = rects.iter().enumerate().find(|r| {
        !rects
            .iter()
            .filter(|&other| *other != *r.1)
            .any(|other| overlap_rectangle(r.1, other).is_some())
    })?;

    Some(id + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap_rectangle() {
        let a = Rectangle::new(1, 3, 4, 4);
        let b = Rectangle::new(3, 1, 4, 4);

        let result = overlap_rectangle(&a, &b);

        assert_eq!(result, Some(Rectangle::new(3, 3, 2, 2)));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
