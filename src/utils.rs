pub fn de_indent(input: &str) -> String {
    let mut result = String::new();
    for line in input.lines() {
        let trimmed_line = line.trim_start();
        result.push_str(trimmed_line);
        if !trimmed_line.is_empty() {
            result.push('\n');
        }
    }
    if result.ends_with('\n') {
        result.pop();
    }
    result
}
