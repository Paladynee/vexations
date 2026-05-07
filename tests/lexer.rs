#![feature(ptr_cast_slice)]
#![allow(unused)]
extern crate libc;

use vexations_compiler::frontend::token::TokenKind as TK;
use vexations_generator::lexer_test_generator::LexerTestGenerator;

mod math_expressions {
    use core::panic::Location;
    use std::alloc::Layout;

    use vexations_compiler::compiler::lexer;
    use vexations_compiler::compiler::lexer::error::LexerError;
    use vexations_compiler::frontend::source::VexationsSource;
    use vexations_compiler::frontend::token::TokenKind;

    use super::*;

    #[track_caller]
    fn assert_lex_tokens(src: &str, expected_kinds: &[TK]) {
        let location = Location::caller();

        let mut bytes = src.as_bytes().to_vec();
        bytes.extend_from_slice(&[0; 3]);
        let source = VexationsSource::try_from_bytes(&bytes).unwrap();
        let (tokens, spans, idents, errors) =
            lexer::lex(source.clone()).finalize();

        if !errors.is_empty() {
            eprintln!(
                "unexpected lexing errors in test {}\nSource: {}\nErrors:",
                location, src
            );
            for error in errors {
                eprintln!("{}", error.display(source.clone()));
            }
        }

        assert_eq!(
            tokens, expected_kinds,
            "Lexer mismatch\nSource: {}\nExpected: {:?}\nActual: {:?}\nAt test {}",
            src, expected_kinds, tokens, location
        );
    }

    #[track_caller]
    fn assert_lex_identifiable(
        src: &str, expected_kinds: &[TK], expected_idents: &[&str],
    ) {
        let location = Location::caller();

        let mut bytes = src.as_bytes().to_vec();
        bytes.extend_from_slice(&[0; 3]);
        let source = VexationsSource::try_from_bytes(&bytes).unwrap();
        let (toks, spans, idents, errs) = lexer::lex(source.clone()).finalize();

        if !errs.is_empty() {
            eprintln!(
                "unexpected lexing errors in test {}\nSource: {}\nErrors:",
                location, src
            );
            for error in errs {
                eprintln!("{}", error.display(source.clone()));
            }
        }

        assert_eq!(
            toks, expected_kinds,
            "Lexer mismatch\nSource: {}\nExpected: {:?}\nActual: {:?}\nAt test {}",
            src, expected_kinds, toks, location
        );

        assert_eq!(
            idents, expected_idents,
            "Lexer identifier mismatch\nSource: {}\nExpected: {:?}\nActual: {:?}\nAt test {}",
            src, expected_idents, idents, location
        )
    }

    #[test]
    fn empty() {
        assert_lex_tokens("", &[]);
    }

    #[test]
    fn whitespace() {
        assert_lex_tokens(" \t\r\n", &[]);
    }

    #[test]
    fn delims() {
        assert_lex_tokens("()[]{}", &[
            TK::IndentLParen,
            TK::IndentRParen,
            TK::IndentLBracket,
            TK::IndentRBracket,
            TK::IndentLBrace,
            TK::IndentRBrace,
        ]);
    }

    #[test]
    fn operators() {
        assert_lex_tokens("+ - * / % = ^ & |", &[
            TK::PuncPlus,
            TK::PuncMinus,
            TK::PuncStar,
            TK::PuncSlash,
            TK::PuncModulo,
            TK::PuncEq,
            TK::PuncXor,
            TK::PuncAnd,
            TK::PuncOr,
        ]);
    }

    #[test]
    fn compound_operators() {
        assert_lex_tokens("+= -= *= /= %= ^= &= |=", &[
            TK::PuncPlusEq,
            TK::PuncMinusEq,
            TK::PuncStarEq,
            TK::PuncSlashEq,
            TK::PuncModuloEq,
            TK::PuncXorEq,
            TK::PuncAndEq,
            TK::PuncOrEq,
        ]);
    }

    #[test]
    fn comparison_operators() {
        assert_lex_tokens("== != < > <= >=", &[
            TK::PuncEqEq,
            TK::PuncBangEq,
            TK::PuncLt,
            TK::PuncGt,
            TK::PuncLtEq,
            TK::PuncGtEq,
        ]);
    }

    #[test]
    fn logical_operators() {
        assert_lex_tokens("&& || !", &[
            TK::PuncAndAnd,
            TK::PuncOrOr,
            TK::PuncBang,
        ]);
    }

    #[test]
    fn misc_punctuation() {
        assert_lex_tokens(", ; : :: .", &[
            TK::PuncComma,
            TK::PuncSemi,
            TK::PuncColon,
            TK::PuncColonColon,
            TK::PuncDot,
        ]);
    }

    #[test]
    fn shiftlike() {
        assert_lex_tokens("<< >> <<= >>=", &[
            TK::PuncShl,
            TK::PuncShr,
            TK::PuncShlEq,
            TK::PuncShrEq,
        ]);
    }

    #[test]
    fn arrow_right() {
        assert_lex_tokens("->", &[TK::PuncArrowRight]);
    }

    #[test]
    fn zero() {
        assert_lex_identifiable("0", &[TK::LitInteger], &["0"]);
    }

    #[test]
    fn one() {
        assert_lex_identifiable("1", &[TK::LitInteger], &["1"]);
    }

    #[test]
    fn big_integers() {
        assert_lex_identifiable(
            "10 130 934943",
            &[TK::LitInteger, TK::LitInteger, TK::LitInteger],
            &["10", "130", "934943"],
        )
    }

    #[test]
    fn float_zero() {
        assert_lex_identifiable("0.0", &[TK::LitFloat], &["0.0"])
    }

    #[test]
    fn float_one() {
        assert_lex_identifiable("1.0", &[TK::LitFloat], &["1.0"])
    }

    #[test]
    fn big_floats() {
        assert_lex_identifiable(
            "3.14 0.001 123456.789",
            &[TK::LitFloat, TK::LitFloat, TK::LitFloat],
            &["3.14", "0.001", "123456.789"],
        )
    }

    #[rustfmt::skip]
    #[test]
    fn freeform_mathematical_code() {
        assert_lex_identifiable("
        1 + 1 >= 0;
        2[10] == 12;
        1 << 10 == 1024;
        3.14 * 2.0 == 6.28;
        0[0] != 0.0;
",
        &[
            TK::LitInteger, TK::PuncPlus, TK::LitInteger, TK::PuncGtEq, TK::LitInteger, TK::PuncSemi,
            TK::LitInteger, TK::IndentLBracket, TK::LitInteger, TK::IndentRBracket, TK::PuncEqEq, TK::LitInteger, TK::PuncSemi,
            TK::LitInteger, TK::PuncShl, TK::LitInteger, TK::PuncEqEq, TK::LitInteger, TK::PuncSemi,
            TK::LitFloat, TK::PuncStar, TK::LitFloat, TK::PuncEqEq, TK::LitFloat, TK::PuncSemi,
            TK::LitInteger, TK::IndentLBracket, TK::LitInteger, TK::IndentRBracket, TK::PuncBangEq, TK::LitFloat, TK::PuncSemi,
        ],
        &[
            "1", "1", "0",
            "2", "10", "12",
            "1", "10", "1024",
            "3.14", "2.0", "6.28",
            "0", "0", "0.0",
        ]);

    }

    // #[test]
    // fn safe_follow() {
    //     use std::io::Write;
    //     let mut scratch: Vec<u8> = Vec::new();
    //     let mut fails = vec![];
    //     for a in TokenKind::ALL.iter().copied() {
    //         for b in TokenKind::ALL.iter().copied() {
    //             if !TokenKind::can_safely_follow(a, b) {
    //                 scratch.clear();
    //                 let _ = write!(
    //                     &mut scratch,
    //                     "{}{}",
    //                     a.source_repr(),
    //                     b.source_repr()
    //                 );
    //                 scratch.extend_from_slice(&[0, 0, 0]);
    //                 let source =
    //                     VexationsSource::try_from_bytes(&scratch).unwrap();

    //                 let (toks, ..) = lexer::lex(source).finalize();
    //                 if toks == [a, b] {
    //                     fails.push((a, b));
    //                 }
    //             }
    //         }
    //     }

    //     if !fails.is_empty() {
    //         panic!("{:#?}", fails);
    //     }
    // }

    const TEST_SEED: u64 = 1395;

    fn validate_gen_output(
        src: VexationsSource, toks: &[TokenKind], spans: &[usize],
        idents: &[&str], errs: Vec<LexerError>, expected: &[TokenKind],
    ) {
        if !errs.is_empty() {
            eprintln!("TEST ERROR: lexing errors encountered:");
            for err in errs {
                eprintln!("{}", err.display(src.clone()));
            }
            panic!("TEST ERROR: lexing errors encountered")
        }
        let mut ident_i = 0;
        for i in 0..toks.len() {
            let tok = &toks[i];
            let ident: Option<&str> = if tok.is_identifier_extractable() {
                let ret = idents[ident_i];
                ident_i += 1;
                Some(ret)
            } else {
                None
            };
            let expected = expected[i];
            let span = spans[i];
            if *tok != expected {
                let mut out_file_path = std::env::temp_dir();
                out_file_path.push("test_fail.vxa");
                std::fs::write(&out_file_path, src.source()).unwrap();
                eprintln!(
                    "test fail source written to {}",
                    out_file_path.display()
                );

                let Some(src_rest) = src.source().get(span..) else {
                    panic!("shouldn't happen");
                };

                let src_line = src_rest
                    .split_once(['\r', '\n'])
                    .map(|(line, _)| line)
                    .unwrap_or(src_rest);

                panic!(
                    "Token mismatch at {i}th token with start {span}\nExpected: {:?}\nGot: {:?}{}\nSource: {:?}",
                    expected,
                    tok,
                    if let Some(ident) = ident {
                        format!("\nIdentifier: {:?}", ident)
                    } else {
                        "".to_string()
                    },
                    src_line,
                )
            };
        }
    }

    const WINDOW_TEST_LENGTH: usize = 1000000;

    #[test]
    fn generated_token_window_test() {
        let mut generator =
            LexerTestGenerator::new(WINDOW_TEST_LENGTH, Some(TEST_SEED));
        let mut out_source: Vec<u8> =
            Vec::with_capacity(WINDOW_TEST_LENGTH * 4);
        let mut expected_tokens: Vec<TokenKind> =
            Vec::with_capacity(WINDOW_TEST_LENGTH);
        let mut i = 0;
        while let Some((ws, kind, span)) = generator.next_span() {
            i += 1;
            if let Some(whitespace) = ws {
                out_source.extend_from_slice(whitespace.as_bytes());
            }
            out_source.extend_from_slice(span.as_bytes());
            expected_tokens.push(kind);
            out_source.extend_from_slice(&[0; 3]);
            let aligned = vec_align_page_end(&out_source);
            let src =
                VexationsSource::try_from_bytes(aligned.as_slice()).unwrap();
            let (toks, spans, idents, errs) =
                lexer::lex(src.clone()).finalize();
            eprintln!(
                "testing output of byte length {}, token length {}",
                src.source_len(),
                i
            );
            validate_gen_output(
                src,
                &toks,
                &spans,
                &idents,
                errs,
                &expected_tokens,
            );
            out_source.pop();
            out_source.pop();
            out_source.pop();
        }
    }

    struct PsuedoVec {
        alloc_ptr: *mut u8,
        data_ptr: *mut u8,
        alloc_len: usize,
        data_len: usize,
    }

    impl PsuedoVec {
        pub fn as_slice(&self) -> &[u8] {
            unsafe { &*self.data_ptr.cast_slice(self.data_len) }
        }
    }

    impl Drop for PsuedoVec {
        fn drop(&mut self) {
            unsafe {
                libc::mprotect(
                    self.alloc_ptr
                        .byte_add(self.alloc_len)
                        .byte_sub(4096)
                        .cast(),
                    4096,
                    libc::PROT_READ | libc::PROT_WRITE,
                );
                std::alloc::dealloc(
                    self.alloc_ptr,
                    Layout::from_size_align_unchecked(self.alloc_len, 4096),
                )
            };
        }
    }

    fn vec_align_page_end(data: &[u8]) -> PsuedoVec {
        use std::alloc::*;

        unsafe {
            let data_ptr = data.as_ptr();
            let data_len = data.len();
            let next_mult = data_len.next_multiple_of(4096) + 4096;
            let lo = Layout::from_size_align_unchecked(next_mult, 4096);
            let ptr = alloc(lo);

            if ptr.is_null() {
                handle_alloc_error(lo);
            }
            // [0, 0, 0, 0] len 4
            // [1, 1, 1] len 3
            // [0, 1, 1, 1] offset = a.len - b.len
            let offset = next_mult - (data_len + 4096);
            let write_start = ptr.byte_add(offset);
            write_start.copy_from_nonoverlapping(data_ptr, data_len);

            let last_page = ptr.byte_add(next_mult).byte_sub(4096);
            // unmap last page
            libc::mprotect(last_page.cast(), 4096, libc::PROT_NONE);

            PsuedoVec {
                alloc_ptr: ptr,
                alloc_len: next_mult,
                data_ptr: write_start,
                data_len,
            }
        }
    }

    #[test]
    fn generated_test() {
        const AMOUNT_TOKENS_TEST: usize = 10000000;
        let (bytes, expected) = {
            let mut generator = LexerTestGenerator::new(
                AMOUNT_TOKENS_TEST,
                Some(TEST_SEED + 1),
            );
            let mut out_source = Vec::with_capacity(AMOUNT_TOKENS_TEST * 4);
            let mut expected_tokens = Vec::with_capacity(AMOUNT_TOKENS_TEST);
            while let Some((ws, kind, span)) = generator.next_span() {
                if let Some(whitespace) = ws {
                    out_source.extend_from_slice(whitespace.as_bytes());
                }
                out_source.extend_from_slice(span.as_bytes());
                expected_tokens.push(kind);
            }
            out_source.extend_from_slice(&[0; 3]);
            (out_source, expected_tokens)
        };
        let src = VexationsSource::try_from_bytes(&bytes).unwrap();
        let (toks, spans, idents, errs) = lexer::lex(src.clone()).finalize();
        validate_gen_output(src, &toks, &spans, &idents, errs, &expected);
    }
}
