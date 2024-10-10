use std::collections::HashMap;
use crate::parser::ASTNode;

pub struct Interpreter {
    variables: HashMap<String, ASTNode>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, ast: ASTNode) -> String {
        match ast {
            ASTNode::Command(command, args) => self.execute_command(command, args),
            ASTNode::Assignment(name, value) => self.assign_variable(name, *value),
            ASTNode::Variable(name) => self.get_variable(name),
            ASTNode::Number(n) => n.to_string(),
            ASTNode::String(s) => format!("\"{}\"", s),
            ASTNode::Boolean(b) => b.to_string(),
        }
    }

    fn execute_command(&self, command: String, args: Vec<ASTNode>) -> String {
        match command.as_str() {
            "cast" => self.cast_spell(args),
            "summon" => self.summon_creature(args),
            "enchant" => self.enchant_item(args),
            "scry" => self.scry(args),
            _ => format!("The runes for '{}' are unfamiliar.", command),
        }
    }

    fn assign_variable(&mut self, name: String, value: ASTNode) -> String {
        self.variables.insert(name.clone(), value);
        format!("Variable {} has been inscribed with arcane power.", name)
    }

    fn get_variable(&self, name: String) -> String {
        match self.variables.get(&name) {
            Some(value) => format!("{:?}", value),
            None => format!("The runes for {} have not been inscribed.", name),
        }
    }

    fn cast_spell(&self, args: Vec<ASTNode>) -> String {
        if args.is_empty() {
            return "Error: No spell runes provided".to_string();
        }
        match &args[0] {
            ASTNode::Variable(spell) => {
                if let Some(ASTNode::String(spell_name)) = self.variables.get(spell) {
                    match spell_name.as_str() {
                        "fireball" => "A sphere of flame erupts from your fingertips!".to_string(),
                        "frostbolt" => "Crystals of ice form and launch from your palm!".to_string(),
                        "heal" => "Soothing energies flow from your hands, mending wounds.".to_string(),
                        _ => format!("You trace the runes for {}, but the magic fizzles.", spell_name),
                    }
                } else {
                    format!("The spell '{}' has not been properly inscribed.", spell)
                }
            },
            _ => "Error: Invalid spell name".to_string(),
        }
    }

    fn summon_creature(&self, args: Vec<ASTNode>) -> String {
        if args.is_empty() {
            return "Error: No creature specified in the summoning circle".to_string();
        }
        match &args[0] {
            ASTNode::Variable(name) => {
                if let Some(ASTNode::String(creature)) = self.variables.get(name) {
                    format!("A mystical portal opens, and a {} steps forth!", creature)
                } else {
                    format!("The runes for {} are unclear. Summoning fails.", name)
                }
            },
            _ => "Error: Invalid creature name".to_string(),
        }
    }

    fn enchant_item(&self, args: Vec<ASTNode>) -> String {
        if args.len() < 2 {
            return "Error: Enchantment requires both an item and a magical property".to_string();
        }
        match (&args[0], &args[1]) {
            (ASTNode::Variable(item_name), ASTNode::Variable(property_name)) => {
                let item = self.variables.get(item_name).and_then(|v| match v {
                    ASTNode::String(s) => Some(s.as_str()),
                    _ => None,
                });
                let property = self.variables.get(property_name).and_then(|v| match v {
                    ASTNode::String(s) => Some(s.as_str()),
                    _ => None,
                });
                match (item, property) {
                    (Some(i), Some(p)) => format!("Arcane energies swirl around the {}, imbuing it with the power of {}.", i, p),
                    _ => "Error: Invalid item or property for enchantment".to_string(),
                }
            },
            _ => "Error: Invalid item or property for enchantment".to_string(),
        }
    }

    fn scry(&self, args: Vec<ASTNode>) -> String {
        if args.is_empty() {
            return "You gaze into your crystal ball, but see only swirling mists.".to_string();
        }
        let vision = args.iter().filter_map(|arg| match arg {
            ASTNode::Variable(name) => self.variables.get(name).and_then(|v| match v {
                ASTNode::String(s) => Some(s.clone()),
                _ => None,
            }),
            _ => None,
        }).collect::<Vec<String>>().join(" ");
        
        if vision.is_empty() {
            "The scrying mists are thick. Your vision is unclear.".to_string()
        } else {
            format!("As you focus, visions of {} begin to form in your mind's eye.", vision)
        }
    }
}