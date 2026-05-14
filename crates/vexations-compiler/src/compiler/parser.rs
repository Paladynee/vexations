use core::ptr;

use crate::frontend::source::VexationsSource;
use crate::frontend::token::TokenKind;

pub mod error;
pub mod plumbing;

pub struct Parser<'src> {
    source: VexationsSource<'src>,
    current: TokenKind,
    previous: TokenKind,

    tokens: Vec<TokenKind>,
    spans: Vec<usize>,
    idents: Vec<&'src str>,
}

impl<'src> Parser<'src> {
    #[inline]
    pub const fn new(
        source: VexationsSource<'src>, tokens: Vec<TokenKind>,
        spans: Vec<usize>, idents: Vec<&'src str>,
    ) -> Self {
        Parser {
            source,
            previous: TokenKind::MetaDummy,
            current: TokenKind::MetaDummy,

            tokens,
            spans,
            idents,
        }
    }

    #[inline]
    pub fn error_at(&self, tok: &TokenKind) {
        let idx = self.tokens.iter().position(|a| ptr::addr_eq(a, tok));
    }
}
