use advent_of_code::{parse_computer_program, Computer};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    let (ip_register, program) = parse_computer_program(input)?;
    let mut computer = Computer::new(ip_register, program);

    computer.run();

    Some(computer.registers[0])
}

pub fn part_two(input: &str) -> Option<usize> {
    let (ip_register, program) = parse_computer_program(input)?;
    let mut computer = Computer::new(ip_register, program);

    computer.registers[0] = 1;

    loop {
        computer.tick();

        if computer.ip == 2 {
            return Some(
                computer.registers[2]
                    + (1..=computer.registers[2] / 2)
                        .filter(|x| computer.registers[2] % x == 0)
                        .sum::<usize>(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
