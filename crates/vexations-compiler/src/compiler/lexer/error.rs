use core::fmt;
use std::fmt::Display;

use crate::compiler::lexer::Lexer;
use crate::frontend::source::LineCol;

#[derive(Debug, Clone)]
pub enum LexerErrorKind {
    UnclosedBlockComment,
    LeadingZeroInNonZeroLiteral,
    UnexpectedEndOfSource,
    FloatNoFractionalPart,
    NoBinaryDigits,
    NoOctalDigits,
    NoHexadecimalDigits,
    EmptyCharLiteral,
    UnknownCharacter(u8),
    UnknownEscapeSequence(u8),
    UnexpectedWhileExpecting(u8),
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
            LexerErrorKind::LeadingZeroInNonZeroLiteral => write!(
                f,
                "at {}, leading zero in non-zero number literal",
                self.location
            ),
            LexerErrorKind::UnexpectedEndOfSource => write!(
                f,
                "at {}, unexpectedly encountered end of source",
                self.location
            ),
            LexerErrorKind::FloatNoFractionalPart => write!(
                f,
                "at {}, float literal has no fractional part",
                self.location
            ),
            LexerErrorKind::NoBinaryDigits => write!(
                f,
                "at {}, binary literal has no binary part",
                self.location
            ),
            LexerErrorKind::NoOctalDigits => write!(
                f,
                "at {}, octal literal has no octal part",
                self.location
            ),
            LexerErrorKind::NoHexadecimalDigits => write!(
                f,
                "at {}, hexadecimal literal has no hexadecimal part",
                self.location
            ),
        }
    }
}

impl core::error::Error for LexerError {}

impl<'src> Lexer<'src> {
    #[inline(never)]
    #[cold]
    pub fn error_here(&mut self, kind: LexerErrorKind) {
        let err = LexerError {
            location: self.location(),
            kind,
        };
        self.errors.push(err);
    }
}
