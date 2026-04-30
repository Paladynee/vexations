mod alnum_lit;
mod char_lit;
mod num_lit;
mod per_char_dispatch;
mod plumbing;
mod skip_whitespace;
mod string_lit;
mod zero_lit;

use crate::source::LineCol;
use crate::source::VexationsSource;
use crate::token::TokenKind;

pub enum LexerErrorKind {
    UnclosedBlockComment,
}

pub struct LexerError {
    location: LineCol,
    kind: LexerErrorKind,
}

#[allow(unused)]
pub fn lex<'src>(
    src: &VexationsSource<'src>, tokens: &mut Vec<TokenKind>,
    errors: &mut Vec<LexerError>, idents: &mut Vec<&'src str>,
) {
    let mut lexer = Lexer::new(src);
    // lexer.lex_all(tokens, errors, idents);
}

pub struct Lexer<'a, 'src> {
    src: &'a VexationsSource<'src>,
    start: usize,
    index: usize,
}

impl<'a, 'src> Lexer<'a, 'src> {
    #[inline]
    pub const fn new(src: &'a VexationsSource<'src>) -> Self {
        Lexer {
            src,
            start: 0,
            index: 0,
        }
    }

    /// You may be at the end after this function returns.
    ///
    /// # @Safety
    ///
    /// `self.skip_whitespace()` must have been called prior
    /// `self.is_at_end()` be false.
    /// `self.start == self.index`
    #[inline]
    #[rustfmt::skip]
    pub unsafe fn lex_one(
        &mut self, tokens: &mut Vec<TokenKind>, errors: &mut Vec<LexerError>,
        idents: &mut Vec<&'src str>,
    ) {
        // @SAFETY: explicit check @Lexer::lex_all
        let c = unsafe { self.advance_unchecked() };

        // we may be at the end here

        per_char_dispatch::PER_CHAR_FN_TABLE[c as usize](
            self, c, tokens, errors, idents
        );
    }

    #[inline]
    pub fn lex_all(
        &mut self, tokens: &mut Vec<TokenKind>, errors: &mut Vec<LexerError>,
        idents: &mut Vec<&'src str>,
    ) {
        while !self.is_at_end() {
            self.skip_whitespace(errors);

            // we may be at end here

            if self.is_at_end() {
                return;
            }

            // we can NOT be at end here

            self.start = self.index;
            unsafe {
                self.assert_within_bounds(self.index);
                self.assert_within_bounds(self.start);
                self.lex_one(tokens, errors, idents);
            }
        }
    }
}
