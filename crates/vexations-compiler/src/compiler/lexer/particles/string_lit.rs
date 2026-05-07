use core::hint::cold_path;

use crate::compiler::lexer::Lexer;
use crate::compiler::lexer::error::LexerErrorKind;
use crate::frontend::token::TokenKind;

impl<'src> Lexer<'src> {
    /// Check for [`Lexer::is_at_end`] after this function returns.
    #[inline]
    pub fn string_lit(&mut self) {
        // current lexer state looks like:
        // ```_
        // [", ?, ...
        //  ^ start
        //     ^ index
        // ```

        // callsite PER_CHAR_DISPATCHER
        // we might be at source-end here, 3 more advances are valid
        // ```_
        // [a, b, c, \0, \0, \0]
        //            ^ index
        // ```

        while !self.is_at_end() {
            let c = unsafe { self.advance_unchecked() };

            // we might be at source-end here, 3 more advances are valid
            // ```_
            // [a, b, c, \0, \0, \0]
            //            ^ index
            // ```

            match c {
                // escape
                b'\\' => {
                    // help the optimizer skip through the rest of the
                    // characters faster
                    cold_path();

                    // current lexer state looks like:
                    // ```_
                    // [", ..., \, ?, ...
                    //   ^ start
                    //             ^ index
                    // ```

                    if self.is_at_end() {
                        // unexpected eof while expecting string escape
                        // ```_
                        // [", ..., \, \0, \0, \0]
                        //   ^ start
                        //              ^ index
                        // ```
                        self.error_here(LexerErrorKind::UnexpectedEndOfSource);
                        return;
                    }

                    // todo: multi-char escapes

                    let escaped = unsafe { self.advance_unchecked() };

                    // we might be at source-end here, 3 more advances are valid
                    // ```_
                    // [a, b, c, \0, \0, \0]
                    //            ^ index
                    // ```

                    if !matches!(
                        escaped,
                        b'\"' | b'\\' | b'0' | b'n' | b'r' | b't'
                    ) {
                        // current lexer state looks like
                        // ```_
                        // [", ..., \, x, ?, ...
                        //   ^ start
                        //                ^ index
                        // ```

                        // consume the closing quotes if it exists
                        unsafe {
                            if self.peek_unchecked() == b'"' {
                                self.incr_unchecked();
                            }
                        }

                        // current lexer state looks like either
                        // ```_
                        // [", ..., \, x, ", ?, ...
                        //   ^ start
                        //                   ^ index
                        // ```
                        // or
                        // ```_
                        // [", ..., \, x, ?, ...
                        //   ^ start
                        //                ^ index
                        // ```

                        self.error_here(LexerErrorKind::UnknownEscapeSequence(
                            escaped,
                        ));
                        return;
                    }
                }
                // close string
                b'"' => {
                    // help the optimizer skip through the rest of the
                    // characters faster
                    cold_path();

                    // current lexer state looks like:
                    // ```_
                    // [", ..., ", ?, ...
                    //   ^ start
                    //             ^ index
                    // ```
                    self.push_token_with_ident(
                        TokenKind::LitStr,
                        self.make_identifier(),
                    );
                    return;
                }
                // any other ascii character
                // VexationsSource guarantees we're working with ascii here
                _ => continue,
            }
        }

        // `string_lit` returns above when the closing quote is reached, so the
        // current lexer state looks like:
        // ```_
        // [", ..., \0, \0, \0]
        //   ^ start
        //           ^ index
        // ```
    }
}
