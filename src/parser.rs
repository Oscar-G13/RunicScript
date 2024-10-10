#[derive(Debug)]
pub enum ASTNode {
    Command(String, Vec<String>),
}

pub fn parse(tokens: Vec<String>) -> Result<ASTNode, String> {
    if tokens.is_empty() {
        return Err("No tokens to parse".to_string());
    }

    let command = tokens[0].clone();
    let arguments = tokens[1..].to_vec();

    Ok(ASTNode::Command(command, arguments))
}