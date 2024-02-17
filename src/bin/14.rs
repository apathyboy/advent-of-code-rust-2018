use std::fmt::Write;

advent_of_code::solution!(14);

struct ReceipeScoreboard {
    elf1: usize,
    elf2: usize,
    scores: Vec<u8>,
}

impl ReceipeScoreboard {
    fn new() -> Self {
        Self {
            elf1: 0,
            elf2: 1,
            scores: vec![3, 7],
        }
    }

    fn round(&mut self) {
        let elf1_score = self.scores[self.elf1];
        let elf2_score = self.scores[self.elf2];

        let new_score = elf1_score + elf2_score;
        let digit1 = new_score / 10;
        let digit2 = new_score.rem_euclid(10);

        if digit1 != 0 {
            self.scores.push(digit1);
        }

        self.scores.push(digit2);

        self.elf1 = (self.elf1 + 1 + elf1_score as usize).rem_euclid(self.scores.len());
        self.elf2 = (self.elf2 + 1 + elf2_score as usize).rem_euclid(self.scores.len());
    }
}

fn parse(input: &str) -> Option<usize> {
    input.trim().parse().ok()
}

pub fn part_one(input: &str) -> Option<String> {
    let iterations = parse(input)?;

    let mut scoreboard = ReceipeScoreboard::new();

    for _ in 0..iterations + 10 {
        scoreboard.round();
    }

    Some(
        scoreboard
            .scores
            .iter()
            .skip(iterations)
            .take(10)
            .fold(String::new(), |mut output, b| {
                let _ = write!(output, "{b:01X}");
                output
            }),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let search = input
        .trim()
        .chars()
        .filter_map(|c| Some(c.to_digit(10)? as u8))
        .collect::<Vec<_>>();

    let mut scoreboard = ReceipeScoreboard::new();

    while scoreboard.scores.len() < search.len() + 2 {
        scoreboard.round();
    }

    loop {
        scoreboard.round();

        let len = scoreboard.scores.len();
        let search_len = search.len();

        // Check the last and second to last parts for a match
        if search == scoreboard.scores[len - search_len..]
            || (len > search_len && search == scoreboard.scores[len - search_len - 1..len - 1])
        {
            // Instead of iterating over all windows, start searching from a possible position
            let start_pos = len.saturating_sub(search_len * 2);
            return scoreboard.scores[start_pos..]
                .windows(search_len)
                .position(|window| window == search)
                .map(|pos| start_pos + pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5158916779".to_owned()));
    }

    #[rstest]
    #[case("51589", 9)]
    #[case("01245", 5)]
    #[case("92510", 18)]
    #[case("59414", 2018)]
    fn test_part_two(#[case] input: &str, #[case] expected: usize) {
        let result = part_two(input);
        assert_eq!(result, Some(expected));
    }
}
