use std::collections::BTreeMap;

use glam::IVec3;

advent_of_code::solution!(23);

#[derive(Debug)]
struct Nanobot {
    position: IVec3,
    radius: u32,
}

impl Nanobot {
    fn new(position: IVec3, radius: u32) -> Self {
        Self { position, radius }
    }
}

fn manhattan(a: &IVec3, b: &IVec3) -> u32 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y) + a.z.abs_diff(b.z)
}

fn parse_nanobot(line: &str) -> Option<Nanobot> {
    let (pos, radius) = line.split_once(", ")?;

    let elements = pos[5..pos.len() - 1]
        .split(',')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<_>>();

    let radius = radius[2..].parse::<u32>().ok()?;

    Some(Nanobot::new(
        IVec3::new(elements[0], elements[1], elements[2]),
        radius,
    ))
}

fn parse(input: &str) -> Vec<Nanobot> {
    input.lines().filter_map(parse_nanobot).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let nanobots = parse(input);

    let strongest = nanobots.iter().max_by(|a, b| a.radius.cmp(&b.radius))?;

    let in_range = nanobots
        .iter()
        .filter(|nanobot| manhattan(&strongest.position, &nanobot.position) <= strongest.radius)
        .count();

    Some(in_range)
}

pub fn part_two(input: &str) -> Option<u32> {
    let nanobots = parse(input);
    let mut dist = BTreeMap::new();
    for bot in nanobots {
        let d = bot.position.x + bot.position.y + bot.position.z;
        *dist.entry(d - bot.radius as i32).or_insert(0) += 1;
        *dist.entry(d + bot.radius as i32 + 1).or_insert(0) -= 1;
    }
    let run = dist
        .iter()
        .scan(0i32, |s, (d, &x)| {
            *s += x;
            Some((d, *s))
        })
        .collect::<Vec<_>>();
    let max = run.iter().map(|&(_, n)| n).max().unwrap();
    let intervals = run
        .iter()
        .zip(run.iter().skip(1))
        .filter_map(
            |(&(a, n), &(b, _))| {
                if n == max {
                    Some((*a, *b - 1))
                } else {
                    None
                }
            },
        )
        .collect::<Vec<_>>();
    let result = if intervals.iter().any(|&(a, b)| a <= 0 && b >= 0) {
        0
    } else {
        intervals
            .iter()
            .map(|&(a, b)| if b < 0 { -b } else { a })
            .min()
            .unwrap()
    };

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(36));
    }
}
