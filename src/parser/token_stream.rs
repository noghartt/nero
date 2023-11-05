use crate::lexer::tokens::{Token, TokenType};

#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<Token>,
    curr: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> TokenStream {
        TokenStream { tokens, curr: 0 }
    }

    pub fn prev(&self) -> &Token {
        self.nth(self.curr - 1)
    }

    pub fn discard(&mut self) {
        self.curr -= 1;
    }

    pub fn current(&self) -> &Token {
        self.nth(self.curr)
    }

    pub fn accept(&mut self) -> &Token {
        self.skip();
        self.prev()
    }

    pub fn match_next(&mut self, tokens: &'static [TokenType]) -> bool {
        if tokens.contains(&self.current().typ) {
            self.accept();
            return true;
        }

        false
    }

    fn skip(&mut self) {
        self.curr += 1;
    }

    fn nth(&self, n: usize) -> &Token {
        let mut iter = self.tokens.iter();

        match iter.nth(n) {
            Some(t) => t,
            None => return self.prev(),
        }
    }
}
