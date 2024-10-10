pub fn tokenize(input: &str) -> Vec<String> {
    input.split_whitespace().map(String::from).collect()
}