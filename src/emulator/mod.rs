pub mod mem;

use crate::consts;
use mem::Mem;

/// Runs a program with the cpu emulator
///
/// # Arguments
/// * `code` - The program to run
pub fn run(code: Vec<u8>) {
    // This keeps track of where in the code we are, will be passed around to functions to seperate
    // code
    let mut code_pointer: u32 = 0;
    let mut mem = mem::Mem::new();

    loop {
        if code_pointer >= code.len() as u32 {
            break;
        }
        let instr = code[code_pointer as usize];
        code_pointer += 1;

        match instr {
            consts::ADD => exec_add(&code, &mut code_pointer, &mut mem),
            consts::SUB => exec_sub(&code, &mut code_pointer, &mut mem),
            consts::MUL => exec_mul(&code, &mut code_pointer, &mut mem),
            consts::DIV => exec_div(&code, &mut code_pointer, &mut mem),
            consts::MOV => exec_mov(&code, &mut code_pointer, &mut mem),
            consts::CND => exec_cmp(&code, &mut code_pointer, &mut mem),
            consts::JMP | consts::JEQ | consts::JNE | consts::JGT | consts::JLT | consts::JGE => {
                exec_jmp(&code, &mut code_pointer, &mut mem, instr)
            }
            byte => {
                println!("Unknown instruction: {}", byte);
                std::process::exit(1);
            }
        }
    }

    println!("Program finished");
    println!("{}", mem);
}

fn exec_jmp(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem, jmp_type: u8) {
    let a = parse_val(code, code_pointer);

    let a_val = match a {
        ValueRes::Reg(_) => {
            println!("Cannot jump to a register");
            std::process::exit(1);
        }
        ValueRes::Const(val) => val,
    };

    match jmp_type {
        consts::JMP => {
            *code_pointer = a_val as u32;
        }
        consts::JEQ => {
            if mem.get_reg(consts::FX).unwrap() == 0 {
                *code_pointer = a_val as u32;
            }
        }
        consts::JNE => {
            if mem.get_reg(consts::FX).unwrap() != 0 {
                *code_pointer = a_val as u32;
            }
        }
        consts::JGT => {
            if mem.get_reg(consts::FX).unwrap() == 1 {
                *code_pointer = a_val as u32;
            }
        }
        consts::JLT => {
            if mem.get_reg(consts::FX).unwrap() == -1 {
                *code_pointer = a_val as u32;
            }
        }
        consts::JGE => {
            if mem.get_reg(consts::FX).unwrap() >= 0 {
                *code_pointer = a_val as u32;
            }
        }
        consts::JLE => {
            if mem.get_reg(consts::FX).unwrap() <= 0 {
                *code_pointer = a_val as u32;
            }
        }
        _ => {
            println!("Unknown jump type: {}", jmp_type);
            std::process::exit(1);
        }
    }
}

fn exec_cmp(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    let a = parse_val(code, code_pointer);
    let b = parse_val(code, code_pointer);

    let a_val = match a {
        ValueRes::Reg(reg) => mem.get_reg(reg).unwrap_or_else(|()| {
            println!("Unknown register: {}", reg);
            std::process::exit(1);
        }),
        ValueRes::Const(val) => val,
    };

    let b_val = match b {
        ValueRes::Reg(reg) => mem.get_reg(reg).unwrap_or_else(|()| {
            println!("Unknown register: {}", reg);
            std::process::exit(1);
        }),
        ValueRes::Const(val) => val,
    };

    if a_val == b_val {
        mem.set_reg(consts::FX, 0).unwrap();
    } else if a_val > b_val {
        mem.set_reg(consts::FX, 1).unwrap();
    } else {
        mem.set_reg(consts::FX, -1).unwrap();
    }
}

fn exec_op(
    code: &Vec<u8>,
    code_pointer: &mut u32,
    mem: &mut Mem,
    mut op: impl FnMut(i32, i32) -> i32,
) {
    let a = parse_val(code, code_pointer);
    let b = parse_val(code, code_pointer);
    let src = parse_val(code, code_pointer);

    let a_val = match a {
        ValueRes::Reg(reg) => mem.get_reg(reg).unwrap_or_else(|()| {
            println!("Unknown register: {}", reg);
            std::process::exit(1);
        }),
        ValueRes::Const(val) => val,
    };

    let b_val = match b {
        ValueRes::Reg(reg) => mem.get_reg(reg).unwrap_or_else(|()| {
            println!("Unknown register: {}", reg);
            std::process::exit(1);
        }),
        ValueRes::Const(val) => val,
    };

    let src_reg = match src {
        ValueRes::Reg(reg) => reg,
        ValueRes::Const(_) => {
            println!("Source value must be a register");
            std::process::exit(1);
        }
    };

    let res = op(a_val, b_val);

    mem.set_reg(src_reg, res).unwrap_or_else(|_| {
        println!("Unknown register: {}", src_reg);
        std::process::exit(1);
    });
}

fn exec_add(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    exec_op(code, code_pointer, mem, |a, b| a.wrapping_add(b));
}

fn exec_sub(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    exec_op(code, code_pointer, mem, |a, b| a.wrapping_sub(b));
}

fn exec_mul(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    exec_op(code, code_pointer, mem, |a, b| a.wrapping_mul(b));
}

fn exec_mov(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    let a = parse_val(code, code_pointer);
    let src = parse_val(code, code_pointer);

    let a_val = match a {
        ValueRes::Reg(reg) => mem.get_reg(reg).unwrap_or_else(|()| {
            println!("Unknown register: {}", reg);
            std::process::exit(1);
        }),
        ValueRes::Const(val) => val,
    };

    let src_reg = match src {
        ValueRes::Reg(reg) => reg,
        ValueRes::Const(_) => {
            println!("Source value must be a register");
            std::process::exit(1);
        }
    };

    mem.set_reg(src_reg, a_val).unwrap_or_else(|_| {
        println!("Unknown register: {}", src_reg);
        std::process::exit(1);
    });
}

fn exec_div(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    let mut rest = 0;

    exec_op(code, code_pointer, mem, |a, b| {
        if b == 0 {
            println!("Division by zero");
            std::process::exit(1);
        }

        let res = a.wrapping_div(b);
        rest = a.wrapping_rem(b);

        res
    });

    mem.set_reg(consts::AX, rest).unwrap();
}

enum ValueRes {
    Reg(u8),
    Const(i32),
}

/// Parses a value, aka either an int constant or a register
fn parse_val(code: &Vec<u8>, code_pointer: &mut u32) -> ValueRes {
    let ty_byte = code[*code_pointer as usize];

    match ty_byte {
        1 => {
            // This is a register

            *code_pointer += 1;
            let reg_byte = code[*code_pointer as usize];
            *code_pointer += 1;

            ValueRes::Reg(reg_byte)
        }
        2 => {
            // This is a constant

            *code_pointer += 1;
            let first = code[*code_pointer as usize];
            let second = code[*code_pointer as usize + 1];
            let third = code[*code_pointer as usize + 2];
            let fourth = code[*code_pointer as usize + 3];
            *code_pointer += 4;

            ValueRes::Const(
                ((first as i32) << 24)
                    | ((second as i32) << 16)
                    | ((third as i32) << 8)
                    | (fourth as i32),
            )
        }
        _ => {
            println!("Unknown value type: {}", ty_byte);
            std::process::exit(1);
        }
    }
}
