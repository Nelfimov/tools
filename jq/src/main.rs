use std::error::Error;
use std::io::BufRead;

use crate::lexer::Lexer;

pub mod lexer;

fn main() {
    let stdin = std::io::stdin();

    let line = stdin.lock().lines().next().unwrap().unwrap();
    let result = run(line.as_str());
    println!("Result: {:?}", result);
}

#[derive(Debug, PartialEq, Eq)]
pub struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub struct Token {
    pub kind: JSONToken,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub enum JSONToken {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Colon,
    String(String),
    Number(String),
    True,
    False,
    Null,
    Eof,
}

#[derive(Debug)]
pub struct ErrorInfo {
    pub offset: usize,
    pub found: Found,
}

#[derive(Debug)]
pub enum Found {
    Byte(String),
    Eof,
}

#[derive(Debug)]
pub enum JSONError {
    UnexpectedByte(ErrorInfo),
    UnterminatedString(ErrorInfo),
    InvalidEscape(ErrorInfo),
    InvalidUnicodeEscape(ErrorInfo),
    InvalidNumber(ErrorInfo),
}

impl std::fmt::Display for JSONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Error for JSONError {}

fn run(input: &str) -> Result<Vec<Token>, JSONError> {
    let mut result: Vec<Token> = Vec::new();
    let mut lexer = Lexer::new(input.as_bytes());

    loop {
        match lexer.next_token() {
            Ok(t) => match t.kind {
                JSONToken::Eof => {
                    result.push(t);
                    break;
                }
                _ => result.push(t),
            },
            Err(e) => return Err(e),
        };
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::JSONToken;
    use crate::Span;
    use crate::run;

    /// Eof
    ///
    /// один токен Eof со span start=end=0
    #[test]
    fn no_input() {
        let input = "";
        let result = run(input).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result.first().unwrap().kind, JSONToken::Eof);
        assert_eq!(result.first().unwrap().span, Span { start: 0, end: 0 });
    }

    /// WS/Tab/CR/LF
    ///
    /// Eof со span в конце строки
    #[test]
    fn only_ws() {
        let input = "     ";
        let result = run(input).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result.first().unwrap().kind, JSONToken::Eof);
        assert_eq!(result.first().unwrap().span, Span { start: 5, end: 5 });
    }

    /// `{}`
    #[test]
    fn only_braces() {
        let input = "{}";
        let result = run(input).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result.first().unwrap().kind, JSONToken::LBrace);
        assert_eq!(result.first().unwrap().span, Span { start: 0, end: 1 });
        assert_eq!(result[1].kind, JSONToken::RBrace);
        assert_eq!(result[1].span, Span { start: 1, end: 2 });
        assert_eq!(result.last().unwrap().kind, JSONToken::Eof);
        assert_eq!(result.last().unwrap().span, Span { start: 2, end: 2 });
    }

    /// ` [ ] `
    #[test]
    fn brackets_with_ws() {
        let input = " [ ] ";
        let result = run(input).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result.first().unwrap().kind, JSONToken::LBracket);
        assert_eq!(result.first().unwrap().span, Span { start: 1, end: 2 });
        assert_eq!(result[1].kind, JSONToken::RBracket);
        assert_eq!(result[1].span, Span { start: 3, end: 4 });
        assert_eq!(result.last().unwrap().kind, JSONToken::Eof);
        assert_eq!(result.last().unwrap().span, Span { start: 5, end: 5 });
    }

    /// `@`
    #[test]
    #[should_panic]
    fn unexpected_byte() {
        let input = "@";
        let _ = run(input).unwrap();
    }

    /// `true`
    #[test]
    fn true_input() {
        let input = "true";
        let result = run(input).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result.first().unwrap().kind, JSONToken::True);
        assert_eq!(result.first().unwrap().span, Span { start: 0, end: 4 });
    }

    /// `true`
    #[test]
    fn false_input() {
        let input = "false";
        let result = run(input).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result.first().unwrap().kind, JSONToken::False);
        assert_eq!(result.first().unwrap().span, Span { start: 0, end: 5 });
    }

    /// `null`
    #[test]
    fn null_input() {
        let input = "null";
        let result = run(input).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result.first().unwrap().kind, JSONToken::Null);
        assert_eq!(result.first().unwrap().span, Span { start: 0, end: 4 });
    }

    /// `tRue`
    #[test]
    #[should_panic = "UnexpectedByte"]
    fn invalid_true_input() {
        let input = "tRue";
        let _ = run(input).unwrap();
    }

    /// ` true `
    #[test]
    fn true_ws_input() {
        let input = " true ";
        let result = run(input).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result.first().unwrap().kind, JSONToken::True);
        assert_eq!(result.first().unwrap().span, Span { start: 1, end: 5 });
    }
}
