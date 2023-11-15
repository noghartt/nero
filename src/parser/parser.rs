use super::{
    stmt::{statement, Statement},
    token_stream::TokenStream,
};
use crate::lexer::tokens::{Token, TokenType};

pub struct Parser {
    pub tokens: Box<TokenStream>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Self {
            tokens: Box::new(TokenStream::new(tokens)),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, Vec<&str>> {
        let mut statements = vec![];
        let mut errors = vec![];

        while self.tokens.current().typ != TokenType::Eof {
            match statement(self.tokens.as_mut()) {
                Ok(stmt) => statements.push(stmt),
                Err(_) => {
                    self.sync();
                    errors.push("Error while parsing");
                }
            }
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
        }
    }

    fn sync(&mut self) {
        while self.tokens.current().typ != TokenType::Eof {
            self.tokens.accept();
        }
    }
}
