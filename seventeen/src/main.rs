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
    let mut a = input.a;
    let mut b = input.b;
    let mut c = input.c;

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

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!(part_1: "3,6,7,0,5,7,3,1,4".to_string());