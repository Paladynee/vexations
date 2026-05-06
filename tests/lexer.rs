use vexations::middle::token::TokenKind as TK;

mod math_expressions {
    use std::panic::Location;

    use vexations::compiler::lexer;
    use vexations::middle::source::VexationsSource;

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
    fn simple_addition() {
        assert_lex_tokens("1 + 3", &[
            TK::LitInteger,
            TK::PuncPlus,
            TK::LitInteger,
        ]);
    }

    #[test]
    fn simple_multiplication() {
        assert_lex_tokens("3 * 9", &[
            TK::LitInteger,
            TK::PuncStar,
            TK::LitInteger,
        ]);
    }

    #[test]
    fn addition_and_multiplication() {
        assert_lex_tokens("1 + 3 * 9", &[
            TK::LitInteger,
            TK::PuncPlus,
            TK::LitInteger,
            TK::PuncStar,
            TK::LitInteger,
        ]);
    }

    #[test]
    fn subtraction() {
        assert_lex_tokens("10 - 5", &[
            TK::LitInteger,
            TK::PuncMinus,
            TK::LitInteger,
        ]);
    }

    #[test]
    fn division() {
        assert_lex_tokens("20 / 4", &[
            TK::LitInteger,
            TK::PuncSlash,
            TK::LitInteger,
        ]);
    }

    #[test]
    fn modulo() {
        assert_lex_tokens("10 % 3", &[
            TK::LitInteger,
            TK::PuncModulo,
            TK::LitInteger,
        ]);
    }

    #[test]
    fn parenthesized_expression() {
        assert_lex_tokens("(1 + 3) * 9", &[
            TK::IndentLParen,
            TK::LitInteger,
            TK::PuncPlus,
            TK::LitInteger,
            TK::IndentRParen,
            TK::PuncStar,
            TK::LitInteger,
        ]);
    }

    #[test]
    fn float_arithmetic() {
        assert_lex_tokens("3.14 + 2.86", &[
            TK::LitFloat,
            TK::PuncPlus,
            TK::LitFloat,
        ]);
    }

    #[test]
    fn negative_number() {
        assert_lex_tokens("-5 + 3", &[
            TK::PuncMinus,
            TK::LitInteger,
            TK::PuncPlus,
            TK::LitInteger,
        ]);
    }

    #[test]
    fn complex_expression() {
        assert_lex_tokens("2 * (3 + 4) - 5 / 2", &[
            TK::LitInteger,
            TK::PuncStar,
            TK::IndentLParen,
            TK::LitInteger,
            TK::PuncPlus,
            TK::LitInteger,
            TK::IndentRParen,
            TK::PuncMinus,
            TK::LitInteger,
            TK::PuncSlash,
            TK::LitInteger,
        ]);
    }
}
