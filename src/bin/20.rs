use pathfinding::prelude::dijkstra_all;
use std::collections::{BTreeMap, BTreeSet};

advent_of_code::solution!(20);

type Point = (i32, i32);

fn explore(
    map: &mut BTreeMap<Point, BTreeSet<Point>>,
    start: Point,
    input: &[u8],
    index: &mut usize,
) -> Vec<Point> {
    let mut exits = vec![start];
    loop {
        match input[*index] {
            b'|' | b')' | b'$' => return exits,
            b'(' => {
                let mut new_exits = BTreeSet::new();
                while input[*index] != b')' {
                    let old_index = *index;
                    new_exits.extend(exits.iter().flat_map(|pos| {
                        *index = old_index + 1;
                        explore(map, *pos, input, index)
                    }));
                }
                exits = new_exits.into_iter().collect();
            }
            dir => {
                let dir = usize::from((dir ^ (dir >> 2)) & 3);
                let (dx, dy) = ([1, 0, -1, 0][dir], [0, -1, 0, 1][dir]);
                for pos in &mut exits {
                    let newpos = (pos.0 + dx, pos.1 + dy);
                    map.entry(*pos).or_default().insert(newpos);
                    *pos = newpos;
                }
            }
        }
        *index += 1;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = BTreeMap::new();

    explore(&mut map, (0, 0), input.as_bytes(), &mut 1);

    let map = dijkstra_all(&(0, 0), |pos| {
        map.get(pos)
            .into_iter()
            .flat_map(|neighbours| neighbours.iter().map(|n| (*n, 1)))
    });

    Some(map.values().map(|(_, c)| *c).max().unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map = BTreeMap::new();

    explore(&mut map, (0, 0), input.as_bytes(), &mut 1);

    let map = dijkstra_all(&(0, 0), |pos| {
        map.get(pos)
            .into_iter()
            .flat_map(|neighbours| neighbours.iter().map(|n| (*n, 1)))
    });

    Some(map.values().filter(|&(_, c)| *c >= 1000).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
