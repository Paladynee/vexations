use core::fmt;
use core::fmt::Display;
use core::hint::assert_unchecked;
use core::hint::cold_path;
use core::hint::unlikely;
use core::hint::unreachable_unchecked;
use core::str;
use std::num::NonZeroUsize;

use crate::source::LineCol;
use crate::source::VexationsSource;
use crate::token::TokenKind;

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
                write!(f, "unclosed block comment"),
            LexerErrorKind::UnknownCharacter(c) =>
                write!(f, "unknown character: '{c:#x}'"),
        }
    }
}

impl core::error::Error for LexerError {}

#[allow(unused)]
pub fn lex<'src>(
    src: &VexationsSource<'src>, tokens: &mut Vec<TokenKind>,
    errors: &mut Vec<LexerError>, idents: &mut Vec<&'src str>,
) {
    let mut lexer = Lexer::new(src, tokens, errors, idents);
    lexer.lex_all();
}

pub struct Lexer<'a, 'b, 'c, 'd, 'src> {
    src: &'a VexationsSource<'src>,
    tokens: &'b mut Vec<TokenKind>,
    errors: &'c mut Vec<LexerError>,
    idents: &'d mut Vec<&'src str>,
}

impl<'a, 'b, 'c, 'd, 'src> Lexer<'a, 'b, 'c, 'd, 'src> {
    #[inline]
    pub const fn new(
        src: &'a VexationsSource<'src>, tokens: &'b mut Vec<TokenKind>,
        errors: &'c mut Vec<LexerError>, idents: &'d mut Vec<&'src str>,
    ) -> Self {
        Lexer {
            src,
            tokens,
            errors,
            idents,
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
    #[rustfmt::skip]
    pub fn lex_all(&mut self) {
        let base: *const u8 = self.src.base_ptr();
        let limit: *const u8 = self.src.end_ptr();
        let mut cursor: *const u8 = base;
        let mut start: *const u8;
        macro_rules! assert_pointers_within_bounds {
            () => {{
                assert_unchecked(cursor >= base);
                assert_unchecked(cursor <= limit);
            }};
        }
        macro_rules! len {
            () => {{ limit.byte_offset_from_unsigned(base) }};
        }
        macro_rules! is_at_end {
            () => {{ unlikely(cursor >= limit) }};
        }
        macro_rules! increment {
            () => {{
                cursor = cursor.add(1);
            }};
        }
        macro_rules! make_lit {
            () => {{
                let len: usize = cursor.byte_offset_from_unsigned(start);
                let slice = core::slice::from_raw_parts(start, len);
                str::from_utf8_unchecked(slice)
            }};
        }
        macro_rules! peek {
            () => {{
                assert_pointers_within_bounds!();
                *cursor
            }};
        }
        macro_rules! advance {
            () => {{
                let c = peek!();
                cursor = cursor.add(1);
                assert_pointers_within_bounds!();
                c
            }};
        }
        macro_rules! match1 {
            ($Byte:literal) => {{
                assert_pointers_within_bounds!();
                if *cursor != $Byte {
                    false
                } else {
                    cursor = cursor.add(1);
                    assert_pointers_within_bounds!();
                    true
                }
            }};
        }

        // let mut i: u32 = 0;
        // if !self.is_at_end() {
        //     self.tokens.reserve(128);
        //     self.errors.reserve(128);
        //     self.idents.reserve(128);
        // }
        self.tokens.reserve(self.src.len());
        self.errors.reserve(self.src.len());
        self.idents.reserve(self.src.len());

        while !is_at_end!() {
            // if i % 128 == 127 {
            //     self.tokens.reserve(128);
            //     self.errors.reserve(128);
            //     self.idents.reserve(128);
            //     i = 0;
            // } else {
            //     i += 1;
            // }

            // skip whitespace
            'whitespace: while !is_at_end!() {
                let c = unsafe { peek!() };
                match c {
                    b' ' | b'\t' | b'\r' | b'\n' => {
                        unsafe { increment!() };
                        continue 'whitespace;
                    }
                    _ => break 'whitespace,
                }
            }
            // we may be at end here
            if is_at_end!() {
                return;
            }

            start = cursor;
            let c = unsafe { advance!() };

            // we might be at end here

            unsafe {
                self.tokens.push_unchecked(match c {
                    b'(' => TokenKind::IndentLParen,
                    b')' => TokenKind::IndentRParen,
                    b'{' => TokenKind::IndentLBrace,
                    b'}' => TokenKind::IndentRBrace,
                    b'[' => TokenKind::IndentLBracket,
                    b']' => TokenKind::IndentRBracket,
                    b'.' => TokenKind::PuncDot,
                    b',' => TokenKind::PuncComma,
                    b';' => TokenKind::PuncSemi,
                    b':' => if match1!(b':') { TokenKind::PuncColonColon }
                        else { TokenKind::PuncColon },
                    b'-' => if match1!(b'>') { TokenKind::PuncArrowRight }
                        else { TokenKind::PuncMinus },
                    b'=' => if match1!(b'=') { TokenKind::PuncEqEq }
                        else { TokenKind::PuncEq },
                    b'!' => if match1!(b'=') { TokenKind::PuncBangEq }
                        else { TokenKind::PuncBang },
                    b'<' => if match1!(b'=') { TokenKind::PuncLtEq }
                        else if match1!(b'<') {
                            if match1!(b'=') { TokenKind::PuncShlEq }
                            else { TokenKind::PuncShl }
                        } else { TokenKind::PuncLt },
                    b'>' => if match1!(b'=') { TokenKind::PuncGtEq }
                        else if match1!(b'>') {
                            if match1!(b'=') { TokenKind::PuncShrEq }
                            else { TokenKind::PuncShr }
                        } else { TokenKind::PuncGt },
                    b'+' => if match1!(b'=') { TokenKind::PuncPlusEq }
                        else { TokenKind::PuncPlus },
                    b'*' => if match1!(b'=') { TokenKind::PuncStarEq }
                        else { TokenKind::PuncStar },
                    b'/' => if match1!(b'=') { TokenKind::PuncSlashEq }
                        else { TokenKind::PuncSlash },
                    b'%' => if match1!(b'=') { TokenKind::PuncModuloEq }
                        else { TokenKind::PuncModulo },
                    b'^' => if match1!(b'=') { TokenKind::PuncXorEq }
                        else { TokenKind::PuncXor },
                    b'&' => if match1!(b'&') { TokenKind::PuncAndAnd }
                        else if match1!(b'=') { TokenKind::PuncAndEq }
                        else { TokenKind::PuncAnd },
                    b'|' => if match1!(b'|') { TokenKind::PuncOrOr }
                        else if match1!(b'=') { TokenKind::PuncOrEq }
                        else { TokenKind::PuncOr },
                    b'\'' => {
                        // char lit
                        // '
                        //  ^
                        continue;
                    },
                    b'"' => {
                        // string lit
                        // "
                        //  ^
                        continue;
                    },
                    b'0' => {
                        // zero lit
                        let next = peek!();
                        match next {
                            b'x' => {
                                // hex lit
                                // 0x
                                //  ^
                                continue;
                            },
                            b'o' => {
                                // octal lit
                                // 0o
                                //  ^
                                continue;
                            },
                            b'b' => {
                                // binary literal
                                // 0b
                                //  ^
                                continue;
                            },
                            b'1'..=b'9' => {
                                // num lit
                                // 03
                                //  ^
                                cold_path(); // who writes a number literal like 0934?
                                continue;
                            },
                            _ => {
                                // zero
                                // 0
                                //  ^
                                self.tokens.push_unchecked(TokenKind::LitInteger);
                                self.idents.push_unchecked(make_lit!());
                                continue;
                            }
                        };
                    },
                    b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                        // identifier start
                        // f
                        //  ^
                        continue;
                    },
                    rest => {
                        cold_path();
                        let e = LexerError {
                            location:
                                Self::location(self.src.src,
                                    start.byte_offset_from_unsigned(base)),
                            kind: LexerErrorKind::UnknownCharacter(rest),
                        };
                        self.errors.push_unchecked(e);
                        continue;
                    },
                });
            }
        }

        if cfg!(feature = "save_memory") {
            self.tokens.shrink_to_fit();
            self.errors.shrink_to_fit();
            self.idents.shrink_to_fit();
        }
    }
}

pub trait PushUnchecked<T> {
    /// Pushes a value to the vector without checking capacity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the vector has enough capacity to hold the
    /// new value.
    unsafe fn push_unchecked(&mut self, value: T);
}

impl<T> PushUnchecked<T> for Vec<T> {
    #[inline(always)]
    unsafe fn push_unchecked(&mut self, value: T) {
        let len = self.len();
        unsafe {
            let ptr = self.as_mut_ptr().add(len);
            ptr.write(value);
            self.set_len(len.unchecked_add(1));
        }
    }
}
