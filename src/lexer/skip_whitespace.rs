use crate::lexer::Lexer;
use crate::lexer::LexerError;
use crate::lexer::LexerErrorKind;

impl<'a, 'src> Lexer<'a, 'src> {
    /// You may be at the end after this function returns.
    /// [0, 0, 0]
    ///  ^
    ///
    /// # @Safety
    ///
    /// - `self.is_at_end()` must be false.
    /// - `self.peek_next()` must have returned Some(_).
    /// - `self.src()[self.index..self.index+2] == "//"`
    /// - [`Lexer`] must not have been modified after the `peek_next`
    #[inline]
    pub unsafe fn unconsumed_single_line_comment(&mut self) {
        // consuming the first /
        // @SAFETY: loop condition + no modification to
        // lexer yet
        unsafe { self.incr_index_unchecked() };

        // we may NOT be at end here due to peek_next

        // consuming the second /
        // @SAFETY: peek_next success
        unsafe { self.incr_index_unchecked() };

        // we may be at end here

        // skipping until the next new line
        'skip_line: while !self.is_at_end() {
            // @SAFETY: loop condition
            let c2 = unsafe { self.advance_unchecked() };

            // we may be at end here

            if c2 == b'\n' {
                break 'skip_line;
            }
        }

        // we may be at end here, returned
    }

    /// You may be at the end after this function returns.
    /// ```
    /// [0, 0, 0]
    ///     ^
    /// ```
    ///
    /// # @Safety
    /// - `self.is_at_end()` must be false.
    /// - `self.peek_next()` must have returned Some(_).
    /// - `self.src()[self.index..self.index+2] == "/*"`
    /// - [`Lexer`] must not have been modified after the `peek_next`
    #[inline]
    pub unsafe fn unconsumed_block_comment(
        &mut self, errors: &mut Vec<LexerError>,
    ) {
        // consuming the /
        // @SAFETY: loop condition + no modification to
        // lexer yet
        unsafe { self.incr_index_unchecked() }

        // we may NOT be at end here due to peek_next

        // consuming the *
        // @SAFETY: peek_next success
        unsafe { self.incr_index_unchecked() }

        // we may be at end here
        // [0, 0, 0]
        //  ^

        'block_comment: while !self.is_at_end() {
            // @SAFETY: loop condition
            let c = unsafe { self.advance_unchecked() };

            // we may be at end here
            // [0, 0, 0]
            //  ^

            match c {
                // c ==
                b'*' => {
                    // @SAFETY: being 1 past end is sound due to
                    // VexationsSource.
                    if (unsafe { self.advance_unchecked() }) == b'/' {
                        return;
                    }

                    // we may be at end here
                    // [0, 0, 0]
                    //     ^

                    if self.is_at_end() {
                        errors.push(LexerError {
                            location: self.location(),
                            kind: LexerErrorKind::UnclosedBlockComment,
                        });
                        return;
                    }

                    // we may no longer be at the end here
                    // we could inline another unchecked iteration of the
                    // 'block_comment but that'd hurt icache so much lolol

                    continue 'block_comment;
                }
                // c ==
                _ => continue 'block_comment,
            };

            unreachable!(
                "handle all match cases in `unconsumed_block_comment`"
            );
        }

        // we may be at end here, to be returned
        // [0, 0, 0]
        //  ^

        if self.is_at_end() {
            errors.push(LexerError {
                location: self.location(),
                kind: LexerErrorKind::UnclosedBlockComment,
            });
        }

        // returned maybe at end
    }

    /// You may be at the end after this function returns.
    /// ```
    /// [0, 0, 0]
    ///     ^
    /// ```
    ///
    /// If you were at or past the end, your past-end is preserved.
    #[inline]
    pub fn skip_whitespace(&mut self, errors: &mut Vec<LexerError>) {
        'search: while !self.is_at_end() {
            // @SAFETY: loop condition
            let c = unsafe { self.peek_unchecked() };
            match c {
                // c ==
                b' ' | b'\n' | b'\t' | b'\r' => {
                    // @SAFETY: loop condition + no modification to lexer yet
                    unsafe { self.incr_index_unchecked() }

                    // we may be at end here
                    // [0, 0, 0]
                    //  ^

                    continue 'search;
                }
                // c ==
                b'/' => {
                    match unsafe { self.peek_next() } {
                        // peek_next ==
                        b'/' => {
                            unsafe { self.unconsumed_single_line_comment() };

                            // we may be at end here
                            // [0, 0, 0]
                            //  ^

                            continue 'search;
                        }
                        // peek_next ==
                        b'*' => {
                            unsafe { self.unconsumed_block_comment(errors) };

                            // we may be at end here
                            // [0, 0, 0]
                            //     ^

                            continue 'search;
                        }
                        // peek_next ==
                        _ => break 'search,
                    };

                    unreachable!(
                        "handle all branches in `skip_whitespace` \
                        comment handling"
                    )
                }
                // c ==
                _ => break,
            };

            unreachable!("handle all branches in `skip_whitespace`")
        }

        // we may be at end here, returned
        // [0, 0, 0]
        //     ^
        // OR
        // preserved
    }
}
