use crate::compiler::lexer::Lexer;
use crate::compiler::lexer::error::LexerErrorKind;
use crate::frontend::token::TokenKind as TK;

pub type PerCharHandler = for<'src> fn(&mut Lexer<'src>);

#[allow(unused_unsafe)]
pub static PER_CHAR_DISPATCHER: [PerCharHandler; 256] = {
    let mut handlers = [unknown_character as PerCharHandler; 256];

    // empty handler for nul byte
    handlers[0] = |_| {};

    macro_rules! h {
        ( raw $char:literal, $kind:ident ) => {
            handlers[$char as usize] = |l| l.push_token(TK::$kind);
        };
        ( range $start:literal..=$end:literal, $lexer:ident, $($body:tt)* ) => {{
            let mut c = $start;
            while c <= $end {
                h!(c, $lexer, $($body)*);
                c += 1;
            }
        }};
        ( $char:expr, $lexer:pat, $($body:tt)* ) => {
            handlers[$char as usize] = |$lexer| unsafe { $($body)* };
        };
    }

    h!(raw b'(', IndentLParen);
    h!(raw b')', IndentRParen);
    h!(raw b'{', IndentLBrace);
    h!(raw b'}', IndentRBrace);
    h!(raw b'[', IndentLBracket);
    h!(raw b']', IndentRBracket);
    h!(raw b'.', PuncDot);
    h!(raw b',', PuncComma);
    h!(raw b';', PuncSemi);

    h!(b':', l, {
        let token = if l.matches_unchecked(b':') {
            TK::PuncColonColon
        } else {
            TK::PuncColon
        };
        l.push_token(token)
    });
    h!(b'=', l, {
        let token = if l.matches_unchecked(b'=') {
            TK::PuncEqEq
        } else {
            TK::PuncEq
        };
        l.push_token(token)
    });
    h!(b'!', l, {
        let token = if l.matches_unchecked(b'=') {
            TK::PuncBangEq
        } else {
            TK::PuncBang
        };
        l.push_token(token)
    });
    h!(b'<', l, {
        let token = if l.matches_unchecked(b'=') {
            TK::PuncLtEq
        } else if l.matches_unchecked(b'<') {
            if l.matches_unchecked(b'=') {
                TK::PuncShlEq
            } else {
                TK::PuncShl
            }
        } else {
            TK::PuncLt
        };
        l.push_token(token)
    });
    h!(b'>', l, {
        let token = if l.matches_unchecked(b'=') {
            TK::PuncGtEq
        } else if l.matches_unchecked(b'>') {
            if l.matches_unchecked(b'=') {
                TK::PuncShrEq
            } else {
                TK::PuncShr
            }
        } else {
            TK::PuncGt
        };
        l.push_token(token)
    });
    h!(b'+', l, {
        let token = if l.matches_unchecked(b'=') {
            TK::PuncPlusEq
        } else {
            TK::PuncPlus
        };
        l.push_token(token)
    });
    h!(b'-', l, {
        let token = if l.matches_unchecked(b'=') {
            TK::PuncMinusEq
        } else if l.matches_unchecked(b'>') {
            TK::PuncArrowRight
        } else {
            TK::PuncMinus
        };
        l.push_token(token)
    });
    h!(b'*', l, {
        let token = if l.matches_unchecked(b'=') {
            TK::PuncStarEq
        } else {
            TK::PuncStar
        };
        l.push_token(token)
    });
    h!(b'/', l, {
        let token = if l.matches_unchecked(b'=') {
            TK::PuncSlashEq
        } else if l.peek_unchecked() == b'/' {
            // todo: comments (line comment)
            // don't forget to change above call to matches_unchecked
            TK::PuncSlash
        } else if l.peek_unchecked() == b'*' {
            // todo: comments (block comment)
            // don't forget to change above call to matches_unchecked
            TK::PuncSlash
        } else {
            TK::PuncSlash
        };
        l.push_token(token)
    });
    h!(b'%', l, {
        let token = if l.matches_unchecked(b'=') {
            TK::PuncModuloEq
        } else {
            TK::PuncModulo
        };
        l.push_token(token)
    });
    h!(b'&', l, {
        let token = if l.matches_unchecked(b'&') {
            TK::PuncAndAnd
        } else if l.matches_unchecked(b'=') {
            TK::PuncAndEq
        } else {
            TK::PuncAnd
        };
        l.push_token(token)
    });
    h!(b'|', l, {
        let token = if l.matches_unchecked(b'|') {
            TK::PuncOrOr
        } else if l.matches_unchecked(b'=') {
            TK::PuncOrEq
        } else {
            TK::PuncOr
        };
        l.push_token(token)
    });
    h!(b'^', l, {
        let token = if l.matches_unchecked(b'=') {
            TK::PuncXorEq
        } else {
            TK::PuncXor
        };
        l.push_token(token)
    });

    h!(b'0', l, l.zero_lit());
    h!(range b'1'..=b'9', l, l.decimal_lit());

    h!(range b'a'..=b'z', l, l.wordlike());
    h!(range b'A'..=b'Z', l, l.wordlike());
    h!(b'_', l, l.wordlike());

    h!(b'\'', l, l.char_lit());
    h!(b'"', l, l.string_lit());

    handlers
};

fn unknown_character(lexer: &mut Lexer) {
    let c = unsafe { lexer.index_unchecked(lexer.start) };
    lexer.error_here(LexerErrorKind::UnknownCharacter(c));
}
