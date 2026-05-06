use crate::compiler::lexer::Lexer;
use crate::compiler::lexer::error::LexerError;
use crate::compiler::lexer::error::LexerErrorKind;
use crate::frontend::token::TokenKind as TK;

pub type PerCharHandler = for<'lx> fn(&mut Lexer<'lx>);

#[allow(unused_unsafe)]
pub static PER_CHAR_DISPATCHER: [PerCharHandler; 256] = {
    let mut handlers = [unknown_character as PerCharHandler; 256];

    // empty handler for nul byte
    handlers[0] = |_| {};

    macro_rules! h {
        ( raw $char:literal, $kind:ident ) => {
            handlers[$char as usize] = |l| l.tokens.push(TK::$kind);
        };
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
        if l.matches_unchecked(b':') {
            l.tokens.push(TK::PuncColonColon);
        } else {
            l.tokens.push(TK::PuncColon);
        }
    });
    h!(b'=', l, {
        if l.matches_unchecked(b'=') {
            l.tokens.push(TK::PuncEqEq);
        } else {
            l.tokens.push(TK::PuncEq);
        }
    });
    h!(b'!', l, {
        if l.matches_unchecked(b'=') {
            l.tokens.push(TK::PuncBangEq);
        } else {
            l.tokens.push(TK::PuncBang);
        }
    });
    h!(b'<', l, {
        if l.matches_unchecked(b'=') {
            l.tokens.push(TK::PuncLtEq);
        } else if l.matches_unchecked(b'<') {
            if l.matches_unchecked(b'=') {
                l.tokens.push(TK::PuncShlEq);
            } else {
                l.tokens.push(TK::PuncShl);
            }
        } else {
            l.tokens.push(TK::PuncLt);
        }
    });
    h!(b'>', l, {
        if l.matches_unchecked(b'=') {
            l.tokens.push(TK::PuncGtEq);
        } else if l.matches_unchecked(b'>') {
            if l.matches_unchecked(b'=') {
                l.tokens.push(TK::PuncShrEq);
            } else {
                l.tokens.push(TK::PuncShr);
            }
        } else {
            l.tokens.push(TK::PuncGt);
        }
    });
    h!(b'+', l, {
        if l.matches_unchecked(b'=') {
            l.tokens.push(TK::PuncPlusEq);
        } else {
            l.tokens.push(TK::PuncPlus);
        }
    });
    h!(b'-', l, {
        if l.matches_unchecked(b'=') {
            l.tokens.push(TK::PuncMinusEq);
        } else if l.matches_unchecked(b'>') {
            l.tokens.push(TK::PuncArrowRight);
        } else {
            l.tokens.push(TK::PuncMinus);
        }
    });
    h!(b'*', l, {
        if l.matches_unchecked(b'=') {
            l.tokens.push(TK::PuncStarEq);
        } else {
            l.tokens.push(TK::PuncStar);
        }
    });
    h!(b'/', l, {
        if l.matches_unchecked(b'=') {
            l.tokens.push(TK::PuncSlashEq);
        } else if l.peek_unchecked() == b'/' {
            // todo: comments (line comment)
            // don't forget to change above call to matches_unchecked
        } else if l.peek_unchecked() == b'*' {
            // todo: comments (block comment)
            // don't forget to change above call to matches_unchecked
        } else {
            l.tokens.push(TK::PuncSlash);
        }
    });
    h!(b'%', l, {
        if l.matches_unchecked(b'=') {
            l.tokens.push(TK::PuncModuloEq);
        } else {
            l.tokens.push(TK::PuncModulo);
        }
    });
    h!(b'&', l, {
        if l.matches_unchecked(b'&') {
            l.tokens.push(TK::PuncAndAnd);
        } else if l.matches_unchecked(b'=') {
            l.tokens.push(TK::PuncAndEq);
        } else {
            l.tokens.push(TK::PuncAnd);
        }
    });
    h!(b'|', l, {
        if l.matches_unchecked(b'|') {
            l.tokens.push(TK::PuncOrOr);
        } else if l.matches_unchecked(b'=') {
            l.tokens.push(TK::PuncOrEq);
        } else {
            l.tokens.push(TK::PuncOr);
        }
    });
    h!(b'^', l, {
        if l.matches_unchecked(b'=') {
            l.tokens.push(TK::PuncXorEq);
        } else {
            l.tokens.push(TK::PuncXor);
        }
    });
    h!(b'0', l, l.zero_lit());

    macro_rules! to {
        ($start:literal..=$end:literal, $lexer:ident, $($body:tt)*) => {{
            let mut c = $start;
            while c <= $end {
                h!(c, $lexer, $($body)*);
                c += 1;
            }
        }};
    }
    to!(b'1'..=b'9', l, l.decimal_lit());

    to!(b'a'..=b'z', l, l.wordlike());
    to!(b'A'..=b'Z', l, l.wordlike());
    h!(b'_', l, l.wordlike());

    h!(b'\'', l, l.char_lit());
    h!(b'"', l, l.string_lit());

    handlers
};

fn unknown_character(lexer: &mut Lexer) {
    let c = unsafe { lexer.index_unchecked(lexer.start) };
    lexer.error_here(LexerErrorKind::UnknownCharacter(c));
}
