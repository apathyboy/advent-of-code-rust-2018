use glam::IVec2;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Clone, Copy)]
struct LightPoint {
    position: IVec2,
    velocity: IVec2,
}

impl LightPoint {
    fn new(position: IVec2, velocity: IVec2) -> Self {
        Self { position, velocity }
    }
}

fn parse_light_point(line: &str) -> Option<LightPoint> {
    let (position, velocity) = line.split_once("> velocity=<")?;

    let (x, y) = position.split_once(',')?;
    let pos_x = x[10..].trim().parse().ok()?;
    let pos_y = y.trim().parse().ok()?;

    let (x, y) = velocity.split_once(',')?;
    let vel_x = x.trim().parse().ok()?;
    let vel_y = y[0..y.len() - 1].trim().parse().ok()?;

    Some(LightPoint::new(
        IVec2::new(pos_x, pos_y),
        IVec2::new(vel_x, vel_y),
    ))
}

fn find_bounds(points: &[LightPoint]) -> (IVec2, IVec2) {
    let min_x = points.iter().map(|p| p.position.x).min().unwrap();
    let min_y = points.iter().map(|p| p.position.y).min().unwrap();

    let max_x = points.iter().map(|p| p.position.x).max().unwrap();
    let max_y = points.iter().map(|p| p.position.y).max().unwrap();

    (IVec2::new(min_x, min_y), IVec2::new(max_x, max_y))
}

fn format_points(points: &[LightPoint]) -> String {
    let mut result = "".to_owned();
    let (upper_left, lower_right) = find_bounds(points);

    for y in upper_left.y..=lower_right.y {
        for x in upper_left.x..=lower_right.x {
            if points.iter().any(|p| p.position == IVec2::new(x, y)) {
                result.push('#');
            } else {
                result.push('.');
            }
        }

        result.push('\n');
    }

    result
}

fn tick(points: &mut [LightPoint]) {
    for point in points.iter_mut() {
        point.position += point.velocity;
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut points = input
        .lines()
        .filter_map(parse_light_point)
        .collect::<Vec<_>>();

    let (mut upper_left, mut lower_right) = find_bounds(&points);
    let mut prev_state = points.clone();
    let mut prev_height = i32::MAX;

    while lower_right.y - upper_left.y < prev_height {
        prev_height = lower_right.y - upper_left.y;
        prev_state = points.clone();

        tick(&mut points);

        (upper_left, lower_right) = find_bounds(&points);
    }

    Some(format_points(&prev_state))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut points = input
        .lines()
        .filter_map(parse_light_point)
        .collect::<Vec<_>>();

    let (mut upper_left, mut lower_right) = find_bounds(&points);
    let mut prev_height = i32::MAX;

    let mut seconds = 0;

    while lower_right.y - upper_left.y < prev_height {
        prev_height = lower_right.y - upper_left.y;

        tick(&mut points);

        (upper_left, lower_right) = find_bounds(&points);

        seconds += 1;
    }

    Some(seconds - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("#...#..###\n#...#...#.\n#...#...#.\n#####...#.\n#...#...#.\n#...#...#.\n#...#...#.\n#...#..###\n")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
