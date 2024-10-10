pub fn tokenize(input: &str) -> Vec<String> {
    // Basic tokenization: split the input by whitespace
    input.split_whitespace().map(String::from).collect()
}