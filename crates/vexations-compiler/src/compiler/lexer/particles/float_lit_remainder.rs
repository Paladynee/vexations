use crate::compiler::lexer::Lexer;
use crate::compiler::lexer::error::LexerError;
use crate::compiler::lexer::error::LexerErrorKind;
use crate::frontend::token::TokenKind;

impl<'src> Lexer<'src> {
    /// Check for [`Lexer::is_at_end`] after this function returns.
    #[inline]
    pub fn float_lit_remainder(&mut self) {
        // usually, the lexer state looks like:
        // ```_
        // ['1', '.', '?'...
        //   ^ start
        //             ^ index
        // ```

        // callsite decimal_lit
        // we might be at source-end here, 3 more advances are valid
        // ```_
        // [a, b, c, 0, 0, 0]
        //           ^ index
        // ```

        if self.is_at_end() {
            // eof while expecting fractional part
            // ```_
            // ['1', '.', 0, 0, 0]
            //   ^ start
            //            ^ index
            // ```
            self.error_here(LexerErrorKind::UnexpectedEndOfSource);
            return;
        }

        let c = unsafe { self.peek_unchecked() };
        match c {
            b'0'..=b'9' => unsafe { self.incr_unchecked() },
            _ => {
                // no fractional part error
                // ```_
                // ['1', '.', ';'...
                //   ^ start
                //             ^ index
                // ```
                self.error_here(LexerErrorKind::FloatNoFractionalPart);
                return;
            }
        };

        // we might be at source-end here, 3 more advances are valid
        // ```_
        // [a, b, c, 0, 0, 0]
        //           ^ index
        // ```
        while !self.is_at_end() {
            let c = unsafe { self.peek_unchecked() };
            match c {
                b'0'..=b'9' => unsafe { self.incr_unchecked() },
                _ => break,
            }
        }

        // ```_
        // ['1', '.', '0', ';'...
        //   ^ start
        //                  ^ index
        // ```

        // we might be at source-end here, 3 more advances are valid
        // ```_
        // [a, b, c, 0, 0, 0]
        //           ^ index
        // ```
        self.tokens.push(TokenKind::LitFloat);
        let ident = self.make_literal();
        self.idents.push(ident);
    }
}
