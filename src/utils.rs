pub fn de_indent(input: &str) -> String {
    println!("DEBUG:::: input: {}", &input);
    let indentation = get_indentation(input);
    // let lines = input_to_lines(&input);

    input
        .split_inclusive("\n")
        .into_iter()
        .fold(String::new(), |mut acc, line| {
            let trimmed_line = trim_start(line, indentation);
            acc.push_str(&trimmed_line);
            acc
        })
        .to_string()
}

pub fn get_indentation(input: &str) -> usize {
    input
        .lines()
        .fold(usize::MAX, |smallest_indentation, line| {
            let indentation = get_line_indentation(line);
            if indentation < smallest_indentation {
                indentation
            } else {
                smallest_indentation
            }
        })
        .clamp(0, usize::MAX)
}

pub fn get_line_indentation(line: &str) -> usize {
    let mut indentation = 0;
    for char in line.chars() {
        if char == ' ' {
            indentation += 1;
        } else {
            break;
        }
    }
    if line.len() == indentation {
        0
    } else {
        indentation
    }
}

pub fn trim_start(input: &str, amount: usize) -> String {
    let mut result = String::new();
    let mut trimmed = 0;
    let mut first_non_whitespace_reached = false;

    if input == "\n" {
        return input.to_string();
    }

    for char in input.chars() {
        if !char.is_whitespace() {
            first_non_whitespace_reached = true;
        }

        if trimmed < amount && !first_non_whitespace_reached {
            trimmed += 1;
        } else {
            result.push(char);
        }
    }
    result
}
