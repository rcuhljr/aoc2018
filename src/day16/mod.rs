use super::utility;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve_a() -> String {
    let samples = parse_input("input16.txt".to_string()).0;
    samples
        .iter()
        .map(|sample| find_potential_opcodes(*sample).len())
        .filter(|x| *x >= 3)
        .count()
        .to_string()
}

pub fn solve_b() -> String {
    run_program("input16.txt".to_string()).to_string()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Sample {
    before: [usize; 4],
    op_code: [usize; 4],
    after: [usize; 4],
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum OpCode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

fn parse_input(filename: String) -> (Vec<Sample>, Vec<[usize; 4]>) {
    lazy_static! {
        static ref BEFORE_REGEX: Regex =
            Regex::new(r"Before:\s+\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
        static ref OPCODE_REGEX: Regex = Regex::new(r"^(\d+) (\d+) (\d+) (\d+)").unwrap();
        static ref AFTER_REGEX: Regex =
            Regex::new(r"After:\s+\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    }
    let raw_data = utility::load_strings(filename);
    let mut samples: Vec<Sample> = Vec::new();
    let mut data: Vec<[usize; 4]> = Vec::new();

    let mut saw_blank = false;
    let mut data_section = false;

    let mut before: [usize; 4] = [0, 0, 0, 0];
    let mut op_code: [usize; 4] = [0, 0, 0, 0];
    let mut after: [usize; 4];

    for line in raw_data.iter() {
        if line == "" {
            if saw_blank {
                data_section = true;
            } else {
                saw_blank = true;
            }
            continue;
        } else {
            saw_blank = false;
        }

        if !data_section {
            if let Some(caps) = BEFORE_REGEX.captures(line) {
                before = [
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                    caps[3].parse().unwrap(),
                    caps[4].parse().unwrap(),
                ];
                continue;
            }
            if let Some(caps) = OPCODE_REGEX.captures(line) {
                op_code = [
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                    caps[3].parse().unwrap(),
                    caps[4].parse().unwrap(),
                ];
                continue;
            }
            if let Some(caps) = AFTER_REGEX.captures(line) {
                after = [
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                    caps[3].parse().unwrap(),
                    caps[4].parse().unwrap(),
                ];
                samples.push(Sample {
                    before,
                    op_code,
                    after,
                });
            }
        } else {
            if let Some(caps) = OPCODE_REGEX.captures(line) {
                op_code = [
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                    caps[3].parse().unwrap(),
                    caps[4].parse().unwrap(),
                ];
                data.push(op_code);
            }
        }
    }

    (samples, data)
}

fn determine_opcodes(samples: &mut Vec<Sample>) -> HashMap<usize, OpCode> {
    let mut potentials: HashMap<usize, HashSet<OpCode>> = HashMap::new();
    for sample in samples.iter() {
        let key = sample.op_code[0];
        let matches = find_potential_opcodes(*sample);
        let mut hash_match: HashSet<OpCode> = HashSet::new();
        for inst in matches {
            hash_match.insert(inst);
        }

        let val = potentials
            .get(&key)
            .unwrap_or_else(|| &hash_match)
            .intersection(&hash_match)
            .cloned()
            .collect();
        potentials.insert(key, val);
    }

    let mut result: HashMap<usize, OpCode> = HashMap::new();

    while result.len() < 16 {
        let mut found_key = 0;
        let mut found_val = HashSet::new();

        for (key, val) in potentials.iter() {
            if val.len() == 1 && !result.contains_key(key) {
                found_key = key.clone();
                found_val = val.clone();
                break;
            }
        }

        let inst = found_val.iter().next().unwrap().clone();
        result.insert(found_key, inst);
        potentials.remove(&found_key);
        potentials.values_mut().for_each(|set| {
            set.remove(&inst);
        });
    }

    result
}

fn find_potential_opcodes(sample: Sample) -> Vec<OpCode> {
    let possibles = vec![
        OpCode::Addr,
        OpCode::Addi,
        OpCode::Mulr,
        OpCode::Muli,
        OpCode::Banr,
        OpCode::Bani,
        OpCode::Borr,
        OpCode::Bori,
        OpCode::Setr,
        OpCode::Seti,
        OpCode::Gtir,
        OpCode::Gtri,
        OpCode::Gtrr,
        OpCode::Eqir,
        OpCode::Eqri,
        OpCode::Eqrr,
    ];
    let mut matches = vec![];

    for inst in possibles.iter() {
        let mut regs = sample.before.clone();
        process_instruction(&mut regs, &sample.op_code, inst);
        if regs == sample.after {
            matches.push(*inst);
        }
    }
    matches
}

fn process_instruction(regs: &mut [usize; 4], inst: &[usize; 4], itype: &OpCode) {
    match itype {
        OpCode::Addr => {
            regs[inst[3]] = regs[inst[1]] + regs[inst[2]];
        }
        OpCode::Addi => {
            regs[inst[3]] = regs[inst[1]] + inst[2];
        }
        OpCode::Mulr => {
            regs[inst[3]] = regs[inst[1]] * regs[inst[2]];
        }
        OpCode::Muli => {
            regs[inst[3]] = regs[inst[1]] * inst[2];
        }
        OpCode::Banr => {
            regs[inst[3]] = regs[inst[1]] & regs[inst[2]];
        }
        OpCode::Bani => {
            regs[inst[3]] = regs[inst[1]] & inst[2];
        }
        OpCode::Borr => {
            regs[inst[3]] = regs[inst[1]] | regs[inst[2]];
        }
        OpCode::Bori => {
            regs[inst[3]] = regs[inst[1]] | inst[2];
        }
        OpCode::Setr => {
            regs[inst[3]] = regs[inst[1]];
        }
        OpCode::Seti => {
            regs[inst[3]] = inst[1];
        }
        OpCode::Gtir => {
            regs[inst[3]] = if inst[1] > regs[inst[2]] { 1 } else { 0 };
        }
        OpCode::Gtri => {
            regs[inst[3]] = if regs[inst[1]] > inst[2] { 1 } else { 0 };
        }
        OpCode::Gtrr => {
            regs[inst[3]] = if regs[inst[1]] > regs[inst[2]] { 1 } else { 0 };
        }
        OpCode::Eqir => {
            regs[inst[3]] = if inst[1] == regs[inst[2]] { 1 } else { 0 };
        }
        OpCode::Eqri => {
            regs[inst[3]] = if regs[inst[1]] == inst[2] { 1 } else { 0 };
        }
        OpCode::Eqrr => {
            regs[inst[3]] = if regs[inst[1]] == regs[inst[2]] { 1 } else { 0 };
        }
    }
}

fn run_program(filename: String) -> usize {
    let (mut samples, data) = parse_input(filename);

    let spec = determine_opcodes(&mut samples);

    let mut regs = [0, 0, 0, 0];

    for inst in data {
        process_instruction(&mut regs, &inst, spec.get(&inst[0]).unwrap());
    }

    regs[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_process_instructions() {
        let mut actual = [3, 2, 1, 1];
        process_instruction(&mut actual, &[9, 2, 1, 2], &OpCode::Addr);
        assert_eq!(actual, [3, 2, 3, 1]);

        actual = [3, 2, 1, 1];
        process_instruction(&mut actual, &[9, 2, 1, 2], &OpCode::Addi);
        assert_eq!(actual, [3, 2, 2, 1]);

        actual = [3, 2, 1, 1];
        process_instruction(&mut actual, &[9, 2, 1, 2], &OpCode::Mulr);
        assert_eq!(actual, [3, 2, 2, 1]);

        actual = [3, 2, 1, 1];
        process_instruction(&mut actual, &[9, 2, 1, 2], &OpCode::Muli);
        assert_eq!(actual, [3, 2, 1, 1]);

        actual = [3, 4, 3, 2];
        process_instruction(&mut actual, &[9, 2, 1, 3], &OpCode::Banr);
        assert_eq!(actual, [3, 4, 3, 0]);

        actual = [3, 4, 3, 2];
        process_instruction(&mut actual, &[9, 2, 1, 3], &OpCode::Bani);
        assert_eq!(actual, [3, 4, 3, 1]);

        actual = [3, 4, 3, 2];
        process_instruction(&mut actual, &[9, 2, 1, 3], &OpCode::Borr);
        assert_eq!(actual, [3, 4, 3, 7]);

        actual = [3, 4, 3, 2];
        process_instruction(&mut actual, &[9, 2, 1, 3], &OpCode::Bori);
        assert_eq!(actual, [3, 4, 3, 3]);

        actual = [3, 4, 3, 2];
        process_instruction(&mut actual, &[9, 2, 1, 3], &OpCode::Setr);
        assert_eq!(actual, [3, 4, 3, 3]);

        actual = [3, 4, 3, 2];
        process_instruction(&mut actual, &[9, 2, 1, 3], &OpCode::Seti);
        assert_eq!(actual, [3, 4, 3, 2]);

        actual = [3, 4, 3, 2];
        process_instruction(&mut actual, &[9, 2, 1, 3], &OpCode::Gtir);
        assert_eq!(actual, [3, 4, 3, 0]);

        actual = [3, 4, 3, 2];
        process_instruction(&mut actual, &[9, 5, 1, 3], &OpCode::Gtir);
        assert_eq!(actual, [3, 4, 3, 1]);

        actual = [3, 4, 3, 2];
        process_instruction(&mut actual, &[9, 2, 1, 3], &OpCode::Gtri);
        assert_eq!(actual, [3, 4, 3, 1]);

        actual = [3, 4, 3, 2];
        process_instruction(&mut actual, &[9, 2, 1, 3], &OpCode::Gtrr);
        assert_eq!(actual, [3, 4, 3, 0]);

        actual = [3, 4, 3, 2];
        process_instruction(&mut actual, &[9, 2, 1, 3], &OpCode::Eqir);
        assert_eq!(actual, [3, 4, 3, 0]);

        actual = [3, 4, 3, 2];
        process_instruction(&mut actual, &[9, 4, 1, 3], &OpCode::Eqir);
        assert_eq!(actual, [3, 4, 3, 1]);

        actual = [3, 4, 3, 2];
        process_instruction(&mut actual, &[9, 1, 4, 3], &OpCode::Eqri);
        assert_eq!(actual, [3, 4, 3, 1]);

        actual = [5, 4, 4, 2];
        process_instruction(&mut actual, &[9, 2, 1, 3], &OpCode::Eqrr);
        assert_eq!(actual, [5, 4, 4, 1]);
    }

    #[test]
    fn should_find_3_opcodes_for_sample() {
        let actual = find_potential_opcodes(Sample {
            before: [3, 2, 1, 1],
            op_code: [9, 2, 1, 2],
            after: [3, 2, 2, 1],
        });

        assert_eq!(actual, [OpCode::Addi, OpCode::Mulr, OpCode::Seti]);
    }

    #[test]
    fn should_work_with_bigger_sample() {
        let samples = parse_input("./src/day16/test2.txt".to_string()).0;
        let result: Vec<usize> = samples
            .iter()
            .map(|sample| find_potential_opcodes(*sample).len())
            .collect();
        assert_eq!(result, [9, 2, 13, 2, 10, 2, 1, 7]);

        assert_eq!(result.iter().cloned().filter(|x| *x >= 3).count(), 4);
    }

    #[test]
    fn should_get_right_number_of_samples() {
        let mut samples = parse_input("input16.txt".to_string()).0;

        assert_eq!(samples.len(), 782);
        assert_eq!(determine_opcodes(&mut samples).len(), 16);
    }

    #[test]
    fn should_read_initial_input16() {
        let actual = parse_input("./src/day16/test.txt".to_string());
        assert_eq!(
            actual.0,
            [Sample {
                before: [3, 2, 1, 1],
                op_code: [9, 2, 1, 2],
                after: [3, 2, 2, 1]
            }]
        );

        assert_eq!(actual.1, [[0, 1, 2, 3]]);
    }

}
