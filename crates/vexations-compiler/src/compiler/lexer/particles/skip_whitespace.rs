use crate::compiler::lexer::Lexer;

impl<'src> Lexer<'src> {
    /// Check for [`Lexer::is_at_end`] after this function returns.
    #[inline]
    pub fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            unsafe {
                let c = self.peek_unchecked();
                if c.is_ascii_whitespace() {
                    self.incr_unchecked();
                } else {
                    break;
                }
            };
        }
    }
}
