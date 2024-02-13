advent_of_code::solution!(8);

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn from_numbers(numbers: &mut Vec<u32>) -> Node {
        let children = numbers.pop().unwrap();
        let metadata = numbers.pop().unwrap();
        let mut children_nodes = Vec::new();
        for _ in 0..children {
            children_nodes.push(Node::from_numbers(numbers));
        }
        let metadata_nodes = numbers.split_off(numbers.len() - metadata as usize);
        Node {
            children: children_nodes,
            metadata: metadata_nodes,
        }
    }

    fn sum_metadata(&self) -> u32 {
        let mut sum = self.metadata.iter().sum();
        for child in &self.children {
            sum += child.sum_metadata();
        }
        sum
    }

    fn value(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .map(|&i| {
                    if i == 0 {
                        0
                    } else {
                        self.children.get(i as usize - 1).map_or(0, |c| c.value())
                    }
                })
                .sum()
        }
    }
}

fn parse(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(' ')
        .map(|line| line.parse::<u32>().unwrap())
        .rev()
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut numbers = parse(input);
    let root = Node::from_numbers(&mut numbers);
    Some(root.sum_metadata())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut numbers = parse(input);
    let root = Node::from_numbers(&mut numbers);
    Some(root.value())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(138));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(66));
    }
}
