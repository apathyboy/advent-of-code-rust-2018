use itertools::Itertools;

advent_of_code::solution!(5);

fn process_reaction(chemicals: &str) -> String {
    let mut stack: Vec<char> = Vec::new();

    for c in chemicals.chars() {
        if let Some(&last) = stack.last() {
            if last != c && last.eq_ignore_ascii_case(&c) {
                stack.pop();
                continue;
            }
        }
        stack.push(c);
    }

    stack.into_iter().collect()
}

fn fully_react(chemicals: &str) -> String {
    let mut previous = chemicals.to_string();

    loop {
        let processed = process_reaction(&previous);

        if processed == previous {
            return processed;
        }

        previous = processed;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(fully_react(input.trim()).len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.trim();

    let units = input
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .unique()
        .collect::<Vec<_>>();

    units
        .iter()
        .map(|&unit| {
            let polymer = input
                .chars()
                .filter(|&c| c.to_ascii_lowercase() != unit)
                .collect::<String>();

            fully_react(&polymer).len()
        })
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fully_react() {
        let result = fully_react("dabAcCaCBAcCcaDA");
        assert_eq!(result, String::from("dabCBAcaDA"));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
