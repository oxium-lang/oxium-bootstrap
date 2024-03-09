use std::fs;
use crate::{print::PrintGlobalState, token::Token};

pub struct Lexer {
    line: usize,
    col: usize,
    tokens: Vec<Token>,
    file: String,
    file_name: String,
    print: PrintGlobalState
}

impl Lexer {
    pub fn new(file_name: &String) -> Self {
        let file_content = fs::read_to_string(file_name)
            .expect(&format!("File '{}' not found !!!", file_name));
        
        Lexer {
            line: 1,
            col: 0,
            tokens: Vec::new(),
            file: file_content,
            file_name: file_name.to_string(),
            print: PrintGlobalState::new()
        }
    }

    fn current(&self) -> Option<char> {
        self.file.chars().nth(self.col)
    }

    fn advance(&mut self) {
        self.col += 1;
    }

    fn peek(&self) -> Option<char> {
        self.file.chars().nth(self.col + 1)
    }

    fn keyword_or_datatype_or_identifier(&mut self) {
        let mut string = String::new();

        while let Some(current_char) = self.current() {
            if current_char.is_alphanumeric() || current_char == '_' {
                string.push(current_char);
                self.advance();
            } else {
                break;
            }
        }

        const DATA_TYPES: [&str; 15] = [
            "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64",
            "f32", "f64", "u128", "i128", "f128", "char", "bool",
        ];

        const KEYWORDS: [&str; 18] =  [ 
            "if", "elif", "else", "while", "for", "as", "ret", "true", "false",
            "struct", "sync", "enum", "void", "volatile", "import", "break", "continue",
            "match"
        ];

        if DATA_TYPES.contains(&string.as_str()) {
            self.tokens.push(Token::DataType(self.line, self.col - string.len(), string));
        } else if KEYWORDS.contains(&string.as_str()) {
            self.tokens.push(Token::Keyword(self.line, self.col - string.len(), string));
        } else {
            self.tokens.push(Token::Identifier(self.line, self.col - string.len(), string));
        }
    }

    fn number(&mut self) {
        let mut string : String = String::new();

        let contains_numeric_chars : bool = self.file[self.col..].chars().any(|c| c.is_digit(10) || c == '.');

        if !contains_numeric_chars {
            return;
        }

        while let Some(c) = self.current() {
            match c {
                '0'..='9' | '.' => {
                    string.push(c);
                    self.advance();
                },

                'x' => {
                    if string == "0" {
                        string.push(c);
                        self.advance();
                    } else {
                        break;
                    }

                    if self.col >= self.file.len() {
                        break;
                    }
                },

                'e' | 'E' => {
                    string.push(c);
                    self.advance();

                    if let Some(next_c) = self.current() {
                        if next_c == '+' || next_c == '-' {
                            string.push(next_c);
                            self.advance();
                        }
                    }
                },

                _ => break,
            }
        }

        if let Ok(_) = string.parse::<f64>() {
            if let Ok(_) = string.parse::<u128>() { 
                self.tokens.push(Token::IntLit(self.line, self.col - string.len(), string));
            } else if let Ok(_) = string.parse::<i128>() {
                self.tokens.push(Token::IntLit(self.line, self.col - string.len(), string));
            } else {
                self.tokens.push(Token::FlLit(self.line, self.col - string.len(), string));
            }
        } else {
            eprintln!("Invalid number format at {}:{}", self.line, self.col  - string.len());
        }
    }

    pub fn lex(mut self) -> Vec<Token> {
        while self.col < self.file.len() {
            match self.current() {
                Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                    self.keyword_or_datatype_or_identifier();
                },

                Some(c) if c.is_digit(10) => {
                    self.number();
                },

                Some(';') | Some(',') | Some('{') | Some('}') | Some('[') | Some(']') | Some('(') | Some(')') => {
                    self.tokens.push(Token::Seperator(
                        self.line, 
                        self.col, 
                        self.current().unwrap()
                    ));
                    self.advance();
                },

                Some('>') | Some('<') | Some('=') | Some('!') | Some('^') | Some('|') | Some('&')
                | Some('+') | Some('-') | Some('*') | Some('/') | Some('%') => {
                    if self.current() == Some('>') && self.peek() == Some('>') {
                        self.tokens.push(Token::Operator(
                            self.line,
                            self.col,
                            self.file[self.col..=self.col + 1].to_string(),
                        ));
                        self.advance();
                        self.advance();
                    } else if self.current() == Some('<') && self.peek() == Some('<') {
                        self.tokens.push(Token::Operator(
                            self.line,
                            self.col,
                            self.file[self.col..=self.col + 1].to_string(),
                        ));
                        self.advance();
                        self.advance();
                    } else if self.peek() == Some('=') {
                        self.tokens.push(Token::Operator(
                            self.line,
                            self.col,
                            self.file[self.col..=self.col + 1].to_string(),
                        ));
                        self.advance();
                        self.advance();
                    } else {
                        self.tokens.push(Token::Operator(
                            self.line,
                            self.col + 1,
                            self.current().unwrap().to_string(),
                        ));
                        self.advance();
                    }
                },

                Some(' ') | Some('\t') | Some('\r') => {
                    self.advance();
                },

                Some('"') => {
                    self.advance();
                    let mut string = String::new();
                
                    while let Some(c) = self.current() {
                        if c == '\\' {
                            self.advance();
                
                            if let Some(escaped_char) = self.current() {
                                match escaped_char {
                                    '\\' | '\'' | '"' | 'n' | 'r' | 't' => {
                                        string.push(escaped_char);
                                        self.advance();
                                    },
                                    'u' => {
                                        self.advance();
                
                                        let mut unicode_sequence = String::new();
                                        let mut valid_unicode = true;
                                        for _ in 0..4 {
                                            if let Some(unicode_char) = self.current() {
                                                if unicode_char.is_ascii_hexdigit() {
                                                    unicode_sequence.push(unicode_char);
                                                    self.advance();
                                                } else {
                                                    valid_unicode = false;
                                                    break;
                                                }
                                            } else {
                                                valid_unicode = false;
                                                break;
                                            }
                                        }
                
                                        if valid_unicode {
                                            if let Ok(unicode_char) = u32::from_str_radix(&unicode_sequence, 16) {
                                                if let Some(unicode) = std::char::from_u32(unicode_char) {
                                                    string.push(unicode);
                                                } else {
                                                    self.print.error("Invalid unicode sequence.", self.line, self.col - unicode_sequence.len(), &self.file);
                                                }
                                            } else {
                                               self.print.error("Invalid unicode sequence.", self.line, self.col - unicode_sequence.len(), &self.file);
                                            }
                                        } else {
                                           self.print.error("Invalid unicode sequence.", self.line, self.col - unicode_sequence.len(), &self.file);
                                        }
                                    },
                                    _ => {
                                       self.print.error("Invalid unicode sequence.", self.line, self.col - 1, &self.file);
                                    }
                                }
                            } else {
                               self.print.error("Invalid escape sequence.", self.line, self.col - 1, &self.file);
                            }
                        } else if c == '"' {
                            self.advance();
                            break;
                        } else {
                            string.push(c);
                            self.advance();
                        }
                    }
                
                    self.tokens.push(Token::StringLit(self.line, self.col - string.len(), string));
                },                

                Some('\'') => {
                    self.advance();
                
                    if let Some(c) = self.current() {
                        if c == '\\' {
                            self.advance();
                
                            if let Some(escaped_char) = self.current() {
                                match escaped_char {
                                    '\\' | '\'' | '"' | 'n' | 'r' | 't' => {
                                        self.advance();
                                        self.tokens.push(Token::CharLit(self.line, self.col - 3, escaped_char));
                                    },
                                    'u' => {
                                        self.advance();
                                        let mut unicode_hex = String::new();
                
                                        while let Some(hex_char) = self.current() {
                                            if hex_char.is_digit(16) {
                                                unicode_hex.push(hex_char);
                                                self.advance();
                                            } else {
                                                break;
                                            }
                                        }
                
                                        if let Ok(unicode_value) = u32::from_str_radix(&unicode_hex, 16) {
                                            if let Some(unicode_char) = std::char::from_u32(unicode_value) {
                                                self.tokens.push(Token::CharLit(self.line, self.col - 6, unicode_char));
                                            } else {
                                               self.print.error("Invalid Unicode code point.", self.line, self.col - unicode_hex.len() - 3, &self.file);
                                            }
                                        } else {
                                           self.print.error("Invalid Unicode escape sequence.", self.line, self.col - unicode_hex.len() - 3, &self.file);
                                        }
                                    },
                                    _ => {
                                       self.print.error("Invalid escape sequence.", self.line, self.col - 1, &self.file);
                                        self.advance();
                                    }
                                }
                            } else {
                                eprintln!("Incomplete escape sequence at {}:{}", self.line, self.col - 1);
                            }
                        } else {
                            self.advance();
                            self.tokens.push(Token::CharLit(self.line, self.col - 2, c));
                        }
                    } else {
                        eprintln!("Incomplete character literal at {}:{}", self.line, self.col - 1);
                    }
                },                

                Some('\n') => {
                    self.line += 1;
                    self.advance();
                },

                _ => {
                    eprintln!("Unexpected character '{}' found at {}:{} of {}.",
                        self.current().unwrap(),
                        self.line,
                        self.col + 1,
                        self.file_name
                    );
                    self.tokens.push(Token::Seperator(self.line, self.col, self.current().unwrap()));
                    self.advance();
                }
            }
        }

        self.tokens.push(Token::Eof);

        self.tokens
    }
}