use core::hint::assert_unchecked;
use core::hint::unreachable_unchecked;

use crate::compiler::lexer::Lexer;
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
    #[inline(never)]
    pub fn trie_check_rest(
        &mut self, rest: &[u8], expected: &[u8], token: TokenKind,
        identifier: &'src str,
    ) {
        if *rest == *expected {
            self.push_token(token);
        } else {
            self.push_token_with_ident(TokenKind::LitIdentifier, identifier);
        }
    }

    #[cold]
    #[inline(never)]
    pub fn trie_check_rest_ident(
        &mut self, rest: &[u8], expected: &[u8], token: TokenKind,
        identifier: &'src str,
    ) {
        self.push_token_with_ident(
            if *rest == *expected {
                token
            } else {
                TokenKind::LitIdentifier
            },
            identifier,
        );
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
                        identifier,
                    ),
                // break
                b'b' =>
                    return self.trie_check_rest(
                        rest,
                        b"reak",
                        TokenKind::KwBreak,
                        identifier,
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
                                identifier,
                            ),
                        // compiletime, const, continue,
                        b'o' => {
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
                                        identifier,
                                    ),
                                // const, continue
                                b'n' => {
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
                                                identifier,
                                            ),
                                        // continue
                                        b't' =>
                                            return self.trie_check_rest(
                                                rest,
                                                b"inue",
                                                TokenKind::KwContinue,
                                                identifier,
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
                                identifier,
                            ),
                        // enum
                        b'n' =>
                            return self.trie_check_rest(
                                rest,
                                b"um",
                                TokenKind::KwAdtEnum,
                                identifier,
                            ),
                        // extern
                        b'x' =>
                            return self.trie_check_rest(
                                rest,
                                b"tern",
                                TokenKind::KwExtern,
                                identifier,
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
                                identifier,
                            ),
                        // for
                        b'o' =>
                            return self.trie_check_rest(
                                rest,
                                b"r",
                                TokenKind::KwFor,
                                identifier,
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
                    return self.trie_check_rest(
                        rest,
                        b"f",
                        TokenKind::KwIf,
                        identifier,
                    ),

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
                                identifier,
                            ),
                        // loop
                        b'o' =>
                            return self.trie_check_rest(
                                rest,
                                b"op",
                                TokenKind::KwLoop,
                                identifier,
                            ),
                        _ => {}
                    }
                }

                // mut
                b'm' =>
                    return self.trie_check_rest(
                        rest,
                        b"ut",
                        TokenKind::KwMut,
                        identifier,
                    ),

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
                                identifier,
                            ),
                        // runtime
                        b'u' =>
                            return self.trie_check_rest(
                                rest,
                                b"ntime",
                                TokenKind::KwRuntime,
                                identifier,
                            ),
                        _ => {}
                    }
                }

                // static, struct
                b's' => {
                    let &[b't', c, ref rest @ ..] = rest else {
                        break 'trie;
                    };

                    match c {
                        // static
                        b'a' =>
                            return self.trie_check_rest(
                                rest,
                                b"tic",
                                TokenKind::KwStatic,
                                identifier,
                            ),
                        // struct
                        b'r' =>
                            return self.trie_check_rest(
                                rest,
                                b"uct",
                                TokenKind::KwAdtStruct,
                                identifier,
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
                                identifier,
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

                // union, uninit
                b'u' => {
                    let &[b'n', ref rest @ ..] = rest else {
                        break 'trie;
                    };
                    let &[b'i', ref rest @ ..] = rest else {
                        break 'trie;
                    };
                    let &[b, ref rest @ ..] = rest else {
                        break 'trie;
                    };
                    match b {
                        // union
                        b'o' =>
                            return self.trie_check_rest(
                                rest,
                                b"n",
                                TokenKind::KwAdtUnion,
                                identifier,
                            ),
                        // uninit
                        b'n' =>
                            return self.trie_check_rest_ident(
                                rest,
                                b"it",
                                TokenKind::LitUninit,
                                identifier,
                            ),
                        _ => {}
                    }
                }

                // while
                b'w' =>
                    return self.trie_check_rest(
                        rest,
                        b"hile",
                        TokenKind::KwWhile,
                        identifier,
                    ),
                // endof keywords
                _ => {}
            };
        }

        // just an identifier
        self.push_token_with_ident(TokenKind::LitIdentifier, identifier);
    }
}
