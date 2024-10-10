mod lexer;
mod parser;
mod interpreter;

fn main() {
    println!("Welcome to the RunicScript interpreter!");
    
    let inputs = vec![
        "cast fireball",
        "summon dragon",
        "enchant sword with fire",
        "cast invisibility",
    ];

    for input in inputs {
        println!("\nExecuting: {}", input);
        let tokens = lexer::tokenize(input);
        
        match parser::parse(tokens) {
            Ok(ast) => {
                println!("AST: {:?}", ast);
                let result = interpreter::interpret(ast);
                println!("Result: {}", result);
            }
            Err(e) => println!("Parsing error: {}", e),
        }
    }
}