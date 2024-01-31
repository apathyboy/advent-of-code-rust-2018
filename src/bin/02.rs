use itertools::Itertools;

advent_of_code::solution!(2);

fn count_differences(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .map(|(a_c, b_c)| if a_c != b_c { 1 } else { 0 })
        .sum()
}

fn common_characters(a: &str, b: &str) -> Vec<char> {
    a.chars()
        .zip(b.chars())
        .filter_map(|(a_c, b_c)| if a_c == b_c { Some(a_c) } else { None })
        .collect()
}

fn has_exact_count(input: &str, count: usize) -> bool {
    input
        .chars()
        .into_group_map_by(|&x| x)
        .into_values()
        .map(|v| v.len())
        .filter(|l| *l == count)
        .count()
        >= 1
}

pub fn part_one(input: &str) -> Option<u32> {
    let (twos, threes) = input
        .lines()
        .map(|line| {
            (
                if has_exact_count(line, 2) { 1 } else { 0 },
                if has_exact_count(line, 3) { 1 } else { 0 },
            )
        })
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    Some(twos * threes)
}

pub fn part_two(input: &str) -> Option<String> {
    input
        .lines()
        .find_map(|line| {
            input
                .lines()
                .find(|&l2| count_differences(line, l2) == 1)
                .map(|matching| common_characters(line, matching))
        })
        .map(|common| common.iter().collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(("abcde", "axcye"), 2)]
    #[case(("fghij", "fguij"), 1)]
    fn test_count_differences(#[case] input: (&str, &str), #[case] expected: usize) {
        let result = count_differences(input.0, input.1);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("abcdef", false)]
    #[case("bababc", true)]
    #[case("abbcde", true)]
    #[case("abcccd", false)]
    #[case("aabcdd", true)]
    #[case("abcdee", true)]
    #[case("ababab", false)]
    fn test_has_two_count(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(has_exact_count(input, 2), expected)
    }

    #[rstest]
    #[case("abcdef", false)]
    #[case("bababc", true)]
    #[case("abbcde", false)]
    #[case("abcccd", true)]
    #[case("aabcdd", false)]
    #[case("abcdee", false)]
    #[case("ababab", true)]
    fn test_has_three_count(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(has_exact_count(input, 3), expected)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(String::from("fgij")));
    }
}
