use crate::compiler::lexer::Lexer;

impl<'src> Lexer<'src> {
    /// Check for [`Lexer::is_at_end`] after this function returns.
    #[inline]
    pub unsafe fn single_line_comment<const ALREADY_CONSUMED: bool>(&mut self) {
        if !ALREADY_CONSUMED {
            // current lexer state looks like:
            // ```_
            // [/, /, ?, ...
            //  ^ index
            // ```

            unsafe { self.incr_unchecked() };
            unsafe { self.incr_unchecked() };
        }

        // we might be at source-end here, 3 more advances are
        // valid
        // ```_
        // [/, /, \0, \0, \0]
        //         ^ index
        // ```

        while !self.is_at_end() {
            let c = unsafe { self.advance_unchecked() };
            if c == b'\n' {
                return;
            }
        }
    }
}
