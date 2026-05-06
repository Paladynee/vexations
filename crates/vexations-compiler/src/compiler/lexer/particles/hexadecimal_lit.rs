use crate::compiler::lexer::Lexer;
use crate::compiler::lexer::error::LexerError;
use crate::compiler::lexer::error::LexerErrorKind;
use crate::frontend::token::TokenKind;

impl<'src> Lexer<'src> {
    /// Check for [`Lexer::is_at_end`] after this function returns.
    #[inline]
    pub fn hexadecimal_lit(&mut self) {
        // usually, the lexer state looks like:
        // ```_
        // ['0', 'x', '?'...
        //   ^ start
        //             ^ index
        // ```

        // callsite zero_lit
        // we might be at source-end here, 3 more advances are valid
        // ```_
        // [a, b, c, 0, 0, 0]
        //           ^ index
        // ```

        if self.is_at_end() {
            // eof while expecting hexadecimal digits
            // ```_
            // ['0', 'x', 0, 0, 0]
            //   ^ start
            //            ^ index
            // ```
            self.error_here(LexerErrorKind::UnexpectedEndOfSource);
            return;
        }

        let c = unsafe { self.peek_unchecked() };
        if matches!(c, b'0'..=b'9')
            | matches!(c, b'a'..=b'f')
            | matches!(c, b'A'..=b'F')
        {
            unsafe { self.incr_unchecked() }
        } else {
            // no hexadecimal digits error
            // ```_
            // ['0', 'x', ';'...
            //   ^ start
            //             ^ index
            // ```
            self.error_here(LexerErrorKind::NoHexadecimalDigits);
            return;
        };

        // we might be at source-end here, 3 more advances are valid
        // ```_
        // [a, b, c, 0, 0, 0]
        //           ^ index
        // ```
        while !self.is_at_end() {
            let c = unsafe { self.peek_unchecked() };
            if matches!(c, b'0'..=b'9')
                | matches!(c, b'a'..=b'f')
                | matches!(c, b'A'..=b'F')
            {
                unsafe { self.incr_unchecked() }
            } else {
                break;
            }
        }

        // ```_
        // ['0', 'x', 'f', ';'...
        //   ^ start
        //                  ^ index
        // ```

        // we might be at source-end here, 3 more advances are valid
        // ```_
        // [a, b, c, 0, 0, 0]
        //           ^ index
        // ```
        self.tokens.push(TokenKind::LitInteger);
        let ident = self.make_identifier();
        self.idents.push(ident);
    }
}
