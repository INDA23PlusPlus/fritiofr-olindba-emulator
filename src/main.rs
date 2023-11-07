fn main() {
    let mut lexer = emulator::lexer::Lexer::new(".prime_loop\ndiv cx bx dx\ncnd ax 0\njeq .prime_loop_end\nadd cx 1 cx\ncnd cx bx\njne .prime_loop\n.prime_loop_end");
    let ret = lexer.generate();
    println!("{:?}", ret);
    println!("{}", ret.len());
}
