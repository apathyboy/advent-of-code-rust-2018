use advent_of_code::{Computer, Instruction, Program};

advent_of_code::solution!(19);

fn parse_ip_register(line: &str) -> Option<usize> {
    line[4..].parse().ok()
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    let label = &line[0..4];
    let vals = line[5..]
        .split(' ')
        .filter_map(|n| n.parse().ok())
        .collect::<Vec<_>>();

    Some(Instruction::new(label, vals[0], vals[1], vals[2]))
}

fn parse(input: &str) -> Option<(usize, Program)> {
    let mut lines = input.lines();
    let ip_register = parse_ip_register(lines.next()?)?;

    Some((ip_register, lines.filter_map(parse_instruction).collect()))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (ip_register, program) = parse(input)?;
    let mut computer = Computer::new(ip_register, program);

    computer.run();

    Some(computer.registers[0])
}

pub fn part_two(input: &str) -> Option<usize> {
    let (ip_register, program) = parse(input)?;
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
