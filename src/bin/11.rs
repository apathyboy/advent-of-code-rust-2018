use glam::IVec2;

advent_of_code::solution!(11);

fn calculate_power_level(x: i32, y: i32, serial: i32) -> isize {
    let rack_id = x as isize + 10;

    let mut power_level = rack_id * y as isize;
    power_level += serial as isize;
    power_level *= rack_id;
    power_level = (power_level / 100).rem_euclid(10);
    power_level -= 5;

    power_level
}

fn find_max_region(region_size: i32, serial: i32) -> Option<(IVec2, isize)> {
    let mut max_power_level = isize::MIN;
    let mut max_location = IVec2::MIN;

    for y in 1..=300 - region_size {
        for x in 1..=300 - region_size {
            let mut power_level = 0;

            for check_y in 0..region_size {
                for check_x in 0..region_size {
                    power_level += calculate_power_level(x + check_x, y + check_y, serial);
                }
            }

            if power_level > max_power_level {
                max_power_level = power_level;
                max_location = IVec2::new(x, y);
            }
        }
    }

    Some((max_location, max_power_level))
}

pub fn part_one(input: &str) -> Option<String> {
    let serial = input.trim().parse().ok()?;
    let (max_region, _) = find_max_region(3, serial)?;

    Some(format!("{},{}", max_region.x, max_region.y))
}

pub fn part_two(input: &str) -> Option<String> {
    let serial = input.trim().parse().ok()?;
    let mut max_region = IVec2::MIN;
    let mut max_region_size = 0;
    let mut max_power_level = isize::MIN;

    for region_size in 0..=300 {
        let (region, power_level) = find_max_region(region_size, serial)?;

        if power_level < 0 {
            break;
        }

        if power_level > max_power_level {
            max_power_level = power_level;
            max_region = region;
            max_region_size = region_size;
        }
    }

    Some(format!(
        "{},{},{}",
        max_region.x, max_region.y, max_region_size
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case((3, 5, 8), 4)]
    fn test_calculate_power_level(#[case] input: (i32, i32, i32), #[case] expected: isize) {
        let result = calculate_power_level(input.0, input.1, input.2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_max_region() {
        let result = find_max_region(3, 18);
        assert_eq!(result, Some((IVec2::new(33, 45), 29)));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("33,45")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("90,269,16")));
    }
}
