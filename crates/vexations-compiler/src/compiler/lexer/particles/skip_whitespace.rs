use crate::compiler::lexer::Lexer;

impl<'src> Lexer<'src> {
    /// Check for [`Lexer::is_at_end`] after this function returns.
    #[inline]
    pub fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            let c = unsafe { self.peek_unchecked() };
            if c.is_ascii_whitespace() {
                unsafe { self.incr_unchecked() };
                continue;
            }
            break;
        }
    }
}
