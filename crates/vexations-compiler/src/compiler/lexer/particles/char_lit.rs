use crate::compiler::lexer::Lexer;
use crate::compiler::lexer::error::LexerError;
use crate::compiler::lexer::error::LexerErrorKind;
use crate::frontend::token::TokenKind;

impl<'src> Lexer<'src> {
    /// Check for [`Lexer::is_at_end`] after this function returns.
    #[inline]
    pub fn char_lit(&mut self) {
        // usually, the lexer state looks like:
        // ```_
        // ['\'', '?',
        //    ^ start
        //         ^ index
        // ```

        // callsite PER_CHAR_DISPATCHER
        // we might be at source-end here, 3 more advances are valid
        // ```_
        // [a, b, c, 0, 0, 0]
        //           ^ index
        // ```

        if self.is_at_end() {
            // eof while expecting character or escape
            // ```_
            // ['\'', 0, 0, 0]
            //    ^ start
            //        ^ index
            // ```
            self.error_here(LexerErrorKind::UnexpectedEndOfSource);
            return;
        }

        let c = unsafe { self.advance_unchecked() };
        match c {
            b'\'' => {
                // empty char literal while expecting character or escape
                // ```_
                // ['\'', '\'', ...
                //    ^ start
                //               ^ index
                // ```
                self.error_here(LexerErrorKind::EmptyCharLiteral);
                return;
            }

            b'\\' => {
                // usually, the lexer state looks like:
                // ```_
                // ['\'', '\\', ?, ...
                //    ^ start
                //              ^ index
                // ```

                if self.is_at_end() {
                    // eof while expecting escape sequence
                    // ```_
                    // ['\'', '\\', 0, 0, 0]
                    //    ^ start
                    //              ^ index
                    // ```
                    self.error_here(LexerErrorKind::UnexpectedEndOfSource);
                    return;
                }
                let next = unsafe { self.advance_unchecked() };
                if !matches!(next, b'\'' | b'\\' | b'n' | b'r' | b't') {
                    // unknown escape sequence
                    // ```_
                    // ['\'', '\\', 'x', ...
                    //    ^ start
                    //                    ^ index
                    // ```

                    // we might be at source-end here, 3 more advances are valid
                    // ```_
                    // [a, b, c, 0, 0, 0]
                    //           ^ index
                    // ```

                    unsafe {
                        // consume the closing quote if it exists.
                        if self.peek_unchecked() == b'\'' {
                            self.incr_unchecked();
                        }
                    }

                    // we might be 1 past source-end here, 2 more advances are
                    // valid
                    // ```_
                    // [a, b, c, 0, 0, 0]
                    //              ^ index
                    // ```

                    self.error_here(LexerErrorKind::UnknownEscapeSequence(
                        next,
                    ));
                    return;
                }

                // usually, the lexer state looks like:
                // ```_
                // ['\'', '\\', 'n', '?', ...
                //    ^ start
                //                    ^ index
                // ```

                // we might be at source-end here, 3 more advances are valid
                // ```_
                // [a, b, c, 0, 0, 0]
                //           ^ index
                // ```

                // expect (and consume) the closing quote
                unsafe { self.expect_consume_unchecked(b'\'') };

                // usually, the lexer state looks like:
                // ```_
                // ['\'', '\\', 'n', '\'', ';', ...
                //    ^ start
                //                          ^ index
                // ```

                // we might be at source-end here, 3 more advances are valid
                // ```_
                // [a, b, c, 0, 0, 0]
                //           ^ index
                // ```

                self.tokens.push(TokenKind::LitChar);
                let ident = self.make_identifier();
                self.idents.push(ident);
            }
            _ => {
                todo!()
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
