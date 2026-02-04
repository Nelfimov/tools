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
                        end: self.pos + 1,
                    },
                },
                b'}' => Token {
                    kind: JSONToken::RBrace,
                    span: Span {
                        start: self.pos,
                        end: self.pos + 1,
                    },
                },
                b'[' => Token {
                    kind: JSONToken::LBracket,
                    span: Span {
                        start: self.pos,
                        end: self.pos + 1,
                    },
                },
                b']' => Token {
                    kind: JSONToken::RBracket,
                    span: Span {
                        start: self.pos,
                        end: self.pos + 1,
                    },
                },
                b':' => Token {
                    kind: JSONToken::Colon,
                    span: Span {
                        start: self.pos,
                        end: self.pos + 1,
                    },
                },
                b',' => Token {
                    kind: JSONToken::Comma,
                    span: Span {
                        start: self.pos,
                        end: self.pos + 1,
                    },
                },
                b't' => self
                    .lex_exact("true".to_string().as_bytes(), JSONToken::True)
                    .unwrap(),
                b'f' => self
                    .lex_exact("false".to_string().as_bytes(), JSONToken::False)
                    .unwrap(),
                b'n' => self
                    .lex_exact("null".to_string().as_bytes(), JSONToken::Null)
                    .unwrap(),
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

    fn lex_exact(&mut self, byte_seq: &[u8], expected: JSONToken) -> Result<Token, JSONError> {
        let mut span = Span {
            start: self.pos,
            end: self.pos,
        };
        let mut inner_idx = byte_seq.first().unwrap();
        let equals = byte_seq.iter().all(|v| {
            let result = *v == self.input[self.pos];
            self.pos += 1;
            inner_idx = v;
            result
        });

        if equals {
            span.end = self.pos;
            Ok(Token {
                kind: expected,
                span,
            })
        } else {
            Err(JSONError::UnexpectedByte(ErrorInfo {
                offset: span.start,
                found: Found::Byte(str::from_utf8(&[*inner_idx]).unwrap().to_string()),
            }))
        }
    }
}
