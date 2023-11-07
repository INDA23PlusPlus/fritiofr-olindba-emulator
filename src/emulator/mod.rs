mod instructions;
pub mod mem;

use crate::consts;

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
            consts::ADD => instructions::exec_add(&code, &mut code_pointer, &mut mem),
            consts::SUB => instructions::exec_sub(&code, &mut code_pointer, &mut mem),
            consts::MUL => instructions::exec_mul(&code, &mut code_pointer, &mut mem),
            consts::DIV => instructions::exec_div(&code, &mut code_pointer, &mut mem),
            consts::MOV => instructions::exec_mov(&code, &mut code_pointer, &mut mem),
            consts::CND => instructions::exec_cmp(&code, &mut code_pointer, &mut mem),
            consts::JMP => instructions::exec_jmp(&code, &mut code_pointer, &mut mem),
            consts::JEQ => instructions::exec_jeq(&code, &mut code_pointer, &mut mem),
            consts::JNE => instructions::exec_jne(&code, &mut code_pointer, &mut mem),
            consts::JGT => instructions::exec_jgt(&code, &mut code_pointer, &mut mem),
            consts::JLT => instructions::exec_jlt(&code, &mut code_pointer, &mut mem),
            consts::JGE => instructions::exec_jge(&code, &mut code_pointer, &mut mem),
            consts::JLE => instructions::exec_jle(&code, &mut code_pointer, &mut mem),
            byte => {
                println!("Unknown instruction: {}", byte);
                std::process::exit(1);
            }
        }
    }

    println!("Program finished");
    println!("{}", mem);
}
