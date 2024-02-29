use std::collections::HashSet;

use glam::IVec4;

advent_of_code::solution!(25);

fn distance(a: IVec4, b: IVec4) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs() + (a.w - b.w).abs()
}

fn parse_input(input: &str) -> Vec<(IVec4, usize)> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut parts = line.trim().split(',').map(|s| s.parse().unwrap());
            (
                IVec4::new(
                    parts.next().unwrap(),
                    parts.next().unwrap(),
                    parts.next().unwrap(),
                    parts.next().unwrap(),
                ),
                i,
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = parse_input(input);

    let mut constellations = Vec::new();
    while !input.is_empty() {
        let mut constellation = HashSet::new();
        let mut to_check = vec![input.remove(0)];
        while !to_check.is_empty() {
            let (point, index) = to_check.remove(0);
            if !constellation.insert(index) {
                continue;
            }
            for i in (0..input.len()).rev() {
                if distance(point, input[i].0) <= 3 {
                    to_check.push(input.remove(i));
                }
            }
        }
        constellations.push(constellation);
    }

    Some(constellations.len())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
