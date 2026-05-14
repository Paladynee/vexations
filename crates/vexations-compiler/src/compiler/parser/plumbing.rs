use crate::compiler::parser::Parser;
use crate::frontend::token::TokenKind;

impl<'src> Parser<'src> {
    #[inline]
    pub fn advance(&mut self) {
        self.previous = self.current;
        self.current = self.tokens.pop().unwrap_or(TokenKind::MetaDummy);
    }
}
