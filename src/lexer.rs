use std::fs;
use crate::token::Token;

pub struct Lexer {
    line: usize,
    col: usize,
    tokens: Vec<Token>,
    file: String,
    file_name: String,
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
        let mut string = String::new();

        // Check if the string contains any numeric characters
        let contains_numeric_chars = self.file[self.col..].chars().any(|c| c.is_digit(10) || c == '.');

        // If there are no numeric characters, return early
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
                    // Check if it's a hexadecimal number
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
                    // Check if it's scientific notation
                    string.push(c);
                    self.advance();

                    // Check for optional sign
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

        // Parse the string to a numeric value and check it
        if let Ok(_num) = string.parse::<f64>() {
            if let Ok(_u) = string.parse::<u128>() { 
                self.tokens.push(Token::IntLit(self.line, self.col - string.len(), string));
            } else if let Ok(_i) = string.parse::<i128>() {
                self.tokens.push(Token::IntLit(self.line, self.col - string.len(), string));
            } else {
                self.tokens.push(Token::FlLit(self.line, self.col - string.len(), string));
            }
        } else {
            eprintln!("Invalid number format at {}:{}", self.line, self.col - string.len());
        }
    }

    pub fn lex(mut self) -> Vec<Token> {
        while self.col < self.file.len() {
            match self.current() {
                // Handle all the identifiers/keywords and data types together
                // Implement the logic in keyword_or_identifier()
                Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                    self.keyword_or_datatype_or_identifier();
                },

                // Handle numbers
                Some(c) if c.is_digit(10) => {
                    self.number();
                },

                // Handle operators and separators
                Some(';') | Some(',') | Some('{') | Some('}') | Some('[') | Some(']') | Some('(') | Some(')') => {
                    self.tokens.push(Token::Seperator(self.line, self.col, self.current().unwrap()));
                    self.advance();
                },

                Some('>') | Some('<') | Some('=') | Some('!') | Some('^') | Some('|') | Some('&') => {
                    if self.current() == Some('>') && self.peek() == Some('>') {
                        self.tokens.push(Token::Operator(self.line, self.col, self.file[self.col..=self.col + 1].to_string()));
                        self.advance(); // Move to the next character
                    } else if self.current() == Some('<') && self.peek() == Some('<') {
                        self.tokens.push(Token::Operator(self.line, self.col, self.file[self.col..=self.col + 1].to_string()));
                        self.advance(); // Move to the next character
                    } else if self.peek() == Some('=') {
                        self.tokens.push(Token::Operator(self.line, self.col, self.file[self.col..=self.col + 1].to_string()));
                        self.advance(); // Move to the next character
                    } else {
                        self.tokens.push(Token::Operator(self.line, self.col, self.current().unwrap().to_string()));
                    }
                    self.advance();
                },

                // Ignore whitespace 
                Some(' ') | Some('\t') | Some('\r') => {
                    self.advance();
                }

                // Newline, increment line counter.
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                }

                _ => {
                    eprintln!("Unexpected character '{}' found at {}:{} of {}.",
                        self.current().unwrap(),
                        self.line,
                        self.col,
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