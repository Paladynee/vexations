use vexations_compiler::frontend::token::TokenKind as TK;

static PREGEN_TEST: &str = include_str!("lexer_test.vxa");

mod math_expressions {
    use std::panic::Location;

    use vexations_compiler::compiler::lexer::lex;
    use vexations_compiler::compiler::lexer::{
        self,
    };
    use vexations_compiler::frontend::source::VexationsSource;

    use super::*;

    #[track_caller]
    fn assert_lex_tokens(src: &str, expected_kinds: &[TK]) {
        let location = Location::caller();

        let mut bytes = src.as_bytes().to_vec();
        bytes.extend_from_slice(&[0; 3]);
        let source = VexationsSource::try_from_bytes(&bytes).unwrap();
        let (tokens, _, errors) = lexer::lex(source);

        if !errors.is_empty() {
            panic!(
                "unexpected lexing errors in test {}\nSource: {}\nErrors: {:#?}",
                location, src, errors
            );
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
        let (tokens, idents, errors) = lexer::lex(source);

        if !errors.is_empty() {
            panic!(
                "unexpected lexing errors in test {}\nSource: {}\nErrors: {:#?}",
                location, src, errors
            );
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
        let mut bytes = PREGEN_TEST.as_bytes().to_vec();
        bytes.extend_from_slice(&[0; 3]);
        let src = VexationsSource::try_from_bytes(&bytes).unwrap();
        let (toks, _, errs) = lex(src);
        assert_eq!(toks.len(), 10000);
        assert!(errs.is_empty(), "unexpected lexing errors: {:#?}", errs);
    }
}
