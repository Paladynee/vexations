use core::fmt;
use std::fmt::Display;

use crate::frontend::source::LineCol;

#[derive(Debug, Clone)]
pub enum LexerErrorKind {
    UnclosedBlockComment,
    UnknownCharacter(u8),
}

#[derive(Debug, Clone)]
pub struct LexerError {
    pub location: LineCol,
    pub kind: LexerErrorKind,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            LexerErrorKind::UnclosedBlockComment =>
                write!(f, "at {}, unclosed block comment", self.location),
            LexerErrorKind::UnknownCharacter(c) =>
                write!(f, "at {}, unknown character: '{c:#x}'", self.location),
        }
    }
}

impl core::error::Error for LexerError {}
