mod error;
mod plumbing;

use core::str;
use std::num::NonZeroUsize;

use crate::compiler::lexer::error::LexerError;
use crate::middle::source::LineCol;
use crate::middle::source::VexationsSource;
use crate::middle::token::TokenKind;

#[allow(unused)]
pub fn lex<'src>(
    src: VexationsSource<'src>,
) -> (Vec<TokenKind>, Vec<&'src str>, Vec<LexerError>) {
    let mut lexer = Lexer::new(src);
    lexer.lex_all();
    (lexer.tokens, lexer.idents, lexer.errors)
}

pub struct Lexer<'src> {
    src: VexationsSource<'src>,
    start: usize,
    index: usize,

    tokens: Vec<TokenKind>,
    idents: Vec<&'src str>,
    errors: Vec<LexerError>,
}

impl<'src> Lexer<'src> {
    #[inline]
    pub const fn new(src: VexationsSource<'src>) -> Self {
        Lexer {
            src,
            start: 0,
            index: 0,
            tokens: vec![],
            idents: vec![],
            errors: vec![],
        }
    }

    #[inline(never)]
    #[cold]
    pub fn location(src: &'src str, offset: usize) -> LineCol {
        let Some(prefix) = src.get(..offset) else {
            return LineCol {
                line: unsafe { NonZeroUsize::new_unchecked(1) },
                col: 0,
            };
        };

        let mut lc: usize = 1;
        let mut last_nl_offset: Option<usize> = None;

        for (i, b) in prefix.bytes().enumerate() {
            if b == b'\n' {
                lc += 1;
                last_nl_offset = Some(i);
            }
        }

        let col = match last_nl_offset {
            Some(nl_pos) => offset - (nl_pos + 1),
            None => offset,
        };

        LineCol {
            line: unsafe { core::num::NonZeroUsize::new_unchecked(lc) },
            col,
        }
    }

    #[inline]
    pub fn lex_all(&mut self) {
        //
    }
}
