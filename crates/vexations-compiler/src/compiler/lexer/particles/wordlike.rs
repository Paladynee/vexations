use core::hint::assert_unchecked;
use core::hint::unreachable_unchecked;

use crate::compiler::lexer::Lexer;
use crate::compiler::lexer::error::LexerError;
use crate::compiler::lexer::error::LexerErrorKind;
use crate::frontend::token::TokenKind;

impl<'src> Lexer<'src> {
    /// Check for [`Lexer::is_at_end`] after this function returns.
    #[inline]
    pub fn wordlike(&mut self) {
        // current lexer state looks like:
        // ```_
        // [a, ?...
        //  ^ start
        //     ^ index
        // ```

        // callsite PER_CHAR_DISPATCH
        // we might be at source-end here, 3 more advances are valid
        // ```_
        // [a, b, c, \0, \0, \0]
        //            ^ index
        // ```

        while !self.is_at_end() {
            let c = unsafe { self.peek_unchecked() };
            if c.is_ascii_alphanumeric() | matches!(c, b'_') {
                unsafe { self.incr_unchecked() };
            } else {
                break;
            }
        }

        let identifier = self.make_identifier();
        unsafe { assert_unchecked(!identifier.is_empty()) };
        unsafe { self.trie_traverse(identifier) };
    }

    #[cold]
    pub fn trie_check_rest(
        &mut self, rest: &[u8], expected: &[u8], token: TokenKind,
    ) {
        if rest == expected {
            self.push_token(token);
        }
    }

    #[cold]
    pub fn trie_check_rest_ident(
        &mut self, rest: &[u8], expected: &[u8], token: TokenKind,
        identifier: &'src str,
    ) {
        if rest == expected {
            self.push_token_with_ident(token, identifier);
        }
    }

    #[inline(never)]
    pub unsafe fn trie_traverse(&mut self, identifier: &'src str) {
        let &[a, ref rest @ ..] = identifier.as_bytes() else {
            unsafe { unreachable_unchecked() };
        };

        'trie: {
            match a {
                // anymut
                b'a' =>
                    return self.trie_check_rest(
                        rest,
                        b"nymut",
                        TokenKind::KwAnymut,
                    ),
                // break
                b'b' =>
                    return self.trie_check_rest(
                        rest,
                        b"reak",
                        TokenKind::KwBreak,
                    ),
                // cast, compiletime, const, continue
                b'c' => {
                    let &[b, ref rest @ ..] = rest else {
                        break 'trie;
                    };
                    match b {
                        // cast
                        b'a' =>
                            return self.trie_check_rest(
                                rest,
                                b"st",
                                TokenKind::KwCast,
                            ),
                        // compiletime, const, continue,
                        b'o' => {
                            if !(5..=11).contains(&identifier.len()) {
                                break 'trie;
                            }
                            let &[c, ref rest @ ..] = rest else {
                                break 'trie;
                            };
                            match c {
                                // compiletime
                                b'm' =>
                                    return self.trie_check_rest(
                                        rest,
                                        b"piletime",
                                        TokenKind::KwCompiletime,
                                    ),
                                // const, continue
                                b'n' => {
                                    if !(5..=8).contains(&identifier.len()) {
                                        break 'trie;
                                    }
                                    let &[d, ref rest @ ..] = rest else {
                                        break 'trie;
                                    };
                                    match d {
                                        // const
                                        b's' =>
                                            return self.trie_check_rest(
                                                rest,
                                                b"t",
                                                TokenKind::KwConst,
                                            ),
                                        // continue
                                        b't' =>
                                            return self.trie_check_rest(
                                                rest,
                                                b"inue",
                                                TokenKind::KwContinue,
                                            ),
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                // else, enum, extern
                b'e' => {
                    let &[b, ref rest @ ..] = rest else {
                        break 'trie;
                    };
                    match b {
                        // else
                        b'l' =>
                            return self.trie_check_rest(
                                rest,
                                b"se",
                                TokenKind::KwElse,
                            ),
                        // enum
                        b'n' =>
                            return self.trie_check_rest(
                                rest,
                                b"um",
                                TokenKind::KwAdtEnum,
                            ),
                        // extern
                        b'x' =>
                            return self.trie_check_rest(
                                rest,
                                b"tern",
                                TokenKind::KwExtern,
                            ),
                        _ => {}
                    }
                }

                // fn, for, false
                b'f' => {
                    let &[b, ref rest @ ..] = rest else {
                        break 'trie;
                    };
                    match b {
                        // fn
                        b'n' =>
                            return self.trie_check_rest(
                                rest,
                                b"",
                                TokenKind::KwFn,
                            ),
                        // for
                        b'o' =>
                            return self.trie_check_rest(
                                rest,
                                b"r",
                                TokenKind::KwFor,
                            ),
                        // false
                        b'a' =>
                            return self.trie_check_rest_ident(
                                rest,
                                b"lse",
                                TokenKind::LitBool,
                                identifier,
                            ),
                        _ => {}
                    }
                }

                // if
                b'i' =>
                    return self.trie_check_rest(rest, b"f", TokenKind::KwIf),

                // let, loop
                b'l' => {
                    let &[b, ref rest @ ..] = rest else {
                        break 'trie;
                    };
                    match b {
                        // let
                        b'e' =>
                            return self.trie_check_rest(
                                rest,
                                b"t",
                                TokenKind::KwLet,
                            ),
                        // loop
                        b'o' =>
                            return self.trie_check_rest(
                                rest,
                                b"op",
                                TokenKind::KwLoop,
                            ),
                        _ => {}
                    }
                }

                // mut
                b'm' =>
                    return self.trie_check_rest(rest, b"ut", TokenKind::KwMut),

                // return, runtime
                b'r' => {
                    let &[b, ref rest @ ..] = rest else {
                        break 'trie;
                    };
                    match b {
                        // return
                        b'e' =>
                            return self.trie_check_rest(
                                rest,
                                b"turn",
                                TokenKind::KwReturn,
                            ),
                        // runtime
                        b'u' =>
                            return self.trie_check_rest(
                                rest,
                                b"ntime",
                                TokenKind::KwRuntime,
                            ),
                        _ => {}
                    }
                }

                // static, struct
                b's' => {
                    let &[b, c, ref rest @ ..] = rest else {
                        break 'trie;
                    };

                    if b != b't' {
                        break 'trie;
                    }

                    match c {
                        // static
                        b'a' =>
                            return self.trie_check_rest(
                                rest,
                                b"tic",
                                TokenKind::KwStatic,
                            ),
                        // struct
                        b'r' =>
                            return self.trie_check_rest(
                                rest,
                                b"uct",
                                TokenKind::KwAdtStruct,
                            ),
                        _ => {}
                    }
                }

                // type, true
                b't' => {
                    let &[b, ref rest @ ..] = rest else {
                        break 'trie;
                    };
                    match b {
                        // type
                        b'y' =>
                            return self.trie_check_rest(
                                rest,
                                b"pe",
                                TokenKind::KwType,
                            ),
                        // true
                        b'r' =>
                            return self.trie_check_rest_ident(
                                rest,
                                b"ue",
                                TokenKind::LitBool,
                                identifier,
                            ),
                        _ => {}
                    }
                }

                // union
                b'u' =>
                    return self.trie_check_rest(
                        rest,
                        b"nion",
                        TokenKind::KwAdtUnion,
                    ),

                // while
                b'w' =>
                    return self.trie_check_rest(
                        rest,
                        b"hile",
                        TokenKind::KwWhile,
                    ),
                // endof keywords
                _ => {}
            };
        }

        // just an identifier
        self.push_token_with_ident(TokenKind::LitIdentifier, identifier);
    }
}
