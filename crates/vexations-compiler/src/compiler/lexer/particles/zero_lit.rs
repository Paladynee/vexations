use crate::compiler::lexer::Lexer;
use crate::compiler::lexer::error::LexerError;
use crate::compiler::lexer::error::LexerErrorKind;
use crate::frontend::token::TokenKind;

impl<'src> Lexer<'src> {
    /// Check for [`Lexer::is_at_end`] after this function returns.
    #[inline]
    pub fn zero_lit(&mut self) {
        // current lexer state looks like:
        // ```_
        // [0, ?...
        //  ^ start
        //     ^ index
        // ```

        // callsite PER_CHAR_DISPATCHER
        // we might be at source-end here, 3 more advances are valid
        // ```_
        // [a, b, c, \0, \0, \0]
        //            ^ index
        // ```

        let c = unsafe { self.peek_unchecked() };
        match c {
            b'b' => {
                unsafe { self.incr_unchecked() };
                // we might be at source-end here, 3 more advances are valid
                // ```_
                // [a, b, c, \0, \0, \0]
                //            ^ index
                // ```
                self.binary_lit();
                return;
            }
            b'o' => {
                unsafe { self.incr_unchecked() };
                // we might be at source-end here, 3 more advances are valid
                // ```_
                // [a, b, c, \0, \0, \0]
                //            ^ index
                // ```
                self.octal_lit();
                return;
            }
            b'x' => {
                unsafe { self.incr_unchecked() };
                // we might be at source-end here, 3 more advances are valid
                // ```_
                // [a, b, c, \0, \0, \0]
                //            ^ index
                // ```
                self.hexadecimal_lit();
                return;
            }
            b'.' => {
                unsafe { self.incr_unchecked() };
                // we might be at source-end here, 3 more advances are valid
                // ```_
                // [a, b, c, \0, \0, \0]
                //            ^ index
                // ```
                self.float_lit_remainder();
                return;
            }
            b'1'..=b'9' => {
                // error: leading zero in non-zero literal
                self.error_here(LexerErrorKind::LeadingZeroInNonZeroLiteral);
                return;
            }
            _ => {}
        };

        // just 0
        self.push_token_with_ident(TokenKind::LitInteger, self.make_identifier());
    }
}
