use std::io::BufRead;

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
    Byte(u8),
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

pub struct Lexer<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self { pos: 0, input }
    }

    pub fn next_token(&mut self) -> Result<Token, JSONError> {
        if self.input.is_empty() {
            Ok(Token {
                kind: JSONToken::Eof,
                span: Span { start: 0, end: 0 },
            })
        } else {
            self.skip_ws();
            if self.pos >= self.input.len() {
                return Ok(Token {
                    kind: JSONToken::Eof,
                    span: Span {
                        start: self.pos,
                        end: self.pos,
                    },
                });
            }
            let token = match self.input[self.pos] {
                123 => Token {
                    kind: JSONToken::LBrace,
                    span: Span {
                        start: self.pos,
                        end: self.pos,
                    },
                },
                125 => Token {
                    kind: JSONToken::RBrace,
                    span: Span {
                        start: self.pos,
                        end: self.pos,
                    },
                },
                91 => Token {
                    kind: JSONToken::LBracket,
                    span: Span {
                        start: self.pos,
                        end: self.pos,
                    },
                },
                93 => Token {
                    kind: JSONToken::RBracket,
                    span: Span {
                        start: self.pos,
                        end: self.pos,
                    },
                },
                d => {
                    println!("Token: {d}");
                    return Err(JSONError::UnexpectedByte(ErrorInfo {
                        offset: self.pos,
                        found: Found::Byte(d),
                    }));
                }
            };
            self.pos += 1;
            Ok(token)
        }
    }

    fn skip_ws(&mut self) {
        while self.pos < self.input.len() {
            let v = self.input[self.pos];
            if v != 32 {
                break;
            }
            self.pos += 1;
        }
    }
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
        assert_eq!(result.first().unwrap().span, Span { start: 0, end: 0 });
        assert_eq!(result[1].kind, JSONToken::RBrace);
        assert_eq!(result[1].span, Span { start: 1, end: 1 });
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
        assert_eq!(result.first().unwrap().span, Span { start: 1, end: 1 });
        assert_eq!(result[1].kind, JSONToken::RBracket);
        assert_eq!(result[1].span, Span { start: 3, end: 3 });
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
}
