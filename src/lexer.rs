use std::collections::HashMap;

use crate::consts;

pub struct Lexer<'a> {
    chars: &'a str,
    cur_ind: usize,
    jumps: HashMap<String, u8>
}

impl<'a> Lexer<'a> {
    pub fn new(text_inp: &str) -> Lexer {
        let mut jumps = HashMap::new();

        Lexer{
            chars: text_inp,
            cur_ind: 0,
            jumps
        }
    }

    fn find_jumps(&mut self) {
        let mut cur_ind = 0;
        loop {
            let token = self.next_token();
            match token.as_str() {
                "add" | "sub" | "mul" | "div" | "mov" | "cnd" => {
                    cur_ind += 1;
                },
                "jmp" | "jeq" | "jne" | "jgt" | "jlt" | "jge" | "jle" => {
                    cur_ind += 6;
                    self.next_token();
                }
                "ax" | "bx" | "cx" | "dx" | "ex" | "fx" => {
                    cur_ind += 2;
                },
                _ => {
                    if token.len() == 0 {
                        break;
                    }
                    if token.chars().nth(0).unwrap() == '.' {
                        self.jumps.insert(token, cur_ind);
                    }
                    else {
                        cur_ind += 5;
                    }
                }
            }
            
        }
    }

    pub fn generate(&mut self) -> Vec<u8> {
        self.find_jumps();
        self.cur_ind = 0;
        let mut out = vec![];
        loop {
            let token = self.next_token();
            match token.as_str() {
                "add" => {
                    out.push(consts::ADD);
                },
                "sub" => {
                    out.push(consts::SUB);
                },
                "mul" => {
                    out.push(consts::MUL);
                },
                "div" => {
                    out.push(consts::DIV);
                },
                "cnd" => {
                    out.push(consts::CND);
                },
                "jmp" => {
                    out.push(consts::JMP);
                    let jump_to = self.next_token();
                    out.push(2);
                    out.append(&mut self.parse_int(*self.jumps.get(&jump_to).unwrap() as usize));
                },
                "jeq" => {
                    out.push(consts::JEQ);
                    let jump_to = self.next_token();
                    out.push(2);
                    out.append(&mut self.parse_int(*self.jumps.get(&jump_to).unwrap() as usize));
                },
                "jne" => {
                    out.push(consts::JNE);
                    let jump_to = self.next_token();
                    out.push(2);
                    out.append(&mut self.parse_int(*self.jumps.get(&jump_to).unwrap() as usize));
                },
                "jgt" => {
                    out.push(consts::JGT);
                    let jump_to = self.next_token();
                    out.push(2);
                    out.append(&mut self.parse_int(*self.jumps.get(&jump_to).unwrap() as usize));
                },
                "jlt" => {
                    out.push(consts::JLT);
                    let jump_to = self.next_token();
                    out.push(2);
                    out.append(&mut self.parse_int(*self.jumps.get(&jump_to).unwrap() as usize));
                },
                "jge" => {
                    out.push(consts::JGE);
                    let jump_to = self.next_token();
                    out.push(2);
                    out.append(&mut self.parse_int(*self.jumps.get(&jump_to).unwrap() as usize));
                },
                "jle" => {
                    out.push(consts::JLE);
                    let jump_to = self.next_token();
                    out.push(2);
                    out.append(&mut self.parse_int(*self.jumps.get(&jump_to).unwrap() as usize));
                },
                "ax" => {
                    out.push(1);
                    out.push(consts::AX);
                },
                "bx" => {
                    out.push(1);
                    out.push(consts::BX);
                },
                "cx" => {
                    out.push(1);
                    out.push(consts::CX);
                },
                "dx" => {
                    out.push(1);
                    out.push(consts::DX);
                },
                "ex" => {
                    out.push(1);
                    out.push(consts::EX);
                },
                "fx" => {
                    out.push(1);
                    out.push(consts::FX);
                },
                _ => { 
                    if token.len() == 0 {
                        break;
                    }
                    if token.chars().nth(0).unwrap() == '.' {
                        continue;
                    }
                    out.push(2);
                    let num = token.parse::<usize>().unwrap();
                    out.append(&mut self.parse_int(num));
                }
            };
        }    
        out
    }

    fn next_token(&mut self) -> String {
        let mut string = String::new();
        loop {
            if let Some(c) = self.chars.chars().nth(self.cur_ind) {
                self.cur_ind += 1;
                if c == '\n' || c == ' ' {
                    return string;
                }
                string.push(c);
            }
            else {
                return string;
            }
        }
    }

    fn parse_int(&self, num: usize) -> Vec<u8> {
        let mut out = vec![];
        for i in 0..4 {
            let mut cur = 0;
            for j in 0..5 {
                if ((1 << (j + i * 5)) & num) != 0 {
                    cur += 1 << j;
                }
            }
            out.push(cur);
        }
        out.reverse();
        out
    }
}