use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(4);

struct SleepPeriod {
    start_minute: usize,
    duration: usize,
}

fn parse(input: &str) -> Option<HashMap<usize, Vec<SleepPeriod>>> {
    let mut current_guard = 0;
    let mut sleep_start_minute: Option<usize> = None;
    let mut sleep_duration: Option<usize>;

    let mut sleep_map = HashMap::new();

    for line in input.lines().sorted() {
        if line.contains("Guard") {
            current_guard = line[26..line.len() - 13].parse().ok()?;

            sleep_map.entry(current_guard).or_insert_with(Vec::new);
        }

        if line.contains("falls") {
            sleep_start_minute = line[15..17].parse().ok();
        }

        if line.contains("wakes") {
            sleep_duration = Some(line[15..17].parse::<usize>().ok()? - sleep_start_minute?);

            sleep_map.get_mut(&current_guard)?.push(SleepPeriod {
                start_minute: sleep_start_minute?,
                duration: sleep_duration?,
            });
        }
    }

    Some(sleep_map)
}

fn find_most_frequent_minute(sleep_periods: &[SleepPeriod]) -> (usize, usize) {
    let mut frequency = [0; 60];

    for sleep_period in sleep_periods {
        for minute in frequency
            .iter_mut()
            .skip(sleep_period.start_minute)
            .take(sleep_period.duration)
        {
            *minute += 1;
        }
    }

    frequency
        .iter()
        .enumerate()
        .max_by_key(|&(_, count)| count)
        .map_or((0, 0), |(minute, &count)| (minute, count))
}

pub fn part_one(input: &str) -> Option<usize> {
    let guard_sleep_periods = parse(input)?;

    guard_sleep_periods
        .iter()
        .map(|(&guard, periods)| {
            let total_sleep: usize = periods.iter().map(|s| s.duration).sum();
            let (minute, _) = find_most_frequent_minute(periods);
            (guard, total_sleep, minute)
        })
        .max_by_key(|&(_, total_sleep, _)| total_sleep)
        .map(|(guard, _, most_overlapped_minute)| guard * most_overlapped_minute)
}

pub fn part_two(input: &str) -> Option<usize> {
    let guard_sleep_periods = parse(input)?;

    guard_sleep_periods
        .into_iter()
        .map(|(guard, sleeps)| {
            let (minute, occurrences) = find_most_frequent_minute(&sleeps);
            (guard, minute, occurrences)
        })
        .max_by_key(|&(_, _, occurrences)| occurrences)
        .map(|(guard, minute, _)| guard * minute)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(240));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4455));
    }
}
