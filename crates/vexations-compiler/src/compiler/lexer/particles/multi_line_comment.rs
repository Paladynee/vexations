use crate::compiler::lexer::Lexer;
use crate::compiler::lexer::error::LexerErrorKind;

impl<'src> Lexer<'src> {
    /// Check for [`Lexer::is_at_end`] after this function returns.
    #[inline]
    pub unsafe fn multi_line_comment<const ALREADY_CONSUMED: bool>(&mut self) {
        if !ALREADY_CONSUMED {
            // current lexer state looks like:
            // ```_
            // [/, *, ?, ...
            //  ^ index
            // ```

            unsafe { self.incr_unchecked() };
            unsafe { self.incr_unchecked() };
        }
        // we might be at source-end here, 3 more advances are
        // valid
        // ```_
        // [/, *, \0, \0, \0]
        //         ^ index
        // ```

        if self.is_at_end() {
            self.error_here(LexerErrorKind::UnexpectedEndOfSource);
            return;
        }

        while !self.is_at_end() {
            let c = unsafe { self.advance_unchecked() };
            if c == b'*' {
                let c2 = unsafe { self.advance_unchecked() };
                if c2 == b'/' {
                    return;
                }
            }
        }
    }
}
