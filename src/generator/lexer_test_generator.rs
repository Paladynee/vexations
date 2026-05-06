use std::io::Write;
use std::io::{
    self,
};

use voxell_rng::prelude::RngCoreExtension;
use voxell_rng::rng::XoRoShiRo128;
use voxell_rng::slice_methods::SelectorOneImmut;
use voxell_rng::slice_methods::SliceSelectRandomExt;
use voxell_rng::time_seeded::TimeSeededXoRoShiRo128Plus;

use crate::middle::token::TokenKind;

pub fn generate_lexer_test(
    out_w: &mut impl Write, n_tok: usize,
) -> io::Result<()> {
    let all_toks = TokenKind::ALL
        .iter()
        .filter(|&t| {
            *t == TokenKind::LitUninit
                || !t.is_identifier_extractable() && *t != TokenKind::MetaDummy
        })
        .copied()
        .collect::<Vec<_>>();

    let whitespace = [" ", "\n", "\r", "\t"];

    let mut rng: XoRoShiRo128 = TimeSeededXoRoShiRo128Plus::generate().unwrap();
    macro_rules! get_rand_tok {
        () => {
            all_toks
                .as_slice()
                .select_random(SelectorOneImmut, &mut rng)
                .unwrap()
        };
    }
    macro_rules! get_rand_whitespace {
        () => {
            whitespace
                .as_slice()
                .select_random(SelectorOneImmut, &mut rng)
                .unwrap()
        };
    }

    let mut last_token: Option<TokenKind> = None;

    static IDENT_START_ALPHABET: &[u8] =
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
    static IDENT_CONTINUE_ALPHABET: &[u8] =
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789";
    static DECIMAL_LIT_VALID_CONTINUE: &[u8] = b"0123456789";
    static HEX_LIT_VALID_CONTINUE: &[u8] = b"0123456789abcdefABCDEF";
    static OCTAL_LIT_VALID_CONTINUE: &[u8] = b"01234567";
    static BINARY_LIT_VALID_CONTINUE: &[u8] = b"01";
    // technically, any ascii value is valid inside a string other than " and \,
    // but for debuggability sake we'll only sample within printable
    // characters, but include " and \ so we can escape them later on in
    // generation
    static STRING_VALID_CONTINUE: &[u8] = b" !\"#$%&'()*+,-./\
        0123456789\
        :;<=>?@\
        ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        [\\]^_`\
        abcdefghijklmnopqrstuvwxyz\
        {|}~";
    const MAX_LIT_LEN: usize = 2;

    let mut scratch = vec![];
    for _ in 0..n_tok {
        scratch.clear();

        // decide if we're generating a literal or picking a token kind
        let is_literal = rng.next_usize() % 2 != 0;
        let current_token: TokenKind;

        if is_literal {
            match rng.next_usize() % 9 {
                0 => {
                    // valid identifier of random length
                    current_token = TokenKind::LitIdentifier;
                    let continue_len = rng.next_usize() % MAX_LIT_LEN;
                    let start = IDENT_START_ALPHABET
                        .select_random(SelectorOneImmut, &mut rng)
                        .unwrap();
                    scratch.push(*start);
                    for _ in 0..continue_len {
                        let next = IDENT_CONTINUE_ALPHABET
                            .select_random(SelectorOneImmut, &mut rng)
                            .unwrap();
                        scratch.push(*next);
                    }
                }
                1 => {
                    // valid integer number literal
                    current_token = TokenKind::LitInteger;
                    let start = DECIMAL_LIT_VALID_CONTINUE
                        .select_random(SelectorOneImmut, &mut rng)
                        .unwrap();
                    scratch.push(*start);
                    let continue_len = if *start == b'0' {
                        // we don't allow decimal integer literals to start with
                        // 0, but we must allow 0 itself to exist
                        0
                    } else {
                        rng.next_usize() % MAX_LIT_LEN
                    };
                    for _ in 0..continue_len {
                        let next = DECIMAL_LIT_VALID_CONTINUE
                            .select_random(SelectorOneImmut, &mut rng)
                            .unwrap();
                        scratch.push(*next);
                    }
                }
                2 => {
                    // valid float number literal
                    current_token = TokenKind::LitFloat;
                    let start = DECIMAL_LIT_VALID_CONTINUE
                        .select_random(SelectorOneImmut, &mut rng)
                        .unwrap();
                    scratch.push(*start);
                    let continue_len = if *start == b'0' {
                        // we don't allow float literals to start with 0, but we
                        // must allow 0 integral part itself to exist
                        0
                    } else {
                        rng.next_usize() % MAX_LIT_LEN
                    };
                    for _ in 0..continue_len {
                        let next = DECIMAL_LIT_VALID_CONTINUE
                            .select_random(SelectorOneImmut, &mut rng)
                            .unwrap();
                        scratch.push(*next);
                    }
                    scratch.push(b'.');
                    // +1 because there must be at least 1 digit in the
                    // fractional part
                    let continue_len = rng.next_usize() % MAX_LIT_LEN + 1;
                    for _ in 0..continue_len {
                        let next = DECIMAL_LIT_VALID_CONTINUE
                            .select_random(SelectorOneImmut, &mut rng)
                            .unwrap();
                        scratch.push(*next);
                    }
                }
                3 => {
                    // valid hex number literal 0x
                    current_token = TokenKind::LitInteger;
                    scratch.extend_from_slice(b"0x");
                    // +1 because there must be at least 1 digit after 0x
                    let continue_len = rng.next_usize() % MAX_LIT_LEN + 1;
                    for _ in 0..continue_len {
                        let next = HEX_LIT_VALID_CONTINUE
                            .select_random(SelectorOneImmut, &mut rng)
                            .unwrap();
                        scratch.push(*next);
                    }
                }
                4 => {
                    // valid octal number literal 0o
                    current_token = TokenKind::LitInteger;
                    scratch.extend_from_slice(b"0o");
                    // +1 because there must be at least 1 digit after 0o
                    let continue_len = rng.next_usize() % MAX_LIT_LEN + 1;
                    for _ in 0..continue_len {
                        let next = OCTAL_LIT_VALID_CONTINUE
                            .select_random(SelectorOneImmut, &mut rng)
                            .unwrap();
                        scratch.push(*next);
                    }
                }
                5 => {
                    // valid binary number literal 0b
                    current_token = TokenKind::LitInteger;
                    scratch.extend_from_slice(b"0b");
                    // +1 because there must be at least 1 digit after 0b
                    let continue_len = rng.next_usize() % MAX_LIT_LEN + 1;
                    for _ in 0..continue_len {
                        let next = BINARY_LIT_VALID_CONTINUE
                            .select_random(SelectorOneImmut, &mut rng)
                            .unwrap();
                        scratch.push(*next);
                    }
                }
                6 => {
                    // valid string literal
                    current_token = TokenKind::LitStr;
                    scratch.push(b'"');
                    let continue_len = rng.next_usize() % MAX_LIT_LEN;
                    for _ in 0..continue_len {
                        let next = STRING_VALID_CONTINUE
                            .select_random(SelectorOneImmut, &mut rng)
                            .unwrap();
                        if *next == b'\\' || *next == b'"' {
                            scratch.push(b'\\');
                        }
                        scratch.push(*next);
                    }
                    scratch.push(b'"');
                }
                7 => {
                    // valid char literal
                    current_token = TokenKind::LitChar;
                    scratch.push(b'\'');
                    let next = STRING_VALID_CONTINUE
                        .select_random(SelectorOneImmut, &mut rng)
                        .unwrap();
                    if *next == b'\\' || *next == b'\'' {
                        scratch.push(b'\\');
                    }
                    scratch.push(*next);
                    scratch.push(b'\'');
                }
                8 => {
                    // valid boolean literal
                    current_token = TokenKind::LitBool;
                    if rng.next_u8() % 2 == 0 {
                        scratch.extend_from_slice(b"true");
                    } else {
                        scratch.extend_from_slice(b"false");
                    }
                }
                _ => unreachable!(),
            };
        } else {
            current_token = *get_rand_tok!();
        }

        if let Some(prev) = last_token {
            if !TokenKind::can_safely_follow(prev, current_token) {
                // we must emit whitespace for grammar validity
                write!(out_w, "{}", get_rand_whitespace!())?;
            } else if rng.next_u8() % 2 == 0 {
                // optional stochastic whitespace for coverage
                write!(out_w, "{}", get_rand_whitespace!())?;
            }
        }

        if is_literal {
            out_w.write_all(&scratch)?;
        } else {
            write!(out_w, "{}", current_token.source_repr())?;
        }

        last_token = Some(current_token);
    }

    Ok(())
}
