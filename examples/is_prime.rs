const CODE: &'static str = include_str!("is_prime.asm");

fn main() {
    let code = emulator::lexer::Lexer::new(CODE).generate();
    emulator::emulator::run(code)
}
