mod plumbing;
mod skip_whitespace;

use crate::source::LineCol;
use crate::source::VexationsSource;
use crate::token::TokenKind;

pub enum LexerErrorKind {
    UnclosedBlockComment,
}

pub struct LexerError {
    location: LineCol,
    kind: LexerErrorKind,
}

#[allow(unused)]
pub fn lex<'src>(
    src: &VexationsSource<'src>, tokens: &mut Vec<TokenKind>,
    errors: &mut Vec<LexerError>, idents: &mut Vec<&'src str>,
) {
    let mut lexer = Lexer::new(src);
    // lexer.lex_all(tokens, errors, idents);
}

pub struct Lexer<'a, 'src> {
    src: &'a VexationsSource<'src>,
    start: usize,
    index: usize,
}

impl<'a, 'src> Lexer<'a, 'src> {
    #[inline]
    pub const fn new(src: &'a VexationsSource<'src>) -> Self {
        Lexer {
            src,
            start: 0,
            index: 0,
        }
    }

    /// You may be at the end after this function returns.
    ///
    /// # @Safety
    ///
    /// `self.skip_whitespace()` must have been called prior
    /// `self.is_at_end()` be false.
    /// `self.start == self.index`
    #[inline]
    pub unsafe fn lex_one(
        &mut self, tokens: &mut Vec<TokenKind>, errors: &mut Vec<LexerError>,
        idents: &mut Vec<&'src str>,
    ) {
        // @SAFETY: explicit check @Lexer::lex_all
        let c = unsafe { self.advance_unchecked() };

        // we may be at the end here

        #[rustfmt::skip]
        let tok = match c {
            b'(' => TokenKind::IndentLParen, b')' => TokenKind::IndentRParen,
            b'{' => TokenKind::IndentLBrace, b'}' => TokenKind::IndentRBrace,
            b'[' => TokenKind::IndentLBracket, b']' => TokenKind::IndentRBracket,
            b',' => TokenKind::PuncComma, b'.' => TokenKind::PuncDot,
            b';' => TokenKind::PuncSemi,
            b':' => if self.matches(b':') { TokenKind::PuncColonColon } else { TokenKind::PuncColon },
            b'+' => if self.matches(b'=') { TokenKind::PuncPlusEq } else { TokenKind::PuncPlus },
            b'-' => if self.matches(b'=') { TokenKind::PuncMinusEq }
                else if self.matches(b'>') { TokenKind::PuncArrowRight }
                else { TokenKind::PuncMinus },
            b'!' => if self.matches(b'=') { TokenKind::PuncBangEq } else { TokenKind::PuncBang },
            b'*' => if self.matches(b'=') { TokenKind::PuncStarEq } else { TokenKind::PuncStar },
            // no need to handle comments here, skip_whitespace handles that
            b'/' => if self.matches(b'=') { TokenKind::PuncSlashEq } else { TokenKind::PuncSlash },
            b'%' => if self.matches(b'=') { TokenKind::PuncModuloEq } else { TokenKind::PuncModulo },
            b'^' => if self.matches(b'=') { TokenKind::PuncXorEq } else { TokenKind::PuncXor }
            b'=' => if self.matches(b'=') { TokenKind::PuncEqEq } else { TokenKind::PuncEq },
            b'<' => if self.matches(b'=') { TokenKind::PuncLtEq }
                else if self.matches(b'<') {
                    if self.matches(b'=') { TokenKind::PuncShlEq }
                    else { TokenKind::PuncShl }
                } else { TokenKind::PuncLt },
            b'>' => if self.matches(b'=') { TokenKind::PuncGtEq }
                else if self.matches(b'>') {
                    if self.matches(b'=') { TokenKind::PuncShrEq }
                    else { TokenKind::PuncShr }
                } else { TokenKind::PuncGt },
            b'|' => if self.matches(b'|') { TokenKind::PuncOrOr }
                else if self.matches(b'=') { TokenKind::PuncOrEq }
                else { TokenKind::PuncOr },
            b'&' => if self.matches(b'&') { TokenKind::PuncAndAnd }
                else if self.matches(b'=') { TokenKind::PuncAndEq }
                else { TokenKind::PuncAnd },
            
            
            
            _ => {todo!()},
        };

        tokens.push(tok);
    }

    #[inline]
    pub fn lex_all(
        &mut self, tokens: &mut Vec<TokenKind>, errors: &mut Vec<LexerError>,
        idents: &mut Vec<&'src str>,
    ) {
        while !self.is_at_end() {
            self.skip_whitespace(errors);

            // we may be at end here

            if self.is_at_end() {
                return;
            }

            // we can NOT be at end here

            self.start = self.index;
            unsafe {
                self.assert_within_bounds(self.index);
                self.assert_within_bounds(self.start);
                self.lex_one(tokens, errors, idents);
            }
        }
    }
}
