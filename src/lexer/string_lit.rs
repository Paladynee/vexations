use crate::lexer::Lexer;
use crate::lexer::LexerError;
use crate::token::TokenKind;

impl<'a, 'src> Lexer<'a, 'src> {
    #[inline]
    pub unsafe fn string_lit(
        &mut self, tokens: &mut Vec<TokenKind>, errors: &mut Vec<LexerError>,
        idents: &mut Vec<&'src str>,
    ) {
        todo!()
    }
}
