use advent_of_code::{parse_computer_program, Computer};

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<usize> {
    let (ip_register, program) = parse_computer_program(input)?;
    let mut computer = Computer::new(ip_register, program);

    let mut seen = Vec::new();
    loop {
        computer.tick();
        if computer.ip == 28 {
            let value = computer.registers[5];
            if seen.contains(&value) {
                return Some(seen.first().unwrap().clone());
            }
            seen.push(value);
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let (ip_register, program) = parse_computer_program(input)?;
    let mut computer = Computer::new(ip_register, program);

    let mut seen = Vec::new();
    loop {
        computer.tick();
        if computer.ip == 28 {
            let value = computer.registers[5];
            if seen.contains(&value) {
                return Some(seen.last().unwrap().clone());
            }
            seen.push(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
