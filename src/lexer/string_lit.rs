use crate::lexer::Lexer;
use crate::lexer::LexerError;
use crate::token::TokenKind;

impl<'a, 'src> Lexer<'a, 'src> {
    #[inline]
    pub unsafe fn string_lit(
        &mut self, tokens: &mut Vec<TokenKind>, errors: &mut Vec<LexerError>,
        idents: &mut Vec<&'src str>,
    ) {
        // when we're called, we might be at the end
        // [0, 0, 0]
        //  ^
        todo!()
    }
}
