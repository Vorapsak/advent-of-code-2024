use advent::prelude::*;

#[derive(HasParser, Debug)]
#[parse(sep_by = "\n")]
struct Input {
    #[parse(before = "Register A: ")]
    a: u32,
    #[parse(before = "Register B: ")]
    b: u32,
    #[parse(before = "Register C: ")]
    c: u32,
    #[parse(before = "\nProgram: ")]
    program: String
}

#[derive(Debug)]
struct CPU {
    a: u32,
    b: u32,
    c: u32
}

impl CPU {
    fn get_combo_operand(self: &Self, val: u8) -> u32 {
        match val {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!()
        }
    }
}

#[part_one]
fn part_one(input: Input) -> String {
    let program: Vec<u8> = input.program.split(",").map(|s| s.parse().unwrap()).collect();

    let mut cpu = CPU {
        a: input.a,
        b: input.b,
        c: input.c
    };

    let mut output_buffer: Vec<String> = Vec::new();
    let mut instruction_pointer = 0;
    
    loop {
        if instruction_pointer > program.len()-1 { break; }
        match program[instruction_pointer] {
            0 => {
                let other = cpu.get_combo_operand(program[instruction_pointer+1]);
                let divisor = 2_u32.pow(other);
                cpu.a /= divisor;
                instruction_pointer += 2;
            },
            1 => {
                let other = program[instruction_pointer+1];
                cpu.b ^= other as u32;
                instruction_pointer += 2;
            },
            2 => {
                let other = cpu.get_combo_operand(program[instruction_pointer+1]);
                cpu.b = other % 8;
                instruction_pointer += 2;
            },
            3 => {
                let other = program[instruction_pointer+1];
                if cpu.a != 0 {
                    instruction_pointer = other as usize;
                } else {
                    instruction_pointer += 2;
                }
                
            },
            4 => {
                cpu.b ^= cpu.c;
                instruction_pointer += 2;
                
            },
            5 => {
                let other = cpu.get_combo_operand(program[instruction_pointer+1]);
                output_buffer.push((other % 8).to_string());
                instruction_pointer += 2;
            },
            6 => {
                let other = cpu.get_combo_operand(program[instruction_pointer+1]);
                let divisor = 2_u32.pow(other);
                cpu.b = cpu.a / divisor;
                instruction_pointer += 2;
                
            },
            7 => {
                let other = cpu.get_combo_operand(program[instruction_pointer+1]);
                let divisor = 2_u32.pow(other);
                cpu.c = cpu.a / divisor;
                instruction_pointer += 2;
                
            },
            _ => panic!()
        }
    }

    output_buffer.join(",").into()
}

#[derive(Debug)]
struct CPU2 {
    a: u64,
    b: u64,
    c: u64
}

impl CPU2 {
    fn get_combo_operand(self: &Self, val: u8) -> u64 {
        match val {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!()
        }
    }
}

#[part_two]
fn part_two(input: Input) -> u64 {
    let program: Vec<u8> = input.program.split(",").map(|s| s.parse().unwrap()).collect();
    
    //0b_011_010_110_111_010_111_101

    dbg!(&program);

    for a in (820_866_749+2_u64.pow(45)..).step_by(2_u64.pow(31) as usize) {
        if sim_and_check(a, &program) {
            return a;
        }
    }

    0
}

fn sim_and_check(a: u64, program: &Vec<u8>) -> bool {
    let mut cpu = CPU2 {
        a: a,
        b: 0,
        c: 0
    };

    
    let mut output_buffer: Vec<String> = Vec::new();
    let mut instruction_pointer = 0;

    loop {
        //dbg!(&cpu, &output_buffer, instruction_pointer);
        match tick(&mut cpu, instruction_pointer, program, &mut output_buffer) {
            Some(ip) => {
                instruction_pointer = ip;
                if output_buffer.len() > program.len() { return false; }
                for i in 0..output_buffer.len() {
                    if program[i] != output_buffer[i].parse().unwrap() {
                        if output_buffer.len() > 15 {
                            dbg!(&output_buffer.len(), &output_buffer, a);
                        }
                        return false;
                    }
                }
                if output_buffer.len() == program.len() {
                    return true;
                }
            },
            None => { dbg!(&output_buffer, a); return false; }
        }
    }
}

fn tick (cpu: &mut CPU2, instruction_pointer: usize, program: &Vec<u8>, output_buffer: &mut Vec<String>) -> Option<usize> {
    if instruction_pointer >= program.len() { return None; }
    match program[instruction_pointer] {
        0 => {
            let other = cpu.get_combo_operand(program[instruction_pointer+1]);
            let divisor = 2_u64.pow(other.try_into().unwrap());
            cpu.a /= divisor;
            Some(instruction_pointer + 2)
        },
        1 => {
            let other = program[instruction_pointer+1];
            cpu.b ^= other as u64;
            Some(instruction_pointer + 2)
        },
        2 => {
            let other = cpu.get_combo_operand(program[instruction_pointer+1]);
            cpu.b = other % 8;
            Some(instruction_pointer + 2)
        },
        3 => {
            let other = program[instruction_pointer+1];
            if cpu.a != 0 {
                Some(other as usize)
            } else {
                Some(instruction_pointer + 2)
            }
        },
        4 => {
            cpu.b ^= cpu.c;
            Some(instruction_pointer + 2)
        },
        5 => {
            let other = cpu.get_combo_operand(program[instruction_pointer+1]);
            output_buffer.push((other % 8).to_string());
            Some(instruction_pointer + 2)
        },
        6 => {
            let other = cpu.get_combo_operand(program[instruction_pointer+1]);
            let divisor = 2_u64.pow(other.try_into().unwrap());
            cpu.b = cpu.a / divisor;
            Some(instruction_pointer + 2)
        },
        7 => {
            let other = cpu.get_combo_operand(program[instruction_pointer+1]);
            let divisor = 2_u64.pow(other.try_into().unwrap());
            cpu.c = cpu.a / divisor;
            Some(instruction_pointer + 2)
        },
        _ => panic!()
        }
}

harness!(part_1: "3,6,7,0,5,7,3,1,4".to_string());