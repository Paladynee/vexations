use crate::lexer::Lexer;
use crate::lexer::LexerError;
use crate::lexer::per_char_dispatch::PushUnchecked;
use crate::token::TokenKind;

impl<'a, 'src> Lexer<'a, 'src> {
    #[inline]
    pub unsafe fn zero_lit(
        &mut self, tokens: &mut Vec<TokenKind>, errors: &mut Vec<LexerError>,
        idents: &mut Vec<&'src str>,
    ) {
        // when we're called, we might be at the end
        // [0, 0, 0]
        //  ^

        let specifier = unsafe { self.peek_unchecked() };
        match specifier {
            b'x' => {
                // @SAFETY: being at end is safe due to VexationsSource.
                unsafe { self.incr_index_unchecked() };

                // we might be at the end
                // [0, 0, 0]
                //     ^

                unsafe { self.hex_lit(tokens, errors, idents) };
                return;
            }
            b'o' => {
                // @SAFETY: being at end is safe due to VexationsSource.
                unsafe { self.incr_index_unchecked() };

                // we might be at the end
                // [0, 0, 0]
                //     ^

                unsafe { self.octal_lit(tokens, errors, idents) };
                return;
            }
            _ => {
                unsafe { self.number_lit(tokens, errors, idents) };
                return;
            }
        }

        unreachable!("handle all match arms in `Lexer::zero_lit`");
    }

    #[inline]
    pub unsafe fn hex_lit(
        &mut self, tokens: &mut Vec<TokenKind>, errors: &mut Vec<LexerError>,
        idents: &mut Vec<&'src str>,
    ) {
        // we might be at the end
        // [0, 0, 0]
        //     ^
        todo!()
    }

    #[inline]
    pub unsafe fn octal_lit(
        &mut self, tokens: &mut Vec<TokenKind>, errors: &mut Vec<LexerError>,
        idents: &mut Vec<&'src str>,
    ) {
        // we might be at the end
        // [0, 0, 0]
        //     ^
        todo!()
    }

    #[inline]
    pub unsafe fn number_lit(
        &mut self, tokens: &mut Vec<TokenKind>, errors: &mut Vec<LexerError>,
        idents: &mut Vec<&'src str>,
    ) {
        // we might be at the end
        // [0, 0, 0]
        //  ^

        'integral_skip: while !self.is_at_end() {
            // @SAFETY: loop condition
            let c = unsafe { self.advance_unchecked() };
            match c {
                b'0'..=b'9' => continue 'integral_skip,
                _ => break 'integral_skip,
            }
        }

        if !self.is_at_end() && unsafe { self.peek_unchecked() } == b'.' {
            // @SAFETY: if condition
            unsafe { self.incr_index_unchecked() };

            // we might be at the end
            // [0, 0, 0]
            //  ^

            return unsafe { self.float_lit(tokens, errors, idents) };
        }

        // @integer_lit

        // we still might be at the end
        // [0, 0, 0]
        //  ^

        // @SAFETY: we might be exactly on the end, or within bounds.
        let lit = unsafe { self.make_lit() };
        // @SAFETY: the vectors were extended with at least 1 capacity within
        // `Lexer::lex_one`.
        unsafe {
            tokens.push_unchecked(TokenKind::LitInteger);
            idents.push_unchecked(lit);
        };
    }

    #[inline]
    pub unsafe fn float_lit(
        &mut self, tokens: &mut Vec<TokenKind>, errors: &mut Vec<LexerError>,
        idents: &mut Vec<&'src str>,
    ) {
        // we might be at the end
        // [0, 0, 0]
        //  ^

        todo!()
    }
}
