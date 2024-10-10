use crate::parser::ASTNode;

pub fn interpret(ast: ASTNode) -> String {
    match ast {
        ASTNode::Command(command, arguments) => {
            match command.as_str() {
                "cast" => cast_spell(arguments),
                "summon" => summon_creature(arguments),
                "enchant" => enchant_item(arguments),
                "scry" => scry(arguments),
                _ => format!("The runes for '{}' are unfamiliar.", command),
            }
        }
    }
}

fn cast_spell(args: Vec<String>) -> String {
    if args.is_empty() {
        return "Error: No spell runes provided".to_string();
    }
    match args[0].as_str() {
        "fireball" => "A sphere of flame erupts from your fingertips!".to_string(),
        "frostbolt" => "Crystals of ice form and launch from your palm!".to_string(),
        "heal" => "Soothing energies flow from your hands, mending wounds.".to_string(),
        _ => format!("You trace the runes for {}, but the magic fizzles.", args[0]),
    }
}

fn summon_creature(args: Vec<String>) -> String {
    if args.is_empty() {
        return "Error: No creature specified in the summoning circle".to_string();
    }
    format!("A mystical portal opens, and a {} steps forth!", args[0])
}

fn enchant_item(args: Vec<String>) -> String {
    if args.len() < 2 {
        return "Error: Enchantment requires both an item and a magical property".to_string();
    }
    format!("Arcane energies swirl around the {}, imbuing it with the power of {}.", args[0], args[1])
}

fn scry(args: Vec<String>) -> String {
    if args.is_empty() {
        return "You gaze into your crystal ball, but see only swirling mists.".to_string();
    }
    format!("As you focus on {}, visions begin to form in your mind's eye.", args.join(" "))
}