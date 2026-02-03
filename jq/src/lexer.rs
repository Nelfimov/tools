use std::str;

use crate::ErrorInfo;
use crate::Found;
use crate::JSONError;
use crate::JSONToken;
use crate::Span;
use crate::Token;

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
                b'{' => Token {
                    kind: JSONToken::LBrace,
                    span: Span {
                        start: self.pos,
                        end: self.pos,
                    },
                },
                b'}' => Token {
                    kind: JSONToken::RBrace,
                    span: Span {
                        start: self.pos,
                        end: self.pos,
                    },
                },
                b'[' => Token {
                    kind: JSONToken::LBracket,
                    span: Span {
                        start: self.pos,
                        end: self.pos,
                    },
                },
                b']' => Token {
                    kind: JSONToken::RBracket,
                    span: Span {
                        start: self.pos,
                        end: self.pos,
                    },
                },
                b':' => Token {
                    kind: JSONToken::Colon,
                    span: Span {
                        start: self.pos,
                        end: self.pos,
                    },
                },
                b',' => Token {
                    kind: JSONToken::Comma,
                    span: Span {
                        start: self.pos,
                        end: self.pos,
                    },
                },
                d => {
                    return Err(JSONError::UnexpectedByte(ErrorInfo {
                        offset: self.pos,
                        found: Found::Byte(str::from_utf8(&[d]).unwrap().to_string()),
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
            if v != b' ' {
                break;
            }
            self.pos += 1;
        }
    }
}
