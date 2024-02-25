use advent_of_code::{load_opcodes, Opcode};
use std::collections::HashMap;

advent_of_code::solution!(19);

#[derive(Debug, Clone)]
struct Instruction {
    label: String,
    input_a: usize,
    input_b: usize,
    output: usize,
}

impl Instruction {
    fn new(label: &str, input_a: usize, input_b: usize, output: usize) -> Self {
        Self {
            label: label.to_owned(),
            input_a,
            input_b,
            output,
        }
    }
}

type Program = Vec<Instruction>;

struct Computer {
    opcodes: HashMap<String, Opcode>,
    ip_register: usize,
    ip: usize,
    registers: [usize; 6],
    program: Program,
}

impl Computer {
    fn new(ip_register: usize, program: Program) -> Self {
        Self {
            opcodes: load_opcodes(),
            ip_register,
            ip: 0,
            registers: [0, 0, 0, 0, 0, 0],
            program,
        }
    }

    fn tick(&mut self) {
        let instruction = &self.program[self.ip];

        self.registers[self.ip_register] = self.ip;

        let opcode = self.opcodes.get(&instruction.label).unwrap();

        opcode(
            &mut self.registers,
            instruction.input_a,
            instruction.input_b,
            instruction.output,
        );

        self.ip = self.registers[self.ip_register];
        self.ip += 1;
    }

    fn run(&mut self) {
        while self.ip < self.program.len() {
            self.tick();
        }
    }
}

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
