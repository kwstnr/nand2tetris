#[derive(Debug, PartialEq)]
pub enum TokenizedLine {
    AInstruction(String),
    CInstruction(String),
    Label(String),
}

pub fn tokenize(source: &Vec<&str>) -> Vec<TokenizedLine> {
    source
        .iter()
        .map(|line| strip_whitespaces(line))
        .map(|line| strip_inline_comments(&line))
        .filter(|line| !line.is_empty())
        .map(|line| tokenize_line(&line))
        .collect::<Vec<TokenizedLine>>()
}

fn tokenize_line(line: &str) -> TokenizedLine {
    if line.starts_with("@") {
        TokenizedLine::AInstruction(line.to_string())
    } else if line.starts_with("(") {
        TokenizedLine::Label(line.to_string())
    } else {
        TokenizedLine::CInstruction(line.to_string())
    }
}

fn strip_whitespaces(line: &str) -> String {
    line
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

fn strip_inline_comments(line: &str) -> String {
    let mut parts = line.split("//");
    parts.nth(0).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_whitespaces_removes_whitespaces1() {
        let input = " ";
        let result = strip_whitespaces(input);
        let expected = "";
        assert_eq!(result, expected);
    }

    #[test]
    fn strip_whitespaces_removes_whitespaces2() {
        let input = " @123";
        let result = strip_whitespaces(input);
        let expected = "@123";
        assert_eq!(result, expected);
    }

    #[test]
    fn strip_whitespaces_removes_whitespaces3() {
        let input = " D = M + 1  ";
        let result = strip_whitespaces(input);
        let expected = "D=M+1";
        assert_eq!(result, expected);
    }

    #[test]
    fn strip_inline_comment_removes_comments1() {
        let input = "D=M+1//testtest";
        let result = strip_inline_comments(input);
        let expected = "D=M+1";
        assert_eq!(result, expected);
    }

    #[test]
    fn strip_inline_comment_removes_comments2() {
        let input = "//test";
        let result = strip_inline_comments(input);
        let expected = "";
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_line_should_return_a_instruction() {
        let input = "@123";
        let result = tokenize_line(input);
        let expected = TokenizedLine::AInstruction(input.to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn tokenize_line_should_return_label() {
        let input = "(test)";
        let result = tokenize_line(input);
        let expected = TokenizedLine::Label(input.to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn tokenize_line_should_return_c_instruction() {
        let input = "D=M+1";
        let result = tokenize_line(input);
        let expected = TokenizedLine::CInstruction(input.to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn tokenize_should_tokenize_string_vector() {
        let input = vec!(
            "   //test",
            "@123  ",
            "D= A +1",
            "//something else",
            "( Label )",
            "M =1 // test"
        );
        let result = tokenize(&input);
        let expected = vec!(
            TokenizedLine::AInstruction("@123".to_string()),
            TokenizedLine::CInstruction("D=A+1".to_string()),
            TokenizedLine::Label("(Label)".to_string()),
            TokenizedLine::CInstruction("M=1".to_string()),
        );
        assert_eq!(result, expected);
    }
}
