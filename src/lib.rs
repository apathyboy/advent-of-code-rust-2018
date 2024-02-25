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
