use super::utility;
use regex::Regex;
use std::collections::HashSet;

pub fn solve_a() -> String {
    run_for_part_a(&"input21.txt".to_string(), [1, 0, 0, 0, 0, 0]).to_string()
}

pub fn solve_b() -> String {
    run_for_part_b(&"input21.txt".to_string()).to_string()
}

#[derive(Debug, Eq, PartialEq)]
struct Inst {
    op_code: String,
    vals: [usize; 3],
}

impl Clone for Inst {
    fn clone(&self) -> Inst {
        Inst {
            op_code: String::from(self.op_code.clone()),
            vals: self.vals,
        }
    }
}

fn parse_input(filename: String) -> (Vec<Inst>, usize) {
    lazy_static! {
        static ref INST_REGEX: Regex = Regex::new(r"^(\w+) (\d+) (\d+) (\d+)").unwrap();
    }
    let raw_data = utility::load_strings(filename);
    let mut data_pass = raw_data.iter();
    let mut samples: Vec<Inst> = Vec::new();

    let ip_dec_raw = data_pass.next().unwrap();

    let ipreg = ip_dec_raw
        .split(' ')
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();

    for line in data_pass {
        if let Some(caps) = INST_REGEX.captures(line) {
            let vals = [
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
                caps[4].parse().unwrap(),
            ];
            let op_code = caps[1].to_string();
            samples.push(Inst { op_code, vals });
        }
    }

    (samples, ipreg)
}

fn process_instruction(regs: &mut [usize; 6], inst: &Inst) {
    let vals = inst.vals;
    match inst.op_code.as_str() {
        "addr" => {
            regs[vals[2]] = regs[vals[0]] + regs[vals[1]];
        }
        "addi" => {
            regs[vals[2]] = regs[vals[0]] + vals[1];
        }
        "mulr" => {
            regs[vals[2]] = regs[vals[0]] * regs[vals[1]];
        }
        "muli" => {
            regs[vals[2]] = regs[vals[0]] * vals[1];
        }
        "banr" => {
            regs[vals[2]] = regs[vals[0]] & regs[vals[1]];
        }
        "bani" => {
            regs[vals[2]] = regs[vals[0]] & vals[1];
        }
        "borr" => {
            regs[vals[2]] = regs[vals[0]] | regs[vals[1]];
        }
        "bori" => {
            regs[vals[2]] = regs[vals[0]] | vals[1];
        }
        "setr" => {
            regs[vals[2]] = regs[vals[0]];
        }
        "seti" => {
            regs[vals[2]] = vals[0];
        }
        "gtir" => {
            regs[vals[2]] = if vals[0] > regs[vals[1]] { 1 } else { 0 };
        }
        "gtri" => {
            regs[vals[2]] = if regs[vals[0]] > vals[1] { 1 } else { 0 };
        }
        "gtrr" => {
            regs[vals[2]] = if regs[vals[0]] > regs[vals[1]] { 1 } else { 0 };
        }
        "eqir" => {
            regs[vals[2]] = if vals[0] == regs[vals[1]] { 1 } else { 0 };
        }
        "eqri" => {
            regs[vals[2]] = if regs[vals[0]] == vals[1] { 1 } else { 0 };
        }
        "eqrr" => {
            regs[vals[2]] = if regs[vals[0]] == regs[vals[1]] { 1 } else { 0 };
        }
        _ => panic!("Bad opcode?"),
    }
}

fn run_for_part_a(filename: &String, initial_reg: [usize; 6]) -> usize {
    let (data, ip) = parse_input(filename.to_string());
    let ip_reg = ip;
    let mut regs = initial_reg;
    loop {
        let inst = &data[regs[ip_reg]];
        process_instruction(&mut regs, &inst);
        if inst.op_code == "eqrr" {
            return regs[4];
        }
        if regs[ip_reg] + 1 >= data.len() {
            break;
        }

        regs[ip_reg] += 1;
    }
    0
}

//Originally this was just the loop from run from part A with a hashset, but it takes about 17 seconds to run because of all of the abstraction.
//So I just put the minimized version of what my input is doing.
fn run_for_part_b(filename: &String) -> usize {
    let (data, _) = parse_input(filename.to_string());
    let base_val = data[7].vals[0];
    let mut seen = HashSet::new();

    let mut curr = 0;
    let mut previous;
    loop {
        previous = curr;
        curr = curr | 0x10000;
        let mut new = base_val + (curr & 255);
        new = (new * 65899) & 16777215;
        new += (curr >> 8) & 255;
        new = (new * 65899) & 16777215;
        new += (curr >> 16) & 255;
        new = (new * 65899) & 16777215;

        if !seen.insert(new) {
            return previous;
        }
        curr = new;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_initial_input21() {
        let actual = run_for_part_b(&"input21.txt".to_string());

        assert_eq!(actual, 12502875);
    }
}
