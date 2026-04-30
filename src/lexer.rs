mod alnum_lit;
mod char_lit;
mod num_lit;
mod per_char_dispatch;
mod plumbing;
mod skip_whitespace;
mod string_lit;
mod zero_lit;

use core::fmt;
use core::fmt::Display;
use std::marker::PhantomData;

use crate::source::LineCol;
use crate::source::VexationsSource;
use crate::token::TokenKind;

#[derive(Debug, Clone)]
pub enum LexerErrorKind {
    UnclosedBlockComment,
    UnknownCharacter(u8),
}

#[derive(Debug, Clone)]
pub struct LexerError {
    pub location: LineCol,
    pub kind: LexerErrorKind,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            LexerErrorKind::UnclosedBlockComment =>
                write!(f, "unclosed block comment"),
            LexerErrorKind::UnknownCharacter(c) =>
                write!(f, "unknown character: '{c:#x}'"),
        }
    }
}

impl core::error::Error for LexerError {}

#[allow(unused)]
pub fn lex<'src>(
    src: &VexationsSource<'src>, tokens: &mut Vec<TokenKind>,
    errors: &mut Vec<LexerError>, idents: &mut Vec<&'src str>,
) {
    let mut lexer = Lexer::new(src);
    lexer.lex_all(tokens, errors, idents);
}

pub struct Lexer<'a, 'src> {
    base: *const u8,
    limit: *const u8,
    start: *const u8,
    cursor: *const u8,
    _phantom: PhantomData<&'a VexationsSource<'src>>,
}

impl<'a, 'src> Lexer<'a, 'src> {
    #[inline]
    pub const fn new(src: &'a VexationsSource<'src>) -> Self {
        Lexer {
            base: src.base_ptr(),
            limit: src.end_ptr(),
            start: src.base_ptr(),
            cursor: src.base_ptr(),
            _phantom: PhantomData,
        }
    }

    /// You may be at the end after this function returns.
    ///
    /// # @Safety
    ///
    /// `self.skip_whitespace()` must have been called prior
    /// `self.is_at_end()` must be false.
    /// `self.start == self.index`
    #[inline]
    pub unsafe fn lex_one(
        &mut self, tokens: &mut Vec<TokenKind>, errors: &mut Vec<LexerError>,
        idents: &mut Vec<&'src str>,
    ) {
        // @SAFETY: explicit check @Lexer::lex_all
        let c = unsafe { self.advance_unchecked() };

        // we may be at the end here
        // [0, 0, 0]
        //  ^

        tokens.reserve(1);
        errors.reserve(1);
        idents.reserve(1);

        // @SAFETY: reserved space for all vectors passed in above
        unsafe {
            per_char_dispatch::PER_CHAR_FN_TABLE[c as usize](
                self, c, tokens, errors, idents,
            )
        };

        // currently, we may be 2-past end here. theoretically, the above
        // dispatcher could let us end up with 3-past-end. that is still
        // safe, and luckily these functions all return back to the main
        // !self.is_at_end() loop within `Lexer::lex_all`.
    }

    #[inline]
    pub fn lex_all(
        &mut self, tokens: &mut Vec<TokenKind>, errors: &mut Vec<LexerError>,
        idents: &mut Vec<&'src str>,
    ) {
        while !self.is_at_end() {
            self.skip_whitespace(errors);

            // we may be at end here
            // [0, 0, 0]
            //     ^

            if self.is_at_end() {
                return;
            }

            // we can NOT be at end here

            self.start = self.cursor;
            unsafe {
                self.assert_ptr_valid(self.cursor);
                self.assert_ptr_valid(self.start);
                self.lex_one(tokens, errors, idents);
            }
        }
    }
}
