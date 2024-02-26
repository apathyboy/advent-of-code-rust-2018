use std::collections::HashMap;

pub mod template;

// Use this file to add helper functions and additional modules.

pub type Opcode = fn(registers: &mut [usize], input_a: usize, input_b: usize, output_c: usize);

fn addr(registers: &mut [usize], input_a: usize, input_b: usize, output_c: usize) {
    let a = registers[input_a];
    let b = registers[input_b];
    registers[output_c] = a + b;
}
fn addi(registers: &mut [usize], input_a: usize, input_b: usize, output_c: usize) {
    let a = registers[input_a];
    registers[output_c] = a + input_b;
}

fn mulr(registers: &mut [usize], input_a: usize, input_b: usize, output_c: usize) {
    let a = registers[input_a];
    let b = registers[input_b];
    registers[output_c] = a * b;
}
fn muli(registers: &mut [usize], input_a: usize, input_b: usize, output_c: usize) {
    let a = registers[input_a];
    registers[output_c] = a * input_b;
}

fn banr(registers: &mut [usize], input_a: usize, input_b: usize, output_c: usize) {
    let a = registers[input_a];
    let b = registers[input_b];
    registers[output_c] = a & b;
}
fn bani(registers: &mut [usize], input_a: usize, input_b: usize, output_c: usize) {
    let a = registers[input_a];
    registers[output_c] = a & input_b;
}

fn borr(registers: &mut [usize], input_a: usize, input_b: usize, output_c: usize) {
    let a = registers[input_a];
    let b = registers[input_b];
    registers[output_c] = a | b;
}
fn bori(registers: &mut [usize], input_a: usize, input_b: usize, output_c: usize) {
    let a = registers[input_a];
    registers[output_c] = a | input_b;
}

fn setr(registers: &mut [usize], input_a: usize, _: usize, output_c: usize) {
    registers[output_c] = registers[input_a];
}
fn seti(registers: &mut [usize], input_a: usize, _: usize, output_c: usize) {
    registers[output_c] = input_a;
}

fn gtir(registers: &mut [usize], input_a: usize, input_b: usize, output_c: usize) {
    let b = registers[input_b];
    registers[output_c] = if input_a > b { 1 } else { 0 };
}
fn gtri(registers: &mut [usize], input_a: usize, input_b: usize, output_c: usize) {
    let a = registers[input_a];
    registers[output_c] = if a > input_b { 1 } else { 0 };
}
fn gtrr(registers: &mut [usize], input_a: usize, input_b: usize, output_c: usize) {
    let a = registers[input_a];
    let b = registers[input_b];
    registers[output_c] = if a > b { 1 } else { 0 };
}

fn eqir(registers: &mut [usize], input_a: usize, input_b: usize, output_c: usize) {
    let b = registers[input_b];
    registers[output_c] = if input_a == b { 1 } else { 0 };
}
fn eqri(registers: &mut [usize], input_a: usize, input_b: usize, output_c: usize) {
    let a = registers[input_a];
    registers[output_c] = if a == input_b { 1 } else { 0 };
}
fn eqrr(registers: &mut [usize], input_a: usize, input_b: usize, output_c: usize) {
    let a = registers[input_a];
    let b = registers[input_b];
    registers[output_c] = if a == b { 1 } else { 0 };
}

pub fn load_opcodes() -> HashMap<String, Opcode> {
    let mut opcodes: HashMap<String, Opcode> = HashMap::new();

    opcodes.insert("addr".to_string(), addr);
    opcodes.insert("addi".to_string(), addi);

    opcodes.insert("mulr".to_string(), mulr);
    opcodes.insert("muli".to_string(), muli);

    opcodes.insert("banr".to_string(), banr);
    opcodes.insert("bani".to_string(), bani);

    opcodes.insert("borr".to_string(), borr);
    opcodes.insert("bori".to_string(), bori);

    opcodes.insert("setr".to_string(), setr);
    opcodes.insert("seti".to_string(), seti);

    opcodes.insert("gtir".to_string(), gtir);
    opcodes.insert("gtri".to_string(), gtri);
    opcodes.insert("gtrr".to_string(), gtrr);

    opcodes.insert("eqir".to_string(), eqir);
    opcodes.insert("eqri".to_string(), eqri);
    opcodes.insert("eqrr".to_string(), eqrr);

    opcodes
}

#[derive(Debug, Clone)]
pub struct Instruction {
    label: String,
    input_a: usize,
    input_b: usize,
    output: usize,
}

impl Instruction {
    pub fn new(label: &str, input_a: usize, input_b: usize, output: usize) -> Self {
        Self {
            label: label.to_owned(),
            input_a,
            input_b,
            output,
        }
    }
}

pub type Program = Vec<Instruction>;

pub struct Computer {
    pub opcodes: HashMap<String, Opcode>,
    pub ip_register: usize,
    pub ip: usize,
    pub registers: [usize; 6],
    pub instructions_executed: usize,
    pub program: Program,
}

impl Computer {
    pub fn new(ip_register: usize, program: Program) -> Self {
        Self {
            opcodes: load_opcodes(),
            ip_register,
            ip: 0,
            registers: [0, 0, 0, 0, 0, 0],
            instructions_executed: 0,
            program,
        }
    }

    pub fn reset_program(&mut self) {
        self.ip = 0;
        self.registers = [0, 0, 0, 0, 0, 0];
        self.instructions_executed = 0;
    }

    pub fn tick(&mut self) {
        let instruction = &self.program[self.ip];

        self.registers[self.ip_register] = self.ip;

        let opcode = self.opcodes.get(&instruction.label).unwrap();

        opcode(
            &mut self.registers,
            instruction.input_a,
            instruction.input_b,
            instruction.output,
        );

        self.instructions_executed += 1;

        self.ip = self.registers[self.ip_register];
        self.ip += 1;
    }

    pub fn run(&mut self) {
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

pub fn parse_computer_program(input: &str) -> Option<(usize, Program)> {
    let mut lines = input.lines();
    let ip_register = parse_ip_register(lines.next()?)?;

    Some((ip_register, lines.filter_map(parse_instruction).collect()))
}
