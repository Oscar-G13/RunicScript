mod lexer;
mod parser;
mod interpreter;
mod repl;

use interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new();
    repl::start_repl(&mut interpreter);
}