use core::fmt;
use core::fmt::Display;
use std::hint::assert_unchecked;
use std::hint::unlikely;
use std::marker::PhantomData;

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
  base: *const u8,
  limit: *const u8,
  start: *const u8,
  cursor: *const u8,
  tokens: &'b mut Vec<TokenKind>,
  errors: &'c mut Vec<LexerError>,
  idents: &'d mut Vec<&'src str>,
  _phantom: PhantomData<&'a VexationsSource<'src>>,
}

impl<'a, 'b, 'c, 'd, 'src> Lexer<'a, 'b, 'c, 'd, 'src> {
  #[inline]
  pub const fn new(
    src: &'a VexationsSource<'src>, tokens: &'b mut Vec<TokenKind>,
    errors: &'c mut Vec<LexerError>, idents: &'d mut Vec<&'src str>,
  ) -> Self {
    Lexer {
      base: src.base_ptr(),
      limit: src.end_ptr(),
      start: src.base_ptr(),
      cursor: src.base_ptr(),
      tokens,
      errors,
      idents,
      _phantom: PhantomData,
    }
  }

  #[inline(always)]
  pub fn is_at_end(&self) -> bool {
    unlikely(self.cursor >= self.limit)
  }

  #[inline(always)]
  pub unsafe fn assert_pointers_within_bounds(&self) {
    unsafe {
      assert_unchecked(self.cursor >= self.base);
      assert_unchecked(self.cursor <= self.limit);
    }
  }

  #[inline]
  #[rustfmt::skip]
  pub fn lex_all(&mut self) {
    macro_rules! match1 {
      ($Byte:literal) => {{
        self.assert_pointers_within_bounds();
        if *self.cursor != $Byte {
          false
        } else {
          self.cursor = self.cursor.add(1);
          self.assert_pointers_within_bounds();
          true
        }
      }};
    }

    let mut i: u32 = 0;
    if !self.is_at_end() {
      self.tokens.reserve(128);
      self.errors.reserve(128);
      self.idents.reserve(128);
    }

    while !self.is_at_end() {
      if i % 128 == 127 {
        self.tokens.reserve(128);
        self.errors.reserve(128);
        self.idents.reserve(128);
        i = 0;
      } else {
        i += 1;
      }

      // skip whitespace
      'whitespace: while !self.is_at_end() {
        unsafe { self.assert_pointers_within_bounds() };
        let c = unsafe { *self.cursor };
        match c {
          b' ' | b'\t' | b'\r' | b'\n' => {
            self.cursor = unsafe { self.cursor.add(1) };
            continue 'whitespace;
          }
          _ => break 'whitespace,
        }
      }
      // we may be at end here
      if self.is_at_end() {
        return;
      }

      self.start = self.cursor;
      unsafe { self.assert_pointers_within_bounds() };

      let c = unsafe { *self.cursor };
      self.cursor = unsafe { self.cursor.add(1) };

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
          _ => continue,
        });
      }
    }
  }
}

pub trait PushUnchecked<T> {
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
