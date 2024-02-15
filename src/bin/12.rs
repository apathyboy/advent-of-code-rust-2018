advent_of_code::solution!(12);

#[derive(Debug)]
struct GrowRule {
    pattern: String,
    result: char,
}

impl GrowRule {
    fn new(pattern: &str, result: char) -> Self {
        Self {
            pattern: pattern.to_owned(),
            result,
        }
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
            starting_pot: 0,
            current_state: initial_state.to_owned(),
            rules,
        }
    }

    fn plant_count(&self) -> usize {
        self.current_state
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c == '#' {
                    i as isize - self.starting_pot
                } else {
                    0
                }
            })
            .sum::<isize>() as usize
    }

    fn progress(&mut self) {
        let mut next_state = String::new();
        let current_len = self.current_state.len();

        for pot in -2..current_len as isize + 2 {
            let nearby = if pot == -2 {
                format!("....{}", &self.current_state.as_str()[0..1])
            } else if pot == -1 {
                format!("...{}", &self.current_state.as_str()[0..2])
            } else if pot == 0 {
                format!("..{}", &self.current_state.as_str()[0..3])
            } else if pot == 1 {
                format!(".{}", &self.current_state.as_str()[0..4])
            } else if pot == current_len as isize - 2 {
                format!(
                    "{}.",
                    &self.current_state.as_str()[current_len - 4..current_len]
                )
            } else if pot == current_len as isize - 1 {
                format!(
                    "{}..",
                    &self.current_state.as_str()[current_len - 3..current_len]
                )
            } else if pot == current_len as isize {
                format!(
                    "{}...",
                    &self.current_state.as_str()[current_len - 2..current_len]
                )
            } else if pot == current_len as isize + 1 {
                format!(
                    "{}....",
                    &self.current_state.as_str()[current_len - 1..current_len]
                )
            } else {
                (self.current_state.as_str()[pot as usize - 2..pot as usize + 3]).to_string()
            };

            //print!("Searching: {}", nearby);

            if let Some(result) = self.rules.iter().find(|r| r.pattern == nearby) {
                next_state.push(result.result);
                //  println!(" result {}", result.result);
            } else {
                next_state.push('.');
                //println!(" result . - no rule found");
            }
        }

        self.starting_pot += 2;
        self.current_state = next_state;
    }
}

fn parse_rule(line: &str) -> Option<GrowRule> {
    let (pattern, result) = line.split_once(" => ")?;

    Some(GrowRule::new(pattern, result.chars().next()?))
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

pub fn part_two(input: &str) -> Option<usize> {
    let mut grow_op = parse(input)?;

    for _ in 1..=110 {
        grow_op.progress();
    }

    Some(grow_op.plant_count() + ((50000000000 - 110) * 46))
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
