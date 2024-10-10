#[derive(Debug, Clone)]
pub enum ASTNode {
    Command(String, Vec<ASTNode>),
    Variable(String),
    Assignment(String, Box<ASTNode>),
    Number(f64),
    String(String),
    Boolean(bool),
    IfElse(Box<ASTNode>, Vec<ASTNode>, Option<Vec<ASTNode>>),
    Loop(Box<ASTNode>, Vec<ASTNode>),
    Spell(String, Vec<String>, Vec<ASTNode>),
    SpellInvocation(String, Vec<ASTNode>),
}

pub fn parse(tokens: Vec<String>) -> Result<ASTNode, String> {
    if tokens.is_empty() {
        return Err("No tokens to parse".to_string());
    }

    match tokens[0].as_str() {
        "inscribe" => parse_assignment(&tokens[1..]),
        "wyrd" => parse_if_else(&tokens[1..]),
        "circle" => parse_loop(&tokens[1..]),
        "spell" => parse_spell_definition(&tokens[1..]),
        "invoke" => parse_spell_invocation(&tokens[1..]),
        _ => parse_command(tokens),
    }
}

fn parse_assignment(tokens: &[String]) -> Result<ASTNode, String> {
    if tokens.len() < 3 || tokens[1] != "as" {
        return Err("Invalid assignment syntax".to_string());
    }
    let var_name = tokens[0].clone();
    let value = parse_expression(&tokens[2..])?;
    Ok(ASTNode::Assignment(var_name, Box::new(value)))
}

fn parse_if_else(tokens: &[String]) -> Result<ASTNode, String> {
    let mut parts = tokens.split(|&ref t| t == "otherwise");
    let if_part = parts.next().ok_or("Invalid if-else structure")?;
    let else_part = parts.next();

    if if_part.len() < 2 {
        return Err("Invalid if structure".to_string());
    }

    let condition = parse_expression(&if_part[0..1])?;
    let if_body = parse_block(&if_part[1..])?;
    let else_body = else_part.map(parse_block).transpose()?;

    Ok(ASTNode::IfElse(Box::new(condition), if_body, else_body))
}

fn parse_loop(tokens: &[String]) -> Result<ASTNode, String> {
    if tokens.len() < 2 {
        return Err("Invalid loop structure".to_string());
    }
    let condition = parse_expression(&tokens[0..1])?;
    let body = parse_block(&tokens[1..])?;
    Ok(ASTNode::Loop(Box::new(condition), body))
}

fn parse_spell_definition(tokens: &[String]) -> Result<ASTNode, String> {
    if tokens.len() < 4 || tokens[1] != "with" {
        return Err("Invalid spell definition".to_string());
    }
    let name = tokens[0].clone();
    let params = tokens[2].split(',').map(String::from).collect();
    let body = parse_block(&tokens[3..])?;
    Ok(ASTNode::Spell(name, params, body))
}

fn parse_spell_invocation(tokens: &[String]) -> Result<ASTNode, String> {
    if tokens.is_empty() {
        return Err("Invalid spell invocation".to_string());
    }
    let name = tokens[0].clone();
    let args = tokens[1..].iter().map(|t| ASTNode::Variable(t.clone())).collect();
    Ok(ASTNode::SpellInvocation(name, args))
}

fn parse_block(tokens: &[String]) -> Result<Vec<ASTNode>, String> {
    if tokens.is_empty() || tokens[0] != "begin" || tokens.last() != Some(&"end".to_string()) {
        return Err("Invalid block structure".to_string());
    }
    tokens[1..tokens.len()-1].split(|&ref t| t == ";")
        .map(|t| parse(t.to_vec()))
        .collect()
}

fn parse_command(tokens: Vec<String>) -> Result<ASTNode, String> {
    let command = tokens[0].clone();
    let args = tokens[1..].iter().map(|t| ASTNode::Variable(t.clone())).collect();
    Ok(ASTNode::Command(command, args))
}

fn parse_expression(tokens: &[String]) -> Result<ASTNode, String> {
    if tokens.is_empty() {
        return Err("Empty expression".to_string());
    }

    match tokens[0].as_str() {
        "true" => Ok(ASTNode::Boolean(true)),
        "false" => Ok(ASTNode::Boolean(false)),
        _ if tokens[0].starts_with('"') && tokens[0].ends_with('"') => {
            Ok(ASTNode::String(tokens[0][1..tokens[0].len()-1].to_string()))
        }
        _ if tokens[0].parse::<f64>().is_ok() => {
            Ok(ASTNode::Number(tokens[0].parse::<f64>().unwrap()))
        }
        _ => Ok(ASTNode::Variable(tokens[0].clone())),
    }
}