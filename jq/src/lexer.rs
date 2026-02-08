use std::str;

use crate::ErrorInfo;
use crate::Found;
use crate::JSONError;
use crate::JSONToken;
use crate::Span;
use crate::Token;

static TRUE_BYTES: &[u8] = "true".as_bytes();
static FALSE_BYTES: &[u8] = "false".as_bytes();
static NULL_BYTES: &[u8] = "null".as_bytes();

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
                b't' => self.lex_exact(TRUE_BYTES, JSONToken::True).unwrap(),
                b'f' => self.lex_exact(FALSE_BYTES, JSONToken::False).unwrap(),
                b'n' => self.lex_exact(NULL_BYTES, JSONToken::Null).unwrap(),
                b'-' | b'0'..=b'9' => self.lex_number().unwrap(),
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

    fn lex_number(&mut self) -> Result<Token, JSONError> {
        let start = self.pos;
        let end;
        let mut is_float = false;

        let mut result = vec![self.input[self.pos]];

        match result.first().unwrap() {
            &b'0' => {
                self.pos += 1;
                if self.input.len() > self.pos {
                    if self.input[self.pos] != b'.' {
                        return Err(JSONError::InvalidNumber(ErrorInfo {
                            offset: start,
                            found: Found::Byte("0".to_string()),
                        }));
                    } else {
                        is_float = true;
                        result.push(self.input[self.pos]);
                    }
                }
            }
            &b'-' | b'1'..=b'9' => (),
            _ => {
                return Err(JSONError::UnexpectedByte(ErrorInfo {
                    offset: self.pos,
                    found: Found::Byte(result.first().unwrap().to_string()),
                }));
            }
        }

        loop {
            self.pos += 1;
            if self.input.len() <= self.pos {
                end = self.pos;
                break;
            }

            let current = self.input[self.pos];

            match current {
                b',' => {
                    end = self.pos;
                    break;
                }
                b'0'..=b'9' | b'e' | b'E' | b'-' | b'+' => result.push(current),
                b'.' => {
                    if is_float {
                        return Err(JSONError::UnexpectedByte(ErrorInfo {
                            offset: self.pos,
                            found: Found::Byte(current.to_string()),
                        }));
                    } else {
                        result.push(current);
                    }
                }
                _ => {
                    return Err(JSONError::UnexpectedByte(ErrorInfo {
                        offset: self.pos,
                        found: Found::Byte(current.to_string()),
                    }));
                }
            }
        }

        Ok(Token {
            kind: JSONToken::Number(
                result
                    .iter()
                    .map(|f| str::from_utf8(&[*f]).unwrap().to_string())
                    .collect(),
            ),
            span: Span { start, end },
        })
    }
}
