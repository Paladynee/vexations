use core::hint::assert_unchecked;
use std::num::NonZeroUsize;

use crate::compiler::lexer::Lexer;
use crate::compiler::lexer::error::LexerErrorKind;
use crate::frontend::source::Span;
use crate::frontend::token::TokenKind;

impl<'src> Lexer<'src> {
    #[inline(always)]
    pub const fn buffer(&self) -> &'src [u8] {
        self.src.buffer()
    }

    /// Should only be accessed for providing diagnostics, lexing goes through
    /// [`Lexer::buffer`].
    #[inline(always)]
    pub const fn source(&self) -> &'src str {
        self.src.source()
    }

    #[inline(always)]
    pub const fn source_len(&self) -> usize {
        self.src.source_len()
    }

    #[inline(always)]
    pub const fn buffer_len(&self) -> usize {
        self.src.buffer_len()
    }

    /// ```
    ///           v oob
    /// [a, b, c, 0, 0, 0]
    ///        ^ within bounds
    /// ```
    #[inline(always)]
    pub const fn is_oob(&self, index: usize) -> bool {
        // catches all of;
        // - source-end,
        // - 1 past source-end
        // - 2 past source-end
        // - 3 past source-end
        // reaching any other `n` past source-end is already illegal, but it
        // still checks for that
        // ```_
        // [a, b, c, \0, \0, \0]
        //            ^ index
        //                ^ index
        // ```
        index >= self.source_len()
    }

    /// ```
    ///           v true
    /// [a, b, c, 0, 0, 0]
    ///        ^ false
    /// ```
    #[inline(always)]
    pub const fn is_at_end(&self) -> bool {
        self.is_oob(self.index)
    }

    /// Check for [`Lexer::is_at_end`] after this function returns.
    #[inline(always)]
    pub unsafe fn incr_unchecked(&mut self) {
        unsafe {
            // before: assume within bounds
            assert_unchecked(self.index < self.buffer_len());
            self.index = self.index.unchecked_add(1);
            // after: might be self.index == self.buffer_len()
        }
    }

    #[inline(always)]
    pub unsafe fn index_unchecked(&self, index: usize) -> u8 {
        unsafe {
            assert_unchecked(index < self.buffer_len());
            *self.buffer().get_unchecked(index)
        }
    }

    #[inline(always)]
    pub unsafe fn peek_unchecked(&self) -> u8 {
        unsafe { self.index_unchecked(self.index) }
    }

    #[inline(always)]
    pub unsafe fn peek_next_unchecked(&self) -> u8 {
        unsafe { self.index_unchecked(self.index.unchecked_add(1)) }
    }

    /// Check for [`Lexer::is_at_end`] after this function returns.
    #[inline(always)]
    pub unsafe fn advance_unchecked(&mut self) -> u8 {
        unsafe {
            let c = self.peek_unchecked();
            self.incr_unchecked();
            c
        }
    }

    /// Check for [`Lexer::is_at_end`] after this function returns.
    #[inline(always)]
    pub unsafe fn matches_unchecked(&mut self, expected: u8) -> bool {
        unsafe {
            if self.peek_unchecked() == expected {
                self.incr_unchecked();
                true
            } else {
                false
            }
        }
    }

    /// Check for [`Lexer::is_at_end`] after this function returns.
    #[inline(always)]
    pub unsafe fn expect_consume_unchecked(&mut self, expected: u8) -> bool {
        unsafe {
            if self.peek_unchecked() == expected {
                self.incr_unchecked();
                true
            } else {
                self.error_here(LexerErrorKind::UnexpectedWhileExpecting(
                    expected,
                ));
                false
            }
        }
    }

    #[inline(always)]
    pub unsafe fn make_identifier_from_raw_parts(
        &self, start: usize, index: usize,
    ) -> &'src str {
        unsafe {
            assert_unchecked(start <= index);
            let s = self.buffer().get_unchecked(start..index);
            str::from_utf8_unchecked(s)
        }
    }

    #[inline(always)]
    pub fn make_identifier(&self) -> &'src str {
        unsafe { self.make_identifier_from_raw_parts(self.start, self.index) }
    }

    #[inline(always)]
    pub fn push_token(&mut self, token_kind: TokenKind) {
        self.tokens.push(token_kind);
        self.spans.push(self.start);
    }

    #[inline(always)]
    pub fn push_token_with_ident(
        &mut self, token_kind: TokenKind, ident: &'src str,
    ) {
        self.push_token(token_kind);
        self.idents.push(ident);
    }

    #[allow(clippy::needless_range_loop)]
    #[inline(never)]
    #[cold]
    pub fn location(&self) -> Span {
        let src = self.source();
        let Some(prefix) = src.get(..self.start) else {
            // shouldn't happen, self.start never hits the padding bytes at the
            // end of the source.
            // ```_
            // [a, b, c, \0, \0, \0]
            //        ^ start never goes past this character
            // ```
            return Span {
                line: unsafe { NonZeroUsize::new_unchecked(1) },
                col: 0,
                source_offset: self.start,
                span_length: 0,
            };
        };

        let prefix = prefix.as_bytes();

        let mut lc: usize = 1;
        let mut last_nl_offset: Option<usize> = None;

        for i in 0..prefix.len() {
            // rustc gets rid of this indexing panic for us thanks to loop
            // invariant being simple asf
            if prefix[i] == b'\n' {
                lc += 1;
                last_nl_offset = Some(i);
            }
        }

        let col = match last_nl_offset {
            Some(nl_pos) => self.start - (nl_pos + 1),
            None => self.start,
        };

        Span {
            line: unsafe { NonZeroUsize::new_unchecked(lc) },
            col,
            source_offset: self.start,
            span_length: unsafe { self.index.unchecked_sub(self.start) },
        }
    }
}
