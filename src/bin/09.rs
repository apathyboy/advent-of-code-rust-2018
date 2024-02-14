advent_of_code::solution!(9);

fn parse(input: &str) -> (usize, usize) {
    let (players, marbles) = input.trim().split_once("; ").unwrap();

    let players = players.split_whitespace().nth(0).unwrap().parse().unwrap();
    let marbles = marbles.split_whitespace().nth(4).unwrap().parse().unwrap();

    (players, marbles)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (players, marbles) = parse(input);

    println!(
        "{} players; last marble is worth {} points",
        players, marbles
    );

    let mut circle = Vec::with_capacity(marbles);
    circle.push(0);

    let mut current_marble = 0;

    let mut scores = vec![0; players];

    for marble in 1..=marbles {
        if marble % 23 == 0 {
            let player = marble % players;
            scores[player] += marble;

            let remove_index = (current_marble + circle.len() - 7) % circle.len();
            scores[player] += circle.remove(remove_index);

            current_marble = remove_index % circle.len();
        } else {
            let insert_index = (current_marble + 2) % circle.len();
            circle.insert(insert_index, marble);
            current_marble = insert_index;
        }
    }

    Some(*scores.iter().max().unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (players, marbles) = parse(input);

    let mut circle = Vec::with_capacity(marbles);
    circle.push(0);

    let mut current_marble = 0;

    let mut scores = vec![0; players];

    for marble in 1..=marbles * 100 {
        if marble % 23 == 0 {
            let player = marble % players;
            scores[player] += marble;

            let remove_index = (current_marble + circle.len() - 7) % circle.len();
            scores[player] += circle.remove(remove_index);

            current_marble = remove_index % circle.len();
        } else {
            let insert_index = (current_marble + 2) % circle.len();
            circle.insert(insert_index, marble);
            current_marble = insert_index;
        }

        println!("{}: {}", marble, current_marble);
    }

    Some(*scores.iter().max().unwrap())
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
