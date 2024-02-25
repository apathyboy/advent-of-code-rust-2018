use std::collections::HashMap;

use glam::IVec2;

advent_of_code::solution!(18);

#[derive(Debug, Clone, Copy, PartialEq)]
enum AreaType {
    Open,
    Woods,
    Lumberyard,
}

#[derive(Debug, Clone)]
struct Area {
    area_type: AreaType,
    neighbors: Vec<IVec2>,
}

impl Area {
    fn new(area_type: AreaType, neighbors: Vec<IVec2>) -> Self {
        Self {
            area_type,
            neighbors,
        }
    }
}

struct Map {
    data: HashMap<IVec2, Area>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(data: HashMap<IVec2, Area>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    #[allow(dead_code)]
    fn top_row(&self) -> String {
        let mut row = String::new();

        for x in 0..self.width {
            let area = self.data.get(&IVec2::new(x as i32, 0)).unwrap();

            row.push(match area.area_type {
                AreaType::Open => '.',
                AreaType::Woods => '|',
                AreaType::Lumberyard => '#',
            });
        }

        row
    }

    #[allow(dead_code)]
    fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let area = self.data.get(&IVec2::new(x as i32, y as i32)).unwrap();

                match area.area_type {
                    AreaType::Open => print!("."),
                    AreaType::Woods => print!("|"),
                    AreaType::Lumberyard => print!("#"),
                }
            }

            println!();
        }

        println!();
    }

    fn tick(&self) -> Map {
        let mut next = self.data.clone();

        for (pos, data) in self.data.iter() {
            let neighbors = data
                .neighbors
                .iter()
                .map(|n| self.data.get(n).unwrap().area_type)
                .collect::<Vec<_>>();

            if data.area_type == AreaType::Open
                && neighbors.iter().filter(|&a| *a == AreaType::Woods).count() >= 3
            {
                next.get_mut(pos).unwrap().area_type = AreaType::Woods;
            }

            if data.area_type == AreaType::Woods
                && neighbors
                    .iter()
                    .filter(|&a| *a == AreaType::Lumberyard)
                    .count()
                    >= 3
            {
                next.get_mut(pos).unwrap().area_type = AreaType::Lumberyard;
            }

            if data.area_type == AreaType::Lumberyard
                && (!neighbors.iter().any(|a| *a == AreaType::Lumberyard)
                    || !neighbors.iter().any(|a| *a == AreaType::Woods))
            {
                next.get_mut(pos).unwrap().area_type = AreaType::Open;
            }
        }

        Map::new(next, self.width, self.height)
    }

    fn resource_count(&self, area_type: AreaType) -> usize {
        self.data
            .iter()
            .filter(|(_, area)| area.area_type == area_type)
            .count()
    }
}

fn parse(input: &str) -> Option<Map> {
    let mut map = HashMap::new();
    let max_y = input.lines().count();
    let max_x = input.lines().next().unwrap().len();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let area_type = match c {
                '.' => AreaType::Open,
                '|' => AreaType::Woods,
                '#' => AreaType::Lumberyard,
                _ => panic!("Invalid area type: {c}"),
            };

            let pos = IVec2::new(x as i32, y as i32);

            let mut neighbors = Vec::new();

            for y in pos.y - 1..=pos.y + 1 {
                for x in pos.x - 1..=pos.x + 1 {
                    if x < 0
                        || y < 0
                        || x >= max_x as i32
                        || y >= max_y as i32
                        || (x == pos.x && y == pos.y)
                    {
                        continue;
                    }

                    neighbors.push(IVec2::new(x, y));
                }
            }

            map.insert(pos, Area::new(area_type, neighbors));
        }
    }

    Some(Map::new(map, max_x, max_y))
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = parse(input)?;

    //println!("Initial state:");
    //map.draw();
    //println!();

    for _ in 0..10 {
        map = map.tick();

        //println!("After {} minute:", i + 1);
        //map.draw();
        //println!();
    }

    Some(map.resource_count(AreaType::Woods) * map.resource_count(AreaType::Lumberyard))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map = parse(input)?;

    let steps = 504 + (1000000000_usize - 504).rem_euclid(28);

    for _ in 0..steps {
        map = map.tick();
    }

    Some(map.resource_count(AreaType::Woods) * map.resource_count(AreaType::Lumberyard))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1147));
    }
}
