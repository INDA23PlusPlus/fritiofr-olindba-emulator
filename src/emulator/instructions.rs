use crate::consts;
use crate::emulator::mem::Mem;

/// Executes a jump operation
pub fn exec_jmp(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    exec_jmp_helper(code, code_pointer, mem, |_| true)
}

/// Executes a jump if equal operation
pub fn exec_jeq(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    exec_jmp_helper(code, code_pointer, mem, |val| val == 0)
}

/// Executes a jump if not equal operation
pub fn exec_jne(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    exec_jmp_helper(code, code_pointer, mem, |val| val != 0)
}

/// Executes a jump if greater than operation
pub fn exec_jgt(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    exec_jmp_helper(code, code_pointer, mem, |val| val > 0)
}

/// Executes a jump if less than operation
pub fn exec_jlt(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    exec_jmp_helper(code, code_pointer, mem, |val| val < 0)
}

/// Executes a jump if greater than or equal operation
pub fn exec_jge(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    exec_jmp_helper(code, code_pointer, mem, |val| val >= 0)
}

/// Executes a jump if less than or equal operation
pub fn exec_jle(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    exec_jmp_helper(code, code_pointer, mem, |val| val <= 0)
}

/// Executes a compare operation
pub fn exec_cmp(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
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

/// Executes an addition operation
pub fn exec_add(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    exec_op_helper(code, code_pointer, mem, |a, b| a.wrapping_add(b));
}

/// Executes a subtraction operation
pub fn exec_sub(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    exec_op_helper(code, code_pointer, mem, |a, b| a.wrapping_sub(b));
}

/// Executes a multiplication operation
pub fn exec_mul(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    exec_op_helper(code, code_pointer, mem, |a, b| a.wrapping_mul(b));
}

/// Executes a mov operation
pub fn exec_mov(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
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

/// Executes a division operation
pub fn exec_div(code: &Vec<u8>, code_pointer: &mut u32, mem: &mut Mem) {
    let mut rest = 0;

    exec_op_helper(code, code_pointer, mem, |a, b| {
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

/// Represents a value that can be parsed from the code
enum ValueRes {
    Reg(u8),
    Const(i32),
}

/// Parses a value either a int literal or a register.
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

/// Executes an operation, this is used by the add, sub, div and mul instructions
fn exec_op_helper(
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

/// Helper for jump operations
fn exec_jmp_helper(
    code: &Vec<u8>,
    code_pointer: &mut u32,
    mem: &mut Mem,
    should_jmp: fn(i32) -> bool,
) {
    let a = parse_val(code, code_pointer);

    let a_val = match a {
        ValueRes::Reg(_) => {
            println!("Cannot jump to a register");
            std::process::exit(1);
        }
        ValueRes::Const(val) => val,
    };

    if should_jmp(mem.get_reg(consts::FX).unwrap()) {
        *code_pointer = a_val as u32;
    }
}
