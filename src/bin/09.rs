use std::collections::VecDeque;

advent_of_code::solution!(9);

fn parse(input: &str) -> (usize, usize) {
    let (players, marbles) = input.trim().split_once("; ").unwrap();

    let players = players.split_whitespace().nth(0).unwrap().parse().unwrap();
    let marbles = marbles.split_whitespace().nth(4).unwrap().parse().unwrap();

    (players, marbles)
}

fn play_game(players: usize, marbles: usize) -> Option<usize> {
    let mut circle = VecDeque::with_capacity(marbles);
    circle.push_back(0);

    let mut scores = vec![0; players];

    for marble in 1..=marbles {
        if marble % 23 == 0 {
            circle.rotate_right(7);
            scores[marble % players] += marble + circle.pop_back().unwrap();
            circle.rotate_left(1);
        } else {
            circle.rotate_left(1);
            circle.push_back(marble);
        }
    }

    Some(*scores.iter().max().unwrap())
}

pub fn part_one(input: &str) -> Option<usize> {
    let (players, marbles) = parse(input);
    play_game(players, marbles)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (players, marbles) = parse(input);
    play_game(players, marbles * 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8317));
    }
}
