use crate::compiler::lexer::Lexer;
use crate::compiler::lexer::error::LexerError;
use crate::compiler::lexer::error::LexerErrorKind;
use crate::frontend::token::TokenKind as TK;

pub type PerCharHandler = for<'lx> fn(&mut Lexer<'lx>);

pub static PER_CHAR_DISPATCHER: [PerCharHandler; 256] = {
    let mut handlers = [unknown_character as PerCharHandler; 256];
    macro_rules! h {
        (raw $char:literal, $kind:ident) => {
            handlers[$char as usize] = |l| l.tokens.push(TK::$kind);
        };
        ( $char:literal, $lexer:ident, $($body:tt)* ) => {
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
    handlers
};

fn unknown_character(lexer: &mut Lexer) {
    let c = unsafe { lexer.index_unchecked(lexer.start) };
    let err = LexerError {
        location: Lexer::location(lexer.source(), lexer.start),
        kind: LexerErrorKind::UnknownCharacter(c),
    };
    lexer.errors.push(err);
}
