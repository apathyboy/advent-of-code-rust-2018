use std::collections::HashSet;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .filter_map(|line| line.parse::<i32>().ok())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut frequencies = HashSet::new();
    let mut current_frequency = 0;

    input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .cycle()
        .find_map(|change| {
            current_frequency += change;
            if frequencies.insert(current_frequency) {
                None
            } else {
                Some(current_frequency)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
