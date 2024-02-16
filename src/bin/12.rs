advent_of_code::solution!(12);

#[derive(Debug)]
struct GrowRule {
    pattern: Vec<char>,
    result: char,
}

impl GrowRule {
    fn new(pattern: Vec<char>, result: char) -> Self {
        Self { pattern, result }
    }
}

#[derive(Debug)]
struct GrowOp {
    starting_pot: isize,
    current_state: String,
    rules: Vec<GrowRule>,
}

impl GrowOp {
    fn new(initial_state: &str, rules: Vec<GrowRule>) -> Self {
        Self {
            starting_pot: 2,
            current_state: format!("..{}..", initial_state),
            rules,
        }
    }

    fn plant_count(&self) -> usize {
        self.current_state
            .chars()
            .enumerate()
            .filter_map(|(i, c)| {
                if c == '#' {
                    Some(i as isize - self.starting_pot)
                } else {
                    None
                }
            })
            .sum::<isize>() as usize
    }

    fn progress(&mut self) {
        let mut next_state = String::new();

        for nearby in self.current_state.chars().collect::<Vec<_>>().windows(5) {
            if let Some(result) = self.rules.iter().find(|&r| r.pattern == nearby) {
                next_state.push(result.result);
            } else {
                next_state.push('.');
            }
        }

        self.starting_pot += 1;
        self.current_state = format!("...{}...", next_state);
    }
}

fn parse_rule(line: &str) -> Option<GrowRule> {
    let (pattern, result) = line.split_once(" => ")?;

    Some(GrowRule::new(
        pattern.chars().collect(),
        result.chars().next()?,
    ))
}

fn parse(input: &str) -> Option<GrowOp> {
    let initial_state = &input.lines().next()?[15..];
    let rules = input.lines().skip(2).filter_map(parse_rule).collect();

    Some(GrowOp::new(initial_state, rules))
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grow_op = parse(input)?;

    for _ in 1..=20 {
        grow_op.progress();
    }

    Some(grow_op.plant_count())
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut grow_op = parse(input)?;

    let mut gen = 1;
    let mut prev_plants = grow_op.plant_count() as isize;
    let mut prev_delta = 0;

    loop {
        grow_op.progress();
        let delta = grow_op.plant_count() as isize - prev_plants;

        if delta == prev_delta {
            break;
        }

        prev_plants = grow_op.plant_count() as isize;
        prev_delta = delta;
        gen += 1;
    }

    Some(grow_op.plant_count() as isize + ((50000000000 - gen) * prev_delta))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(325));
    }
}
