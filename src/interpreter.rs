use std::collections::HashMap;
use crate::parser::ASTNode;

pub struct Interpreter {
    variables: HashMap<String, ASTNode>,
    spells: HashMap<String, (Vec<String>, Vec<ASTNode>)>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            spells: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, ast: ASTNode) -> String {
        match ast {
            ASTNode::Command(command, args) => self.execute_command(command, args),
            ASTNode::Assignment(name, value) => self.assign_variable(name, *value),
            ASTNode::Variable(name) => self.get_variable(&name),
            ASTNode::Number(n) => n.to_string(),
            ASTNode::String(s) => s,
            ASTNode::Boolean(b) => b.to_string(),
            ASTNode::IfElse(condition, if_body, else_body) => self.execute_if_else(*condition, if_body, else_body),
            ASTNode::Loop(condition, body) => self.execute_loop(*condition, body),
            ASTNode::Spell(name, params, body) => self.define_spell(name, params, body),
            ASTNode::SpellInvocation(name, args) => self.invoke_spell(name, args),
        }
    }

    fn execute_command(&mut self, command: String, args: Vec<ASTNode>) -> String {
        match command.as_str() {
            "cast" => self.cast_spell(args),
            "summon" => self.summon_creature(args),
            "enchant" => self.enchant_item(args),
            "scry" => self.scry(args),
            _ => format!("The runes for '{}' are unfamiliar.", command),
        }
    }

    fn execute_if_else(&mut self, condition: ASTNode, if_body: Vec<ASTNode>, else_body: Option<Vec<ASTNode>>) -> String {
        let condition_result = self.evaluate_condition(condition);
        let body = if condition_result { if_body } else { else_body.unwrap_or_default() };
        body.into_iter().map(|node| self.interpret(node)).collect::<Vec<_>>().join("\n")
    }

    fn execute_loop(&mut self, condition: ASTNode, body: Vec<ASTNode>) -> String {
        let mut result = Vec::new();
        while self.evaluate_condition(condition.clone()) {
            for node in &body {
                result.push(self.interpret(node.clone()));
            }
        }
        result.join("\n")
    }

    fn define_spell(&mut self, name: String, params: Vec<String>, body: Vec<ASTNode>) -> String {
        self.spells.insert(name.clone(), (params, body));
        format!("Spell '{}' has been inscribed in the grimoire.", name)
    }

    fn invoke_spell(&mut self, name: String, args: Vec<ASTNode>) -> String {
        if let Some((params, body)) = self.spells.get(&name).cloned() {
            let mut spell_scope = HashMap::new();
            for (param, arg) in params.into_iter().zip(args) {
                spell_scope.insert(param, arg);
            }
            let old_vars = std::mem::replace(&mut self.variables, spell_scope);
            let result = body.into_iter().map(|node| self.interpret(node)).collect::<Vec<_>>().join("\n");
            self.variables = old_vars;
            result
        } else {
            format!("The spell '{}' is not found in the grimoire.", name)
        }
    }

    fn evaluate_condition(&mut self, condition: ASTNode) -> bool {
        match self.interpret(condition).as_str() {
            "true" => true,
            _ => false,
        }
    }

    fn assign_variable(&mut self, name: String, value: ASTNode) -> String {
        let value_str = self.interpret(value.clone());
        self.variables.insert(name.clone(), value);
        format!("Variable {} has been inscribed with arcane power: {}", name, value_str)
    }

    fn get_variable(&self, name: &str) -> String {
        match self.variables.get(name) {
            Some(value) => format!("{:?}", value),
            None => format!("The runes for {} have not been inscribed.", name),
        }
    }

    fn cast_spell(&mut self, args: Vec<ASTNode>) -> String {
        if args.is_empty() {
            return "Error: No spell runes provided".to_string();
        }
        match self.interpret(args[0].clone()).as_str() {
            "fireball" => "A sphere of flame erupts from your fingertips!".to_string(),
            "frostbolt" => "Crystals of ice form and launch from your palm!".to_string(),
            "heal" => "Soothing energies flow from your hands, mending wounds.".to_string(),
            spell => format!("You trace the runes for {}, but the magic fizzles.", spell),
        }
    }

    fn summon_creature(&mut self, args: Vec<ASTNode>) -> String {
        if args.is_empty() {
            return "Error: No creature specified in the summoning circle".to_string();
        }
        let creature = self.interpret(args[0].clone());
        format!("A mystical portal opens, and a {} steps forth!", creature)
    }

    fn enchant_item(&mut self, args: Vec<ASTNode>) -> String {
        if args.len() < 2 {
            return "Error: Enchantment requires both an item and a magical property".to_string();
        }
        let item = self.interpret(args[0].clone());
        let property = self.interpret(args[1].clone());
        format!("Arcane energies swirl around the {}, imbuing it with the power of {}.", item, property)
    }

    fn scry(&mut self, args: Vec<ASTNode>) -> String {
        if args.is_empty() {
            return "You gaze into your crystal ball, but see only swirling mists.".to_string();
        }
        let vision = args.into_iter().map(|arg| self.interpret(arg)).collect::<Vec<_>>().join(" ");
        format!("As you focus, visions of {} begin to form in your mind's eye.", vision)
    }
}