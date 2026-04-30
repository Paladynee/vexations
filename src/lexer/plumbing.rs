use std::hint::assert_unchecked;
use std::hint::unlikely;

use crate::lexer::Lexer;
use crate::source::LineCol;

impl<'a, 'src> Lexer<'a, 'src> {
    #[inline(always)]
    pub fn len(&self) -> usize {
        unsafe { self.limit.offset_from(self.base) as usize }
    }

    #[inline(always)]
    pub fn offset(&self) -> usize {
        unsafe { self.cursor.offset_from(self.base) as usize }
    }

    #[inline(always)]
    pub fn is_at_end(&self) -> bool {
        unlikely(self.cursor >= self.limit)
    }

    /// # @Safety
    ///
    /// `p` must be within the allocated range of VexationSource.
    #[inline(always)]
    pub unsafe fn assert_ptr_valid(&self, p: *const u8) {
        unsafe {
            let end_of_buffer = self.limit.add(3);
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
            self.assert_ptr_valid(self.cursor);
            *self.cursor
        }
    }

    #[inline]
    pub unsafe fn peek_next(&self) -> u8 {
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
            let res = self.peek_unchecked();
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
            if self.peek_unchecked() == expected {
                self.incr_index_unchecked();
                true
            } else {
                false
            }
        }
    }

    #[inline]
    pub fn location(&self) -> LineCol {
        todo!("Lexer::location")
    }
}
