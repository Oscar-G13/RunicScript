use std::io::{self, Write};
use crate::lexer;
use crate::parser;
use crate::interpreter::Interpreter;

pub fn start_repl(interpreter: &mut Interpreter) {
    println!("Welcome to the RunicScript Arcane Console!");
    println!("Speak your incantations, or whisper 'vanish' to close the portal.");

    loop {
        print!("🔮 > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "vanish" {
            println!("The mystical portal closes. Your RunicScript session ends.");
            break;
        }

        let tokens = lexer::tokenize(input);
        
        match parser::parse(tokens) {
            Ok(ast) => {
                let result = interpreter.interpret(ast);
                println!("✨ {}", result);
            }
            Err(e) => println!("🌋 Arcane error: {}", e),
        }
    }
}