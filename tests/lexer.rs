use vexations_compiler::frontend::token::TokenKind as TK;
use vexations_generator::lexer_test_generator::LexerTestGenerator;

mod math_expressions {
    use core::panic::Location;

    use vexations_compiler::compiler::lexer::lex;
    use vexations_compiler::compiler::lexer::{
        self,
    };
    use vexations_compiler::frontend::source::VexationsSource;
    use vexations_compiler::frontend::token::TokenKind;

    use super::*;

    #[track_caller]
    fn assert_lex_tokens(src: &str, expected_kinds: &[TK]) {
        let location = Location::caller();

        let mut bytes = src.as_bytes().to_vec();
        bytes.extend_from_slice(&[0; 3]);
        let source = VexationsSource::try_from_bytes(&bytes).unwrap();
        let (tokens, _, errors) = lexer::lex(source.clone());

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
        let (tokens, idents, errors) = lexer::lex(source.clone());

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

    #[test]
    fn pregen_test() {
        const AMOUNT_TOKENS_TEST: usize = 100000;

        let (mut bytes, expected_tokens) = {
            let mut generator =
                LexerTestGenerator::new(AMOUNT_TOKENS_TEST, Some(0));
            let mut out_source = Vec::with_capacity(AMOUNT_TOKENS_TEST * 4);
            let mut expected_tokens = Vec::with_capacity(AMOUNT_TOKENS_TEST);
            while let Some((ws, kind, span)) = generator.next_span() {
                if let Some(whitespace) = ws {
                    out_source.extend_from_slice(whitespace.as_bytes());
                }
                out_source.extend_from_slice(span.as_bytes());
                expected_tokens.push(kind);
            }
            (out_source, expected_tokens)
        };
        bytes.extend_from_slice(&[0; 3]);
        let src = VexationsSource::try_from_bytes(&bytes).unwrap();
        let (toks, idents, errs) = lex(src.clone());
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
            let expected = expected_tokens[i];

            if *tok != expected {
                let mut out_file_path = std::env::temp_dir();
                out_file_path.push("test_fail.vxa");
                std::fs::write(&out_file_path, src.source()).unwrap();
                eprintln!(
                    "test fail source written to {}",
                    out_file_path.display()
                );

                panic!(
                    "TEST ERROR: token mismatch at token index {}\nExpected: {:?}\nActual: {:?}\nIdentifier: {:?}",
                    i, expected, tok, ident
                );
            }
        }
    }
}
