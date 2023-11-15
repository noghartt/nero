use super::tokens::{Lexeme, Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    source: String,
    current: usize,
    start: usize,
    line: usize,
    column: usize,
}

type Error = String;

impl Lexer {
    pub fn from_string(source: String) -> Self {
        Self {
            source,
            current: 0,
            start: 0,
            line: 1,
            column: 0,
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, Vec<Error>> {
        let mut tokens: Vec<Token> = vec![];
        let mut errors: Vec<String> = vec![];

        loop {
            match self.scan_token() {
                Ok(token) => {
                    tokens.push(token.clone());

                    if token.typ == TokenType::Eof {
                        break;
                    }
                }
                Err(err) => errors.push(err),
            }
        }

        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
    }

    fn scan_token(&mut self) -> Result<Token, Error> {
        self.start = self.current;
        if let Some(ch) = self.peek_char() {
            let token = match ch {
                ' ' => return self.scan_token(),
                '+' => TokenType::Plus,
                '-' => TokenType::Minus,
                '/' => TokenType::Dash,
                '*' => TokenType::Star,
                '0'..='9' => self.lex_int(ch),
                _ => return Err(format!("Weird token: {}", ch)),
            };

            Ok(self.add_token(token))
        } else {
            Ok(self.add_token(TokenType::Eof))
        }
    }

    fn add_token(&self, typ: TokenType) -> Token {
        Token {
            typ,
            lexeme: Lexeme {
                start: self.start,
                end: self.current,
            },
        }
    }

    fn lex_int(&mut self, ch: char) -> TokenType {
        let mut int = String::from(ch);

        while let Some(ch) = self.peek_char() {
            match ch {
                '0'..='9' => int.push(ch),
                '_' => int.push(ch),
                _ => break,
            }
        }

        let int = int.replace('_', "");
        match i64::from_str_radix(int.as_str(), 10) {
            Ok(i) => TokenType::Int(i),
            Err(_) => panic!("Failed to lex the Int"),
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        let c = self.get_current_char();

        if c.is_some() {
            self.current += 1;
            if c.unwrap() == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }

        c
    }

    fn get_previous_char(&self) -> Option<char> {
        self.source.chars().nth(self.current - 1)
    }

    fn get_current_char(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn get_next_char(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }
}
