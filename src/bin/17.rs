use glam::IVec2;
use pathfinding::prelude::dfs_reach;

advent_of_code::solution!(17);

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
    clay: Vec<IVec2>,
    bounds: Bounds,
}

impl GroundSlice {
    fn new(clay: Vec<IVec2>) -> Self {
        let x_min = clay.iter().map(|p| p.x).min().unwrap();
        let x_max = clay.iter().map(|p| p.x).max().unwrap();
        let y_max = clay.iter().map(|p| p.y).max().unwrap();

        Self {
            clay,
            bounds: Bounds::new(IVec2::new(x_min - 1, 0), IVec2::new(x_max + 1, y_max)),
        }
    }

    fn draw(&self, water: &[IVec2]) {
        for y in self.bounds.upper_left.y..=self.bounds.lower_right.y {
            for x in self.bounds.upper_left.x..=self.bounds.lower_right.x {
                let square = IVec2::new(x, y);

                if self.clay.contains(&square) {
                    print!("#");
                } else if square == IVec2::new(500, 0) {
                    print!("+");
                } else if water.contains(&square) {
                    print!("|");
                } else {
                    print!(".");
                }
            }

            println!();
        }

        println!();
    }
}

fn parse_line(line: &str) -> Option<Vec<IVec2>> {
    let (left, right) = line.split_once(", ")?;
    let mut clay = Vec::new();

    if &left[0..1] == "x" {
        let x: i32 = left[2..].parse().ok()?;

        let (y_min, y_max) = right[2..].split_once("..")?;

        for y in y_min.parse::<i32>().ok()?..=y_max.parse::<i32>().ok()? {
            clay.push(IVec2::new(x, y));
        }
    } else {
        let y: i32 = left[2..].parse().ok()?;

        let (x_min, x_max) = right[2..].split_once("..")?;

        for x in x_min.parse::<i32>().ok()?..=x_max.parse::<i32>().ok()? {
            clay.push(IVec2::new(x, y));
        }
    }

    Some(clay)
}

fn parse(input: &str) -> Option<GroundSlice> {
    Some(GroundSlice::new(
        input.lines().filter_map(parse_line).flatten().collect(),
    ))
}

fn successors(map: &GroundSlice, current: &IVec2) -> Vec<IVec2> {
    todo!()
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input)?;

    let water = dfs_reach(IVec2::new(500, 0), |n| successors(&map, n)).collect::<Vec<_>>();

    map.draw(&water);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
