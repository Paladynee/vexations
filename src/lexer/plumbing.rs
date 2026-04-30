use core::slice;
use std::hint::assert_unchecked;
use std::hint::unlikely;

use crate::lexer::Lexer;
use crate::source::LineCol;

impl<'a, 'src> Lexer<'a, 'src> {
    #[allow(clippy::len_without_is_empty)] // shut the FUCK up clippy
    #[inline(always)]
    pub fn len(&self) -> usize {
        // @SAFETY: this relies on VexationsSource providing the correct base
        // and limit pointers.
        unsafe { self.limit.byte_offset_from_unsigned(self.base) }
    }

    #[inline(always)]
    pub fn is_at_end(&self) -> bool {
        unlikely(self.cursor >= self.limit)
    }

    /// # @Safety
    ///
    /// `p` must be within the allocation range of VexationsSource.
    #[inline(always)]
    pub unsafe fn assert_ptr_valid(&self, p: *const u8) {
        unsafe {
            // @SAFETY: self.limit.add(3) will result in a pointer that is just
            // on the end of the allocation.
            let end_of_buffer = self.limit.add(3);
            // @SAFETY: caller guaranteed
            assert_unchecked(p >= self.base && p < end_of_buffer);
        }
    }

    /// # @Safety
    ///
    /// You may be at end, 1-past-end or 2-past-end. 3-past-end and above is UB.
    /// ```
    /// [0, 0, 0]
    ///  ^  ^  ^ safe
    /// end 1  2
    ///
    /// [0, 0, 0]
    ///           ^ danger!
    /// ```
    #[inline]
    pub unsafe fn incr_index_unchecked(&mut self) {
        unsafe {
            // @SAFETY: caller guaranteed
            self.cursor = self.cursor.add(1);
        }
    }

    /// # @Safety
    ///
    /// You may be at end, 1-past-end or 2-past-end. 3-past-end and above is UB.
    /// ```
    /// [0, 0, 0]
    ///  ^  ^  ^ safe
    /// end 1  2
    ///
    /// [0, 0, 0]
    ///           ^ danger!
    /// ```
    #[inline(always)]
    pub unsafe fn peek_unchecked(&self) -> u8 {
        unsafe {
            // @SAFETY: caller guaranteed
            self.assert_ptr_valid(self.cursor);
            *self.cursor
        }
    }

    /// # @Safety
    ///
    /// You may be at end, 1-past-end or 2-past-end. 3-past-end and above is UB.
    /// ```
    /// [0, 0, 0]
    ///  ^  ^  safe
    /// end 1
    ///
    /// [0, 0, 0]
    ///        ^ danger!
    /// ```
    #[inline]
    pub unsafe fn peek_next(&self) -> u8 {
        // @SAFETY: caller guaranteed
        unsafe { *self.cursor.add(1) }
    }

    /// You may be at the end after this function returns.
    ///
    /// # @Safety
    ///
    /// You may be at end, 1-past-end or 2-past-end. 3-past-end and above is UB.
    /// ```
    /// [0, 0, 0]
    ///  ^  ^  ^ safe
    /// end 1  2
    ///
    /// [0, 0, 0]
    ///           ^ danger!
    /// ```
    #[inline]
    pub unsafe fn advance_unchecked(&mut self) -> u8 {
        #[cfg(debug_assertions)]
        {
            let limit_addr = self.limit as isize;
            let cursor_addr = self.cursor as isize;
            let diff = cursor_addr - limit_addr;

            if diff < 0 {
            } else if diff == 0 {
                eprintln!("[DEBUG] at end");
            } else {
                eprintln!("[DEBUG] {} bytes past end", diff);
                if diff >= 3 {
                    eprintln!("[DEBUG]: warning! {} bytes past end.", diff);
                }
            }
        }
        unsafe {
            // @SAFETY: caller guaranteed
            let res = self.peek_unchecked();
            // @SAFETY: caller guaranteed
            self.incr_index_unchecked();
            res
        }
    }

    /// You may be at the end after this function returns.
    ///
    /// # @Safety
    ///
    /// You may be at end, 1-past-end or 2-past-end. 3-past-end and above is UB.
    /// ```
    /// [0, 0, 0]
    ///  ^  ^  ^ safe
    /// end 1  2
    ///
    /// [0, 0, 0]
    ///           ^ danger!
    /// ```
    #[inline]
    pub unsafe fn matches_unchecked(&mut self, expected: u8) -> bool {
        unsafe {
            // @SAFETY: caller guaranteed
            if self.peek_unchecked() == expected {
                // @SAFETY: caller guaranteed
                self.incr_index_unchecked();
                true
            } else {
                false
            }
        }
    }

    /// # @Safety
    ///
    /// You must be within bounds or at the end.
    /// ```
    /// [0, 0, 0]
    ///  ^ safe
    ///
    /// [0, 0, 0]
    ///     ^ danger!
    /// ```
    #[inline]
    pub unsafe fn make_lit(&self) -> &'src str {
        unsafe {
            // @SAFETY: self.start is never incremented with respect to
            // self.cursor, and they point to the same allocation
            // throughout.
            let lit_len = self.cursor.byte_offset_from_unsigned(self.start);
            // @SAFETY: cursor is never incremented beyond the allocation as per
            // the safety precondition of this function, therefore `lit_len`
            // bytes onward from self.start is within the
            // allocation.
            let slice = slice::from_raw_parts(self.start, lit_len);
            // @SAFETY: VexationsSource guarantees that the program source is
            // valid ASCII, which is trivially valid UTF-8.
            str::from_utf8_unchecked(slice)
        }
    }

    #[inline]
    pub fn location(&self) -> LineCol {
        todo!("Lexer::location")
    }
}
