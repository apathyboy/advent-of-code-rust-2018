use std::collections::HashSet;

use glam::IVec2;

advent_of_code::solution!(17);

fn in_range(x: i32, a0: i32, a1: i32) -> bool {
    x >= a0 && x <= a1
}

struct Bounds {
    upper_left: IVec2,
    lower_right: IVec2,
}

impl Bounds {
    fn new(upper_left: IVec2, lower_right: IVec2) -> Self {
        Self {
            upper_left,
            lower_right,
        }
    }
}

struct GroundSlice {
    clay: HashSet<IVec2>,
    settled: HashSet<IVec2>,
    flowing: HashSet<IVec2>,
    bounds: Bounds,
    max_y: i32,
    min_y: i32,
}

impl GroundSlice {
    fn new(clay: HashSet<IVec2>) -> Self {
        let x_min = clay.iter().map(|p| p.x).min().unwrap();
        let x_max = clay.iter().map(|p| p.x).max().unwrap();
        let y_max = clay.iter().map(|p| p.y).max().unwrap();

        let max_y = clay.iter().map(|p| p.y).max().unwrap();
        let min_y = clay.iter().map(|p| p.y).min().unwrap();

        Self {
            clay,
            settled: HashSet::new(),
            flowing: HashSet::new(),
            bounds: Bounds::new(IVec2::new(x_min - 1, 0), IVec2::new(x_max + 1, y_max)),
            max_y,
            min_y,
        }
    }

    #[allow(dead_code)]
    fn draw(&self) {
        for y in self.bounds.upper_left.y..=self.bounds.lower_right.y {
            for x in self.bounds.upper_left.x..=self.bounds.lower_right.x {
                let square = IVec2::new(x, y);

                if self.clay.contains(&square) {
                    print!("#");
                } else if square == IVec2::new(500, 0) {
                    print!("+");
                } else if self.settled.contains(&square) {
                    print!("~");
                } else if self.flowing.contains(&square) {
                    print!("|");
                } else {
                    print!(".");
                }
            }

            println!();
        }

        println!();
    }

    fn fill(&mut self, start: IVec2, dir: IVec2) -> bool {
        self.flowing.insert(start);

        let below = IVec2::new(start.x, start.y + 1);

        if !self.clay.contains(&below)
            && !self.flowing.contains(&below)
            && in_range(below.y, 1, self.bounds.lower_right.y)
        {
            self.fill(below, IVec2::new(0, 1));
        }

        if !self.clay.contains(&below) && !self.settled.contains(&below) {
            return false;
        }

        let mut left = IVec2::new(start.x - 1, start.y);
        let mut right = IVec2::new(start.x + 1, start.y);

        let left_filled = self.clay.contains(&left)
            || !self.flowing.contains(&left) && self.fill(left, IVec2::new(-1, 0));
        let right_filled = self.clay.contains(&right)
            || !self.flowing.contains(&right) && self.fill(right, IVec2::new(1, 0));

        if dir == IVec2::new(0, 1) && left_filled && right_filled {
            self.settled.insert(start);

            while self.flowing.contains(&left) {
                self.settled.insert(left);
                left.x -= 1;
            }

            while self.flowing.contains(&right) {
                self.settled.insert(right);
                right.x += 1;
            }
        }

        dir == IVec2::new(-1, 0) && (left_filled || self.clay.contains(&left))
            || dir == IVec2::new(1, 0) && (right_filled || self.clay.contains(&right))
    }
}

fn parse_line(line: &str) -> Option<HashSet<IVec2>> {
    let (left, right) = line.split_once(", ")?;
    let mut clay = HashSet::new();

    if &left[0..1] == "x" {
        let x: i32 = left[2..].parse().ok()?;

        let (y_min, y_max) = right[2..].split_once("..")?;

        for y in y_min.parse::<i32>().ok()?..=y_max.parse::<i32>().ok()? {
            clay.insert(IVec2::new(x, y));
        }
    } else {
        let y: i32 = left[2..].parse().ok()?;

        let (x_min, x_max) = right[2..].split_once("..")?;

        for x in x_min.parse::<i32>().ok()?..=x_max.parse::<i32>().ok()? {
            clay.insert(IVec2::new(x, y));
        }
    }

    Some(clay)
}

fn parse(input: &str) -> Option<GroundSlice> {
    Some(GroundSlice::new(
        input.lines().filter_map(parse_line).flatten().collect(),
    ))
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut slice = parse(input)?;

    slice.fill(IVec2::new(500, 0), IVec2::new(0, 1));

    Some(
        slice
            .flowing
            .union(&slice.settled)
            .filter(|x| in_range(x.y, slice.min_y, slice.max_y))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut slice = parse(input)?;

    slice.fill(IVec2::new(500, 0), IVec2::new(0, 1));

    Some(
        slice
            .settled
            .iter()
            .filter(|x| in_range(x.y, slice.min_y, slice.max_y))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(57));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(29));
    }
}
