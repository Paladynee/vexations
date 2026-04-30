use std::hint::assert_unchecked;
use std::hint::unlikely;

use crate::lexer::Lexer;
use crate::source::LineCol;

impl<'a, 'src> Lexer<'a, 'src> {
    #[inline]
    pub const fn src(&self) -> &'src [u8] {
        self.src.as_bytes()
    }

    #[inline]
    pub const fn is_oob(&self, index: usize) -> bool {
        index >= self.src().len()
    }

    #[inline]
    pub const fn is_index_oob(&self) -> bool {
        self.is_oob(self.index)
    }

    #[inline]
    pub const fn is_at_end(&self) -> bool {
        unlikely(self.is_index_oob())
    }

    /// # @Safety
    ///
    /// `self.is_oob(index)` be false.
    #[warn(clippy::unnecessary_operation)] // shut the fuck up? i know what i'm doing
    #[inline]
    pub unsafe fn assert_within_bounds(&self, index: usize) {
        unsafe {
            assert_unchecked(!self.is_index_oob());
            *self.src().get_unchecked(index);
        }
    }

    /// # @Safety
    ///
    /// `self.is_oob(index)` be false.
    #[inline]
    pub unsafe fn index_unchecked(&self, index: usize) -> u8 {
        unsafe {
            self.assert_within_bounds(index);
            *self.src().get_unchecked(index)
        }
    }

    #[inline]
    pub fn index(&self, index: usize) -> Option<u8> {
        self.src().get(index).copied()
    }

    /// You may be at the end after this function returns.
    ///
    /// # @Safety
    ///
    /// `self.is_at_end()` be false.
    #[inline]
    pub unsafe fn incr_index_unchecked(&mut self) {
        unsafe {
            self.index = self.index.unchecked_add(1);
        }
    }

    /// # @Safety
    ///
    /// `self.is_at_end()` be false.
    #[inline]
    pub unsafe fn peek_unchecked(&self) -> u8 {
        unsafe {
            self.assert_within_bounds(self.index);
            self.index_unchecked(self.index)
        }
    }

    #[inline]
    pub fn peek(&self) -> Option<u8> {
        if self.is_at_end() {
            None
        } else {
            Some(unsafe { self.peek_unchecked() })
        }
    }

    #[inline]
    pub fn peek_next(&self) -> Option<u8> {
        self.index(self.index + 1)
    }

    /// You may be at the end after this function returns.
    ///
    /// # @Safety
    ///
    /// `self.is_at_end()` be false.
    #[inline]
    pub unsafe fn advance_unchecked(&mut self) -> u8 {
        unsafe {
            let res = self.peek_unchecked();
            self.incr_index_unchecked();
            res
        }
    }

    /// You may be at the end after this function returns.
    #[inline]
    pub fn advance(&mut self) -> Option<u8> {
        if self.is_at_end() {
            None
        } else {
            Some(unsafe { self.advance_unchecked() })
        }
    }

    /// You may be at the end after this function returns.
    #[inline]
    pub fn matches(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }
        unsafe {
            if self.peek_unchecked() == expected {
                self.incr_index_unchecked();
                true
            } else {
                false
            }
        }
    }

    /// Lazy line/column tracking to avoid inflating lexer size.
    #[inline]
    pub fn location(&self) -> LineCol {
        todo!("Lexer::location")
    }
}
