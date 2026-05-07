use crate::compiler::lexer::Lexer;
use crate::compiler::lexer::error::LexerErrorKind;
use crate::frontend::token::TokenKind;

impl<'src> Lexer<'src> {
    /// Check for [`Lexer::is_at_end`] after this function returns.
    #[inline]
    pub fn char_lit(&mut self) {
        // current lexer state looks like:
        // ```_
        // [', ?, ...
        //  ^ start
        //     ^ index
        // ```

        // callsite PER_CHAR_DISPATCHER
        // we might be at source-end here, 3 more advances are valid
        // ```_
        // [a, b, c, \0, \0, \0]
        //            ^ index
        // ```

        if self.is_at_end() {
            // eof while expecting character or escape
            // ```_
            // [', \0, \0, \0]
            //  ^ start
            //      ^ index
            // ```
            self.error_here(LexerErrorKind::UnexpectedEndOfSource);
            return;
        }

        let c = unsafe { self.advance_unchecked() };

        // we might be at source-end here, 3 more advances are valid
        // ```_
        // [a, b, c, \0, \0, \0]
        //            ^ index
        // ```

        match c {
            b'\'' => {
                // empty char literal while expecting character or escape
                // ```_
                // [', ', ?, ...
                //  ^ start
                //        ^ index
                // ```
                self.error_here(LexerErrorKind::EmptyCharLiteral);
            }

            b'\\' => {
                // current lexer state looks like:
                // ```_
                // [', \, ?, ...
                //  ^ start
                //        ^ index
                // ```

                if self.is_at_end() {
                    // eof while expecting escape sequence
                    // ```_
                    // [', \, \0, \0, \0]
                    //  ^ start
                    //         ^ index
                    // ```
                    self.error_here(LexerErrorKind::UnexpectedEndOfSource);
                    return;
                }
                let escaped = unsafe { self.advance_unchecked() };

                // we might be at source-end here, 3 more advances are valid
                // ```_
                // [a, b, c, \0, \0, \0]
                //            ^ index
                // ```

                if !matches!(escaped, b'\'' | b'\\' | b'0' | b'n' | b'r' | b't')
                {
                    // unknown escape sequence
                    // ```_
                    // [', \, x, ?, ...
                    //  ^ start
                    //           ^ index
                    // ```

                    unsafe {
                        // consume the closing quote **if it exists**.
                        // ```_
                        // [', \, x, ', ...
                        //  ^ start
                        //           ^ index
                        // ```
                        if self.peek_unchecked() == b'\'' {
                            self.incr_unchecked();
                        }
                    }

                    // current lexer state looks like either:
                    // ```_
                    // [', \, x, ', ?, ...
                    //  ^ start
                    //              ^ index
                    // ```
                    // or
                    // ```_
                    // [', \, x, ?, ...
                    //  ^ start
                    //           ^ index
                    // ```

                    // we might be 1 past source-end here, 2 more advances are
                    // valid
                    // ```_
                    // [a, b, c, \0, \0, \0]
                    //                ^ index
                    // ```

                    self.error_here(LexerErrorKind::UnknownEscapeSequence(
                        escaped,
                    ));
                    return;
                }

                // current lexer state looks like:
                // ```_
                // [', \, n, ?, ...
                //  ^ start
                //           ^ index
                // ```

                // expect (and consume) the closing quote
                if !unsafe { self.expect_consume_unchecked(b'\'') } {
                    return;
                };

                // current lexer state looks like:
                // ```_
                // [', \, n, ', ?, ...
                //  ^ start
                //              ^ index
                // ```

                // we might be at source-end here, 3 more advances are valid
                // ```_
                // [a, b, c, \0, \0, \0]
                //            ^ index
                // ```

                self.push_token_with_ident(
                    TokenKind::LitChar,
                    self.make_identifier(),
                );
            }
            _ => {
                // current lexer state looks like:
                // ```_
                // [', a, ?, ...
                //  ^ start
                //        ^ index
                // ```

                if !unsafe { self.expect_consume_unchecked(b'\'') } {
                    return;
                };

                // current lexer state looks like:
                // ```_
                // [', a, ', ?, ...
                //  ^ start
                //           ^ index
                // ```

                // we might be at source-end here, 3 more advances are valid
                // ```_
                // [a, b, c, \0, \0, \0]
                //           ^ index
                // ```

                self.push_token_with_ident(
                    TokenKind::LitChar,
                    self.make_identifier(),
                );
            }
        }
    }
}
