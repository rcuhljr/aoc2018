use super::utility;
use regex::Regex;

pub fn solve_a() -> String {
    run_program("input19.txt".to_string(), [0, 0, 0, 0, 0, 0]).to_string()
}

pub fn solve_b() -> String {
    run_program("input19.txt".to_string(), [1, 0, 0, 0, 0, 0]).to_string()
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

fn run_program(filename: String, initial_reg: [usize; 6]) -> usize {
    let (data, ip) = parse_input(filename);
    let ip_reg = ip;
    let mut regs = initial_reg;

    let mut counter = 0;
    loop {
        let inst = &data[regs[ip_reg]];
        process_instruction(&mut regs, &inst);
        if regs[ip_reg] + 1 >= data.len() {
            break;
        }
        counter += 1;
        if counter > 50 {
            let goal = regs[3];
            let mut divisors = vec![];
            for i in 1..goal + 1 {
                if goal % i == 0 {
                    divisors.push(i);
                }
            }

            return divisors.iter().sum();
        }
        regs[ip_reg] += 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_initial_input18() {
        let actual = parse_input("./src/day19/test.txt".to_string());
        assert_eq!(
            actual.0,
            [
                Inst {
                    op_code: String::from("seti"),
                    vals: [5, 0, 1]
                },
                Inst {
                    op_code: String::from("seti"),
                    vals: [6, 0, 2]
                },
                Inst {
                    op_code: String::from("addi"),
                    vals: [0, 1, 0]
                },
                Inst {
                    op_code: String::from("addr"),
                    vals: [1, 2, 3]
                },
                Inst {
                    op_code: String::from("setr"),
                    vals: [1, 0, 0]
                },
                Inst {
                    op_code: String::from("seti"),
                    vals: [8, 0, 4]
                },
                Inst {
                    op_code: String::from("seti"),
                    vals: [9, 0, 5]
                }
            ]
        );

        assert_eq!(actual.1, 0);
    }

}
