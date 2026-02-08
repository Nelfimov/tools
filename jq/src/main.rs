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

#[derive(Debug, PartialEq, Eq)]
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
mod main {
    mod primitives {
        use crate::*;

        /// Eof
        ///
        /// один токен Eof со span start=end=0
        #[test]
        fn no_input() {
            let input = "";
            let result = run(input).unwrap();
            assert_eq!(result.len(), 1);
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::Eof,
                    span: Span { start: 0, end: 0 }
                }
            );
        }

        /// WS/Tab/CR/LF
        ///
        /// Eof со span в конце строки
        #[test]
        fn only_ws() {
            let input = "     ";
            let result = run(input).unwrap();
            assert_eq!(result.len(), 1);
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::Eof,
                    span: Span { start: 5, end: 5 }
                }
            );
        }

        /// `{}`
        #[test]
        fn only_braces() {
            let input = "{}";
            let result = run(input).unwrap();
            assert_eq!(result.len(), 3);
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::LBrace,
                    span: Span { start: 0, end: 1 }
                }
            );
            assert_eq!(
                result.get(1).unwrap(),
                &Token {
                    kind: JSONToken::RBrace,
                    span: Span { start: 1, end: 2 }
                }
            );
        }

        /// ` [ ] `
        #[test]
        fn brackets_with_ws() {
            let input = " [ ] ";
            let result = run(input).unwrap();
            assert_eq!(result.len(), 3);
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::LBracket,
                    span: Span { start: 1, end: 2 }
                }
            );
            assert_eq!(
                result.get(1).unwrap(),
                &Token {
                    kind: JSONToken::RBracket,
                    span: Span { start: 3, end: 4 }
                }
            );
        }

        /// `@`
        #[test]
        #[should_panic]
        fn unexpected_byte() {
            let input = "@";
            let _ = run(input).unwrap();
        }
    }

    mod true_false_null {
        use crate::*;

        /// `true`
        #[test]
        fn true_input() {
            let input = "true";
            let result = run(input).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::True,
                    span: Span { start: 0, end: 4 }
                }
            );
        }

        /// `false`
        #[test]
        fn false_input() {
            let input = "false";
            let result = run(input).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::False,
                    span: Span { start: 0, end: 5 }
                }
            );
        }

        /// `null`
        #[test]
        fn null_input() {
            let input = "null";
            let result = run(input).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::Null,
                    span: Span { start: 0, end: 4 }
                }
            );
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
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::True,
                    span: Span { start: 1, end: 5 }
                }
            );
        }
    }

    mod numbers {
        use crate::*;

        /// `0`
        #[test]
        fn number_zero_input() {
            let input = "0";
            let result = run(input).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::Number(input.to_string()),
                    span: Span { start: 0, end: 2 }
                }
            );
        }

        /// `-0`
        #[test]
        fn number_minus_zero_input() {
            let input = "-0";
            let result = run(input).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::Number(input.to_string()),
                    span: Span { start: 0, end: 2 }
                }
            );
        }

        /// `10`
        #[test]
        fn number_ten_input() {
            let input = "10";
            let result = run(input).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::Number(input.to_string()),
                    span: Span { start: 0, end: 2 }
                }
            );
        }

        /// `10.5`
        #[test]
        fn number_and_half_ten_input() {
            let input = "10.5";
            let result = run(input).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::Number(input.to_string()),
                    span: Span { start: 0, end: 4 }
                }
            );
        }

        /// `0.5`
        #[test]
        fn number_half_input() {
            let input = "0.5";
            let result = run(input).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::Number(input.to_string()),
                    span: Span { start: 0, end: 3 }
                }
            );
        }

        /// `1e10`
        #[test]
        fn number_one_expo_input() {
            let input = "1e10";
            let result = run(input).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::Number(input.to_string()),
                    span: Span { start: 0, end: 4 }
                }
            );
        }

        /// `1E-10`
        #[test]
        fn number_one_minus_expo_input() {
            let input = "1E-10";
            let result = run(input).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::Number(input.to_string()),
                    span: Span { start: 0, end: 5 }
                }
            );
        }

        /// `1e+10`
        #[test]
        fn number_one_plus_expo_input() {
            let input = "1e+10";
            let result = run(input).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::Number(input.to_string()),
                    span: Span { start: 0, end: 5 }
                }
            );
        }

        /// `-12.34e56`
        #[test]
        fn number_twelve_input() {
            let input = "-12.34e56";
            let result = run(input).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(
                result.first().unwrap(),
                &Token {
                    kind: JSONToken::Number(input.to_string()),
                    span: Span { start: 0, end: 9 }
                }
            );
        }
    }
}
