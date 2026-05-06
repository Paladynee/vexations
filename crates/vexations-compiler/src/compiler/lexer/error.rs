use core::fmt;
use core::fmt::Display;

use crate::compiler::lexer::Lexer;
use crate::frontend::source::Span;
use crate::frontend::source::VexationsSource;

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

#[derive(Clone)]
pub struct LexerError {
    pub location: Span,
    pub kind: LexerErrorKind,
}

impl LexerError {
    pub fn display<'src>(self, source: VexationsSource<'src>) -> impl Display {
        LexerErrorDisplay {
            err: self,
            source,
        }
    }

    /// Formats just the error message
    pub fn format_error_message(
        &self, f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self.kind {
            LexerErrorKind::UnclosedBlockComment =>
                write!(f, "unclosed block comment"),
            LexerErrorKind::LeadingZeroInNonZeroLiteral =>
                write!(f, "leading zero in non-zero number literal"),
            LexerErrorKind::UnexpectedEndOfSource =>
                write!(f, "unexpectedly encountered end of source"),
            LexerErrorKind::FloatNoFractionalPart =>
                write!(f, "float literal has no fractional part"),
            LexerErrorKind::NoBinaryDigits =>
                write!(f, "binary literal has no binary part"),
            LexerErrorKind::NoOctalDigits =>
                write!(f, "octal literal has no octal part"),
            LexerErrorKind::NoHexadecimalDigits =>
                write!(f, "hexadecimal literal has no hexadecimal part"),
            LexerErrorKind::EmptyCharLiteral =>
                write!(f, "empty character literal"),
            LexerErrorKind::UnknownCharacter(c) =>
                write!(f, "unknown character: {:?}", c as char),
            LexerErrorKind::UnknownEscapeSequence(c) =>
                write!(f, "unknown escape sequence: {:?}", c as char),
            LexerErrorKind::UnexpectedWhileExpecting(c) =>
                write!(f, "unexpected character, expected {:?}", c as char),
        }
    }

    pub fn error_message_formatter(&self) -> impl Display {
        struct ErrorMessageFormatter<'s> {
            err: &'s LexerError,
        }

        impl Display for ErrorMessageFormatter<'_> {
            #[inline(always)]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.err.format_error_message(f)
            }
        }

        ErrorMessageFormatter {
            err: self,
        }
    }

    /// Formats the location and the error into the formatter without any source
    /// code diagnostics
    pub fn format_no_diagnostics(
        &self, f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self.kind {
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
            LexerErrorKind::EmptyCharLiteral =>
                write!(f, "at {}, empty character literal", self.location),
            LexerErrorKind::UnknownEscapeSequence(c) => write!(
                f,
                "at {}, unknown escape sequence: '{c:#x}'",
                self.location
            ),
            LexerErrorKind::UnexpectedWhileExpecting(c) => write!(
                f,
                "at {}, unexpected character, expected '{c:#x}'",
                self.location
            ),
        }
    }
}

pub struct LexerErrorDisplay<'src> {
    pub err: LexerError,
    pub source: VexationsSource<'src>,
}

impl<'src> Display for LexerErrorDisplay<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let src = self.source.source();
        let Some(src_substring_start) =
            src.get(self.err.location.source_offset..)
        else {
            // shouldn't really happen, fall back to showing just the error
            return self.err.format_no_diagnostics(f);
        };
        // trim the source substring on the first newline to just show that line
        // todo: backwards search for the previous newline/source start
        let src_line = src_substring_start
            .split_once('\n')
            .map(|(line, _)| line)
            .unwrap_or(src_substring_start);

        // error: {message}
        //   at {location}:
        // {line num} | {source line}
        // {some spaces} ^
        // format the source line
        static ANSI_BOLD_RED: &str = "\x1b[1;31m";
        static ANSI_BOLD_WHITE: &str = "\x1b[1;37m";
        static ANSI_WHITE: &str = "\x1b[37m";
        static ANSI_CYAN: &str = "\x1b[36m";
        static ANSI_GRAY: &str = "\x1b[90m";
        static ANSI_RESET: &str = "\x1b[0m";
        writeln!(
            f,
            "goddamn location number: {}",
            self.err.location.line.get()
        );
        writeln!(
            f,
            "{ANSI_BOLD_RED}error{ANSI_RESET}: \
            {ANSI_BOLD_WHITE}{error_msg}{ANSI_RESET}\
            \n    at {ANSI_GRAY}{loc}{ANSI_RESET}:\n\
            {ANSI_CYAN}{line_num}{ANSI_RESET} | \
            {ANSI_WHITE}{src}{ANSI_RESET}\n\
            {spaces} ^ here",
            error_msg = self.err.error_message_formatter(),
            loc = self.err.location,
            line_num = self.err.location.line.get() as usize,
            src = src_line,
            spaces = " ".repeat(
                decimal_length(self.err.location.line.get()) // {line num} 
                + 3 // " | "
                + self.err.location.col // {source line} partial
            ),
        )
    }
}

// length of the number if formatted in decimal
#[inline]
fn decimal_length(mut n: usize) -> usize {
    let mut len = 1;
    while n >= 10 {
        n /= 10;
        len += 1;
    }
    len
}

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
