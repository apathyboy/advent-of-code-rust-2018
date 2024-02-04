use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(7);

struct Graph {
    // Maps each node to its list of dependencies
    edges: HashMap<char, HashSet<char>>,
    // Keeps track of the number of dependencies for each node
    in_degrees: HashMap<char, usize>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
            in_degrees: HashMap::new(),
        }
    }

    fn add_dependency(&mut self, before: char, after: char) {
        self.edges
            .entry(before)
            .or_insert_with(HashSet::new)
            .insert(after);
        self.edges.entry(after).or_insert_with(HashSet::new); // Ensure the node exists
        *self.in_degrees.entry(after).or_insert(0) += 1;
        self.in_degrees.entry(before).or_insert(0);
    }

    fn lexicographical_topological_sort(&mut self) -> Option<Vec<char>> {
        let mut sorted = Vec::new();
        let mut zero_in_degree = self
            .in_degrees
            .iter()
            .filter(|&(_, &degree)| degree == 0)
            .map(|(&node, _)| node)
            .collect::<Vec<char>>();

        // Ensure the initial nodes are sorted alphabetically
        zero_in_degree.sort_unstable();

        let mut zero_in_degree_queue = VecDeque::from(zero_in_degree);

        while let Some(node) = zero_in_degree_queue.pop_front() {
            sorted.push(node);
            if let Some(dependents) = self.edges.get(&node) {
                for &dependent in dependents {
                    let in_degree = self.in_degrees.get(&dependent).unwrap();
                    if *in_degree == 1 {
                        zero_in_degree_queue.push_back(dependent);
                        // Ensure the queue is sorted after adding a new element
                        zero_in_degree_queue.make_contiguous().sort_unstable();
                    }
                    *self.in_degrees.get_mut(&dependent).unwrap() -= 1;
                }
            }
        }

        if sorted.len() == self.edges.len() {
            Some(sorted)
        } else {
            None // Cycle detected or incomplete graph
        }
    }
}

fn parse_step(line: &str) -> Option<(char, char)> {
    let step = line.chars().nth(36)?;
    let next = line.chars().nth(5)?;

    Some((next, step))
}

pub fn part_one(input: &str) -> Option<String> {
    let mut graph = Graph::new();

    for line in input.lines() {
        let (a, b) = parse_step(line)?;
        graph.add_dependency(a, b);
    }

    Some(
        graph
            .lexicographical_topological_sort()?
            .iter()
            .collect::<String>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("CABDFE")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
