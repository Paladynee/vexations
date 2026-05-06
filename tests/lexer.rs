use vexations_compiler::frontend::token::TokenKind as TK;

mod math_expressions {
    use std::panic::Location;

    use vexations_compiler::compiler::lexer;
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
}
