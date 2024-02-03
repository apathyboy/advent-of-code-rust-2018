use glam::IVec2;

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

pub fn part_one(input: &str) -> Option<u32> {
    let coordinates = parse(input);

    let internal = coordinates
        .iter()
        .filter(|&c| {
            //confirm there is at least one coordinate further in each direction
            coordinates.iter().any(|other| c.x < other.x)
                && coordinates.iter().any(|other| c.x > other.x)
                && coordinates.iter().any(|other| c.y < other.y)
                && coordinates.iter().any(|other| c.y > other.y)
        })
        .collect::<Vec<_>>();

    println!("{:?}", internal);

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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
