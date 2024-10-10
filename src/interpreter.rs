use crate::parser::ASTNode;

pub fn interpret(ast: ASTNode) -> String {
    match ast {
        ASTNode::Command(command, arguments) => {
            match command.as_str() {
                "cast" => cast_spell(arguments),
                "summon" => summon_creature(arguments),
                "enchant" => enchant_item(arguments),
                _ => format!("Unknown command: '{}'", command),
            }
        }
    }
}

fn cast_spell(args: Vec<String>) -> String {
    if args.is_empty() {
        return "Error: No spell specified".to_string();
    }
    match args[0].as_str() {
        "fireball" => "You cast a fiery ball of magic!".to_string(),
        "frostbolt" => "You launch a bolt of icy magic!".to_string(),
        "heal" => "A warm, healing light emanates from your hands.".to_string(),
        _ => format!("You attempt to cast {}, but nothing happens.", args[0]),
    }
}

fn summon_creature(args: Vec<String>) -> String {
    if args.is_empty() {
        return "Error: No creature specified".to_string();
    }
    format!("You summon a {}!", args[0])
}

fn enchant_item(args: Vec<String>) -> String {
    if args.len() < 2 {
        return "Error: Enchant requires an item and an enchantment".to_string();
    }
    format!("You enchant the {} with {}.", args[0], args[1])
}