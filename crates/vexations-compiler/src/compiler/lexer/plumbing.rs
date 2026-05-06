use std::hint::assert_unchecked;

use crate::compiler::lexer::Lexer;

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

    #[inline(always)]
    pub fn make_literal(&self) -> &'src str {
        unsafe {
            assert_unchecked(self.start <= self.index);
            let s = self.buffer().get_unchecked(self.start..self.index);
            str::from_utf8_unchecked(s)
        }
    }
}
