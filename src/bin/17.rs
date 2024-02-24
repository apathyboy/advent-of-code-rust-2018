use std::collections::HashSet;

use glam::IVec2;

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

fn explore(map: &GroundSlice, start: IVec2) -> HashSet<IVec2> {
    let mut visited = HashSet::new();
    let mut down_flow = Vec::from([start]);
    let mut current = start;

    visited.insert(start);

    dbg!(&current);
    dbg!(&down_flow);
    dbg!(&visited);

    let mut down = current + IVec2::new(0, 1);

    // explore down until clay or bottom is reached
    while !map.clay.contains(&down) && down.y <= map.bounds.lower_right.y {
        down_flow.push(down);
        visited.insert(down);
        current = down;
        down += IVec2::new(0, 1);
    }

    // return if bottom is reached
    if current.y == map.bounds.lower_right.y {
        return visited;
    }

    down_flow.pop();

    let mut is_overflowing = false;

    // while no overflow is found
    while !is_overflowing {
        // explore left until barrier is found or overflow occurs
        let mut left = current + IVec2::new(-1, 0);
        let mut left_down = left + IVec2::new(0, 1);

        while !map.clay.contains(&left) {
            visited.insert(left);

            if !map.clay.contains(&left_down) && !visited.contains(&left_down) {
                is_overflowing = true;
                visited.extend(explore(map, left_down));
                break;
            }

            left += IVec2::new(-1, 0);
            left_down += IVec2::new(-1, 0);
        }

        // explore right until barrier is found or overflow occurs
        let mut right = current + IVec2::new(1, 0);
        let mut right_down = right + IVec2::new(0, 1);

        while !map.clay.contains(&right) {
            visited.insert(right);

            if !map.clay.contains(&right_down) && !visited.contains(&right_down) {
                is_overflowing = true;
                visited.extend(explore(map, right_down));
                break;
            }

            right += IVec2::new(1, 0);
            right_down += IVec2::new(1, 0);
        }

        if !is_overflowing {
            current = down_flow.pop().unwrap();
        }
    }

    /*
    // beginning with the starting point
    let mut to_visit: VecDeque<IVec2> = VecDeque::from([start]);
    //let mut previous: Vec<IVec2> = Vec::new();

    while !to_visit.is_empty() {
        let current = to_visit.pop_front().unwrap();

        visited.insert(current);

        let down = current + IVec2::new(0, 1);

        // go down until clay or bottom is reached
        if !map.clay.contains(&down)
            && !visited.contains(&down)
            && current.y < map.bounds.lower_right.y
        {
            previous.push(current);

            let explored = explore(map, down, previous);
            visited.extend(explored.clone());
            return visited;
        }

        if current.y == map.bounds.lower_right.y {
            return visited;
        }

        // explore left until

        dbg!(&current);
        dbg!(&previous);
        dbg!(&to_visit);
        dbg!(&visited);
    }
    */

    visited
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse(input)?;

    let water = explore(&map, IVec2::new(500, 1));

    map.draw(&water.iter().cloned().collect::<Vec<_>>());

    //Some(water.len())
    None
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
        assert_eq!(result, Some(57));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
