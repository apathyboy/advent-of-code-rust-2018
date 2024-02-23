use std::collections::{HashSet, VecDeque};

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

/*
fn explore(map: &GroundSlice, start: IVec2) -> HashSet<IVec2> {
    let mut visited = HashSet::new();

    // beginning with the starting point
    let mut to_visit: VecDeque<IVec2> = VecDeque::from([start]);
    let mut previous: Vec<IVec2> = Vec::new();
    let mut backtracking_to_lip: bool = false;

    while !to_visit.is_empty() {
        let current = to_visit.pop_front().unwrap();

        visited.insert(current);

        if map.bounds.lower_right.y == current.y {
            return visited;
        }

        let down = current + IVec2::new(0, 1);
        let left = current + IVec2::new(-1, 0);
        let right = current + IVec2::new(1, 0);

        // go down until clay is reached
        if !map.clay.contains(&down)
            && !visited.contains(&down)
            && current.y < map.bounds.lower_right.y
        {
            if !previous.is_empty() && current.x != previous.last().unwrap().x {
                previous.clear();
                println!("clearing backtrack");
                backtracking_to_lip = false;
            }

            if down.y <= map.bounds.lower_right.y {
                previous.push(current);

                let explored = explore(map, down);

                if !explored.is_empty()
                    && explored.iter().next().unwrap().y == map.bounds.lower_right.y
                {
                    previous.clear();
                }

                println!("Current: {:?}", current);
                dbg!(&explored);

                visited.extend(explored.clone());

                if explored.iter().all(|e| e.y == current.y + 1) {
                    backtracking_to_lip = true;

                    if !previous.is_empty() && previous.last().unwrap().x == current.x {
                        to_visit.push_back(previous.pop().unwrap());
                        continue;
                    }
                }

                if explored.iter().all(|e| e.x == current.x) {
                    return visited;
                }
            } else {
                return visited;
            }
        }

        if !backtracking_to_lip {
            if current.y <= map.bounds.lower_right.y {
                if !map.clay.contains(&left) && !visited.contains(&left) {
                    to_visit.push_back(left);
                }

                if !map.clay.contains(&right) && !visited.contains(&right) {
                    to_visit.push_back(right);
                }
            }
        }

        to_visit.retain(|f| *f != current);

        dbg!(&to_visit);
        dbg!(&previous);
        dbg!(&current);

        // then reach left and right
        // if walls are met on both sides then backtrack
        // reach left and right and backtrack until level of no walls on either (or both) the sides or no more backtracking possible
        // if during the spreading the water goes over the edge, recurse and combine the results
    }

    visited
}
*/

fn explore(map: &GroundSlice, start: IVec2, previous: &mut Vec<IVec2>) -> HashSet<IVec2> {
    let mut visited = HashSet::new();

    // beginning with the starting point
    let mut to_visit: VecDeque<IVec2> = VecDeque::from([start]);
    //let mut previous: Vec<IVec2> = Vec::new();

    while !to_visit.is_empty() {
        let current = to_visit.pop_front().unwrap();

        visited.insert(current);

        let down = current + IVec2::new(0, 1);
        //let left = current + IVec2::new(-1, 0);
        //let right = current + IVec2::new(1, 0);

        // go down until clay is reached
        if !map.clay.contains(&down)
            && !visited.contains(&down)
            && current.y < map.bounds.lower_right.y
        {
            previous.push(current);

            let explored = explore(map, down, previous);
            visited.extend(explored.clone());
            return visited;
        }

        dbg!(&current);
        dbg!(&previous);
        dbg!(&to_visit);
        dbg!(&visited);

        // then reach left and right
        // if walls are met on both sides then backtrack
        // reach left and right and backtrack until level of no walls on either (or both) the sides or no more backtracking possible
        // if during the spreading the water goes over the edge, recurse and combine the results
    }

    visited
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse(input)?;

    let mut previous = Vec::new();
    let water = explore(&map, IVec2::new(500, 1), &mut previous);

    /*
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::from([IVec2::new(500, 1)]);
    let mut previous = Vec::from([IVec2::new(500, 0)]);

    while !to_visit.is_empty() {
        let current = to_visit.pop_front()?;
        visited.insert(current);

        let down = current + IVec2::new(0, 1);
        let left = current + IVec2::new(-1, 0);
        let right = current + IVec2::new(1, 0);

        if map.clay.contains(&down) {
            if !map.clay.contains(&left) && !visited.contains(&left) {
                to_visit.push_back(left);
            }

            if !map.clay.contains(&right) && !visited.contains(&right) {
                to_visit.push_back(right);
            }
        } else if !map.clay.contains(&down) && down.y <= map.bounds.lower_right.y {
            to_visit.push_back(down);
            previous.push(current);

            if current.x < 0 {
                todo!();
            }
        }

        if to_visit.is_empty() && previous.len() > 1 {
            let mut prev = previous.pop()?;
            if prev == current {
                prev = previous.pop()?;
            }
            println!("Backtracking to: {:?}", prev);
            to_visit.push_back(prev);
        }
    }
    */

    map.draw(&water.iter().cloned().collect::<Vec<_>>());

    Some(water.len())
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
