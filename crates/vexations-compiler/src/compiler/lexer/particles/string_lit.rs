use crate::compiler::lexer::Lexer;
use crate::compiler::lexer::error::LexerError;
use crate::compiler::lexer::error::LexerErrorKind;
use crate::frontend::token::TokenKind;

impl<'src> Lexer<'src> {
    /// Check for [`Lexer::is_at_end`] after this function returns.
    #[inline]
    pub fn string_lit(&mut self) {
        // usually, the lexer state looks like:
        // ```_
        // ['0', 'o', '?'...
        //   ^ start
        //             ^ index
        // ```

        // callsite PER_CHAR_DISPATCHER
        // we might be at source-end here, 3 more advances are valid
        // ```_
        // [a, b, c, 0, 0, 0]
        //           ^ index
        // ```

        if self.is_at_end() {
            // eof while expecting octal digits
            // ```_
            // ['0', 'o', 0, 0, 0]
            //   ^ start
            //            ^ index
            // ```
            self.error_here(LexerErrorKind::UnexpectedEndOfSource);
            return;
        }

        let c = unsafe { self.peek_unchecked() };
        match c {
            b'0'..=b'7' => unsafe { self.incr_unchecked() },
            _ => {
                // no octal digits error
                // ```_
                // ['0', 'o', ';'...
                //   ^ start
                //             ^ index
                // ```
                self.error_here(LexerErrorKind::NoOctalDigits);
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
                b'0'..=b'7' => unsafe { self.incr_unchecked() },
                _ => break,
            }
        }

        // ```_
        // ['0', 'o', '7', ';'...
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
