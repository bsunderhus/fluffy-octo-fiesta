pub trait DeIndent {
    fn de_indent(&self) -> Self;
}

impl DeIndent for String {
    fn de_indent(&self) -> Self {
        let indentation = get_indentation(self);

        let mut input = self.lines().peekable();

        let mut de_indented_input = String::new();

        while let Some(line) = input.next() {
            if line.trim_start().is_empty()
                && (de_indented_input.is_empty() || input.peek().is_none())
            {
                continue;
            }
            if !de_indented_input.is_empty() {
                de_indented_input.push('\n');
            }
            de_indented_input.push_str(&trim_start_until(line, indentation))
        }
        de_indented_input
    }
}

/// Get the indentation number of a string.
/// This implementation ignores tabs.
pub fn get_indentation(input: &str) -> usize {
    if input.is_empty() {
        return 0;
    }
    let indentation = input.lines().fold(None, |smallest_indentation, line| {
        if let Some(indentation) = get_line_indentation(line) {
            let Some(smallest_indentation) = smallest_indentation else {
                return Some(indentation);
            };
            if indentation < smallest_indentation {
                return Some(indentation);
            } else {
                return Some(smallest_indentation);
            }
        }
        smallest_indentation
    });
    indentation.unwrap_or(0)
}

#[cfg(test)]
mod get_indentation_tests {
    use super::*;

    #[test]
    fn test_get_indentation_empty_input() {
        let input = "";
        let expected_output = 0;
        assert_eq!(get_indentation(input), expected_output);
    }

    #[test]
    fn test_get_indentation_single_line_no_indentation() {
        let input = "hello world";
        let expected_output = 0;
        assert_eq!(get_indentation(input), expected_output);
    }

    #[test]
    fn test_get_indentation_single_line_with_indentation() {
        let input = "    hello world";
        let expected_output = 4;
        assert_eq!(get_indentation(input), expected_output);
    }

    #[test]
    fn test_get_indentation_multiple_lines_no_indentation() {
        let input = "hello world\nhow are you?";
        let expected_output = 0;
        assert_eq!(get_indentation(input), expected_output);
    }

    #[test]
    fn test_get_indentation_multiple_lines_with_indentation() {
        let input = "     hello world\n    how are you?";
        let expected_output = 4;
        assert_eq!(get_indentation(input), expected_output);
    }

    #[test]
    fn test_get_indentation_mixed_indentation() {
        let input = "    hello world\nhow are you?";
        let expected_output = 0;
        assert_eq!(get_indentation(input), expected_output);
    }

    #[test]
    fn test_get_indentation_only_indentation() {
        let input = "    \n    \n    ";
        let expected_output = 0;
        assert_eq!(get_indentation(input), expected_output);
    }
}

/// Get the indentation number of a line.
/// a line is supposed to be a string without a newline character, or with a trailing newline character.
/// This implementation ignores tabs.
pub fn get_line_indentation(line: &str) -> Option<usize> {
    if line.is_empty() {
        return None;
    }
    let mut indentation = 0;
    for char in line.chars() {
        if char == ' ' {
            indentation += 1;
        } else {
            break;
        }
    }
    if indentation == line.len() {
        None
    } else {
        Some(indentation)
    }
}

#[cfg(test)]
mod get_line_indentation_tests {
    use super::*;

    #[test]
    fn test_get_line_indentation_empty_line() {
        let line = "";
        assert_eq!(get_line_indentation(line), None);
    }

    #[test]
    fn test_get_line_indentation_only_spaces() {
        let line = "    ";
        assert_eq!(get_line_indentation(line), None);
    }

    #[test]
    fn test_get_line_indentation_no_indentation() {
        let line = "hello world";
        let expected_output = 0;
        assert_eq!(get_line_indentation(line), Some(expected_output));
    }

    #[test]
    fn test_get_line_indentation_with_text() {
        let line = "    hello world";
        let expected_output = 4;
        assert_eq!(get_line_indentation(line), Some(expected_output));
    }

    #[test]
    fn test_get_line_indentation_with_trailing_newline() {
        let line = "    hello world\n";
        let expected_output = 4;
        assert_eq!(get_line_indentation(line), Some(expected_output));
    }
}

/// Trim the start of a string until a certain amount of characters have been trimmed.
/// It stops trimming when it encounters a non-whitespace character.
pub fn trim_start_until(input: &str, amount: usize) -> String {
    let mut result = String::new();
    let mut trimmed = 0;

    if input == "\n" {
        return input.to_string();
    }

    for (index, char) in input.chars().enumerate() {
        if !char.is_whitespace() {
            let (_, rest) = input.split_at(index);
            result.push_str(rest);
            return result;
        }

        if trimmed < amount {
            trimmed += 1;
        } else {
            result.push(char);
        }
    }
    result
}

#[cfg(test)]
mod trim_start_until_tests {
    use super::*;

    #[test]
    fn test_trim_start_until_empty_input() {
        let input = "";
        let amount = 5;
        let expected_output = "";
        assert_eq!(trim_start_until(input, amount), expected_output);
    }

    #[test]
    fn test_trim_start_until_no_trim_needed() {
        let input = "hello world";
        let amount = 5;
        let expected_output = "hello world";
        assert_eq!(trim_start_until(input, amount), expected_output);
    }

    #[test]
    fn test_trim_start_until_trimmed_all() {
        let input = "     ";
        let amount = 5;
        let expected_output = "";
        assert_eq!(trim_start_until(input, amount), expected_output);
    }

    #[test]
    fn test_trim_start_until_trimmed_some() {
        let input = "     hello world";
        let amount = 5;
        let expected_output = "hello world";
        assert_eq!(trim_start_until(input, amount), expected_output);
    }

    #[test]
    fn test_trim_start_until_trimmed_until_non_whitespace() {
        let input = "     hello     world";
        let amount = 5;
        let expected_output = "hello     world";
        assert_eq!(trim_start_until(input, amount), expected_output);
    }

    #[test]
    fn test_trim_start_until_newline_input() {
        let input = "\n";
        let amount = 5;
        let expected_output = "\n";
        assert_eq!(trim_start_until(input, amount), expected_output);
    }
}

#[cfg(test)]
mod lines_tests {

    #[test]
    fn test_lines() {
        let input = "\n1\n2\n\n3\n\n";
        let lines = input.lines().collect::<Vec<&str>>();
        println!("DEBUG:::: lines: {:?}", &lines);
        assert_eq!(lines.len(), 6);
    }
}
