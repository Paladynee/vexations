mod error;
mod particles;
mod per_char_dispatch;
mod plumbing;

use core::num::NonZeroUsize;
use core::str;

use crate::compiler::lexer::error::LexerError;
use crate::frontend::source::Span;
use crate::frontend::source::VexationsSource;
use crate::frontend::token::TokenKind;

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
    spans: Vec<usize>,
    idents: Vec<&'src str>,
    errors: Vec<LexerError>,
}

impl<'src> Lexer<'src> {
    #[inline]
    pub fn new(src: VexationsSource<'src>) -> Self {
        let guess = src.source().len() / 4;
        let ident_guess = src.source().len() / 8;
        Lexer {
            src,
            start: 0,
            index: 0,
            tokens: Vec::with_capacity(guess),
            spans: Vec::with_capacity(guess),
            idents: Vec::with_capacity(ident_guess),
            errors: vec![],
        }
    }

    #[inline(always)]
    pub fn push_token(&mut self, token_kind: TokenKind) {
        self.tokens.push(token_kind);
        self.spans.push(self.start);
    }

    #[inline(always)]
    pub fn push_token_with_ident(
        &mut self, token_kind: TokenKind, ident: &'src str,
    ) {
        self.push_token(token_kind);
        self.idents.push(ident);
    }

    #[inline(never)]
    #[cold]
    pub fn location(&self) -> Span {
        let src = self.source();
        let Some(prefix) = src.get(..self.start) else {
            // shouldn't happen, self.start never hits the padding bytes at the
            // end of the source.
            // ```_
            // [a, b, c, \0, \0, \0]
            //        ^ start never goes past this character
            // ```
            return Span {
                line: unsafe { NonZeroUsize::new_unchecked(1) },
                col: 0,
                source_offset: self.start,
                span_length: 0,
            };
        };

        let prefix = prefix.as_bytes();

        let mut lc: usize = 1;
        let mut last_nl_offset: Option<usize> = None;

        for i in 0..prefix.len() {
            // rustc gets rid of this indexing panic for us thanks to loop
            // invariant being simple asf
            if prefix[i] == b'\n' {
                lc += 1;
                last_nl_offset = Some(i);
            }
        }

        let col = match last_nl_offset {
            Some(nl_pos) => self.start - (nl_pos + 1),
            None => self.start,
        };

        Span {
            line: unsafe { NonZeroUsize::new_unchecked(lc) },
            col,
            source_offset: self.start,
            span_length: unsafe { self.index.unchecked_sub(self.start) },
        }
    }

    #[inline]
    pub fn lex_all(&mut self) {
        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                return;
            }

            self.start = self.index;
            let c = unsafe { self.advance_unchecked() };
            per_char_dispatch::PER_CHAR_DISPATCHER[c as usize](self);
        }
    }
}
