pub mod error;
mod particles;
mod per_char_dispatch;
mod plumbing;

use core::str;
use core::mem;

use crate::compiler::lexer::error::LexerError;
use crate::frontend::source::VexationsSource;
use crate::frontend::token::TokenKind;

#[allow(unused)]
pub fn lex<'src>(src: VexationsSource<'src>) -> Lexer<'src> {
    let mut lexer = Lexer::new(src);
    lexer.lex_all();
    lexer
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

    #[inline]
    pub fn new_reuse_allocations(
        src: VexationsSource<'src>, mut v1: Vec<TokenKind>, mut v2: Vec<usize>,
        mut v3: Vec<&'src str>, mut v4: Vec<LexerError>,
    ) -> Lexer<'src> {
        v1.clear();
        v2.clear();
        v3.clear();
        v4.clear();
        Lexer {
            src,
            start: 0,
            index: 0,
            tokens: v1,
            spans: v2,
            idents: v3,
            errors: v4,
        }
    }
    /// # Safety
    ///
    /// Only sound if you don't read from v3 afterwards.
    #[inline]
    pub unsafe fn new_reuse_static_allocations(
        src: VexationsSource<'src>, mut v1: Vec<TokenKind>, mut v2: Vec<usize>,
        mut v3: Vec<&'static str>, mut v4: Vec<LexerError>,
    ) -> Lexer<'src> {
        v1.clear();
        v2.clear();
        v3.clear();
        v4.clear();
        Lexer {
            src,
            start: 0,
            index: 0,
            tokens: v1,
            spans: v2,
            idents: unsafe {
                mem::transmute::<Vec<&'static str>, Vec<&'_ str>>(v3)
            },
            errors: v4,
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

    #[inline]
    pub fn tokens_view(&self) -> &[TokenKind] {
        self.tokens.as_slice()
    }

    #[inline]
    pub fn spans_view(&self) -> &[usize] {
        self.spans.as_slice()
    }

    #[inline]
    pub fn idents_view(&self) -> &[&'src str] {
        self.idents.as_slice()
    }

    #[inline]
    pub fn errors_view(&self) -> &[LexerError] {
        self.errors.as_slice()
    }

    #[inline]
    pub fn take_tokens(&mut self) -> Vec<TokenKind> {
        mem::take(&mut self.tokens)
    }

    #[inline]
    pub fn take_spans(&mut self) -> Vec<usize> {
        mem::take(&mut self.spans)
    }

    #[inline]
    pub fn take_idents(&mut self) -> Vec<&'src str> {
        mem::take(&mut self.idents)
    }

    #[inline]
    pub fn take_errors(&mut self) -> Vec<LexerError> {
        mem::take(&mut self.errors)
    }

    #[inline]
    pub fn finalize(
        self,
    ) -> (Vec<TokenKind>, Vec<usize>, Vec<&'src str>, Vec<LexerError>) {
        (self.tokens, self.spans, self.idents, self.errors)
    }
}
