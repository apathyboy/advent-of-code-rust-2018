use std::collections::{HashMap, HashSet};

advent_of_code::solution!(16);

#[derive(Debug)]
struct Snapshot {
    before: Vec<usize>,
    after: Vec<usize>,
    input: Vec<usize>,
}

impl Snapshot {
    fn new() -> Self {
        Self {
            before: Vec::new(),
            after: Vec::new(),
            input: Vec::new(),
        }
    }
}

type Opcode = fn(registers: &mut [usize], input_a: usize, input_b: usize, output_c: usize);

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

fn load_opcodes() -> HashMap<String, Opcode> {
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

fn identify_opcodes(
    opcodes: &HashMap<String, Opcode>,
    snapshots: &[Snapshot],
) -> Option<HashMap<usize, Opcode>> {
    let mut mapping: HashMap<String, HashSet<usize>> = HashMap::new();

    for name in opcodes.keys() {
        mapping.insert(name.to_string(), HashSet::new());
    }

    for snapshot in snapshots {
        for (name, processor) in opcodes.iter() {
            let mut registers = snapshot.before.clone();

            processor(
                &mut registers,
                snapshot.input[1],
                snapshot.input[2],
                snapshot.input[3],
            );

            if registers == snapshot.after {
                mapping.get_mut(name).unwrap().insert(snapshot.input[0]);
            }
        }
    }

    let mut mapping_vec = mapping.iter_mut().collect::<Vec<_>>();
    mapping_vec.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    let mut updated_mapping: HashMap<usize, Opcode> = HashMap::new();

    for i in 0..mapping_vec.len() {
        let idx = *mapping_vec[i].1.iter().next()?;

        updated_mapping.insert(idx, *opcodes.get(mapping_vec[i].0)?);

        for (_, j) in mapping_vec.iter_mut() {
            j.remove(&idx);
        }

        mapping_vec.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
    }

    Some(updated_mapping)
}

fn parse(input: &str) -> Option<(Vec<Snapshot>, Vec<Vec<usize>>)> {
    let mut snapshots = Vec::new();
    let mut program = Vec::new();

    let mut lines = input.lines().peekable();
    while let Some(line) = lines.next() {
        let mut snapshot = Snapshot::new();

        snapshot.before = line[9..line.len() - 1]
            .split(", ")
            .filter_map(|n| n.parse().ok())
            .collect();

        if let Some(next_line) = lines.next() {
            snapshot.input = next_line
                .split(' ')
                .filter_map(|n| n.parse().ok())
                .collect();
        }

        if let Some(next_line) = lines.next() {
            snapshot.after = next_line[9..next_line.len() - 1]
                .split(", ")
                .filter_map(|n| n.parse().ok())
                .collect();
        }

        if lines.next().is_some() {
            snapshots.push(snapshot);
        }

        // Check if the next two characters are newlines, indicating the end of processing
        if lines.peek().is_none()
            || lines
                .peek()
                .map_or(false, |&next_line| next_line.is_empty())
        {
            break;
        }
    }

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let program_line = line.split(' ').filter_map(|n| n.parse().ok()).collect();
        program.push(program_line);
    }

    Some((snapshots, program))
}

pub fn part_one(input: &str) -> Option<usize> {
    let opcodes = load_opcodes();

    let (snapshots, _) = parse(input)?;

    let mut found = 0;

    for snapshot in snapshots {
        let mut matches = 0;

        for processor in opcodes.values() {
            let mut registers = snapshot.before.clone();

            processor(
                &mut registers,
                snapshot.input[1],
                snapshot.input[2],
                snapshot.input[3],
            );

            if registers == snapshot.after {
                matches += 1;
            }
        }

        if matches >= 3 {
            found += 1;
        }
    }

    Some(found)
}

pub fn part_two(input: &str) -> Option<usize> {
    let opcodes = load_opcodes();

    let (snapshots, program) = parse(input)?;

    let opcodes = identify_opcodes(&opcodes, &snapshots)?;

    let mut registers = Vec::from([0, 0, 0, 0]);

    for line in program {
        if let Some(&opcode) = opcodes.get(&line[0]) {
            opcode(&mut registers, line[1], line[2], line[3]);
        }
    }

    Some(registers[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
