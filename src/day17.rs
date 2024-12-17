use crate::util::read_input;
use core::panic;
use std::io::BufRead;

enum Opcode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}
#[derive(Debug)]
struct Debugger {
    instr: Vec<u8>,
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    ip: usize,
    output: Vec<u64>,
}

impl Debugger {
    fn execute(&mut self) -> bool {
        if let Some(instruction) = self.fetch_and_decode() {
            match instruction {
                Opcode::ADV => self.adv(),
                Opcode::BXL => self.bxl(),
                Opcode::BST => self.bst(),
                Opcode::JNZ => self.jnz(),
                Opcode::BXC => self.bxc(),
                Opcode::OUT => self.out(),
                Opcode::BDV => self.bdv(),
                Opcode::CDV => self.cdv(),
            }
            return true;
        }
        false
    }

    fn fetch_and_decode(&self) -> Option<Opcode> {
        let opcode = self.instr.get(self.ip);
        if opcode == None {
            return None;
        }
        let opcode = opcode.unwrap();
        match opcode {
            0 => Some(Opcode::ADV),
            1 => Some(Opcode::BXL),
            2 => Some(Opcode::BST),
            3 => Some(Opcode::JNZ),
            4 => Some(Opcode::BXC),
            5 => Some(Opcode::OUT),
            6 => Some(Opcode::BDV),
            7 => Some(Opcode::CDV),
            _ => panic!(),
        }
    }

    fn adv(&mut self) {
        let operand = self.get_combo();
        self.reg_a = self.reg_a / 2_u64.pow(operand as u32);
        self.ip += 2;
    }

    fn bxl(&mut self) {
        let b = self.reg_b;
        let l = self.instr[self.ip + 1] as u64;
        self.reg_b = b ^ l;
        self.ip += 2;
    }

    fn bst(&mut self) {
        let operand = self.get_combo();
        self.reg_b = operand % 8;
        self.ip += 2;
    }

    fn jnz(&mut self) {
        if self.reg_a == 0 {
            self.ip += 2;
        } else {
            self.ip = self.instr[self.ip + 1] as usize;
        }
    }

    fn bxc(&mut self) {
        let b = self.reg_b;
        let c = self.reg_c;
        self.reg_b = b ^ c;
        self.ip += 2;
    }

    fn out(&mut self) {
        self.output.push(self.get_combo() % 8);
        self.ip += 2;
    }

    fn bdv(&mut self) {
        let operand = self.get_combo();
        self.reg_b = self.reg_a / 2_u64.pow(operand as u32);
        self.ip += 2;
    }

    fn cdv(&mut self) {
        let operand = self.get_combo();
        self.reg_c = self.reg_a / 2_u64.pow(operand as u32);
        self.ip += 2;
    }

    fn get_combo(&self) -> u64 {
        let operand = self.instr[self.ip + 1];
        if operand <= 3 {
            operand as u64
        } else {
            match operand {
                4 => self.reg_a,
                5 => self.reg_b,
                6 => self.reg_c,
                _ => panic!(),
            }
        }
    }
}

pub fn puzzle1() {
    let mut debugger = parse_input();

    while debugger.execute() {}

    print!("day 17, puzzle 1: ");

    for i in 0..debugger.output.len() - 1 {
        print!("{},", debugger.output[i])
    }
    println!("{}", debugger.output.last().unwrap())
}

pub fn puzzle2() {
    let debugger = parse_input();
    let res =recurse(debugger.instr, vec![]);
    println!("day 17, puzzle 2: {res}");
    
}

fn recurse(instructions: Vec<u8>, digits: Vec<u64>) -> u64 {
    let digit_to_find = instructions.len() - digits.len() - 1;

    let mut init_a = 0;
    for i in 0..digits.len() {
        init_a += digits[i] * 2_u64.pow(45 - 3 * i as u32);
    }

    let mut result = std::u64::MAX;
    for i in 0..8 {
        if i == 0 && init_a == 0 {
            continue; // make sure debugger.output has correct length
        }

        let mut debugger = Debugger {
            instr: instructions.clone(),
            reg_a: init_a + i * 2_u64.pow(3 * digit_to_find as u32),
            reg_b: 0,
            reg_c: 0,
            ip: 0,
            output: vec![],
        };

        while debugger.execute() {}

        if debugger.output[digit_to_find] == instructions[digit_to_find] as u64 {
            let mut digits = digits.clone();
            digits.push(i);
            if digits.len() == instructions.len() {
                if compare_vectors(&instructions, &debugger.output) {
                    let mut candidate = 0;
                    for i in 0..digits.len() {
                        candidate += digits[i] * 2_u64.pow(45 - 3 * i as u32);
                    }
                    return candidate
                }
            } else {
                let candidate = recurse(instructions.clone(), digits);
                if candidate < result {
                    result = candidate;
                }
            }
        }
    }
    result
}

pub fn compare_vectors(v1: &Vec<u8>, v2: &Vec<u64>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    for i in 0..v1.len() {
        if v1[i] as u64 != v2[i] {
            return false;
        }
    }
    return true;
}

fn parse_input() -> Debugger {
    let reader = read_input("input/day_17.txt");
    let lines = reader.lines();
    let lines = lines.map(|l| l.unwrap()).collect::<Vec<_>>();
    let register_a = lines[0].split(": ").collect::<Vec<_>>()[1].parse().unwrap();
    let register_b = lines[1].split(": ").collect::<Vec<_>>()[1].parse().unwrap();
    let register_c = lines[2].split(": ").collect::<Vec<_>>()[1].parse().unwrap();
    let instructions = lines[4].split(": ").collect::<Vec<_>>()[1]
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    Debugger {
        instr: instructions,
        reg_a: register_a,
        reg_b: register_b,
        reg_c: register_c,
        ip: 0,
        output: vec![],
    }
}
