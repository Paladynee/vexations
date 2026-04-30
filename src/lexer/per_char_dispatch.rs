use crate::lexer::Lexer;
use crate::lexer::LexerError;
use crate::token::TokenKind;

type LexFnPtr = for<'a, 'src_ptr, 'src> fn(
    lexer: &'a mut Lexer<'src_ptr, 'src>,
    c: u8,
    tokens: &mut Vec<TokenKind>,
    errors: &mut Vec<LexerError>,
    idents: &mut Vec<&'src str>,
);

pub static PER_CHAR_FN_TABLE: [LexFnPtr; 256] = const {
    let mut table = [Lexer::lex_unknown as LexFnPtr; 256];

    table[b'(' as usize] = |_, _, t, _, _| t.push(TokenKind::IndentLParen);
    table[b')' as usize] = |_, _, t, _, _| t.push(TokenKind::IndentRParen);
    table[b'{' as usize] = |_, _, t, _, _| t.push(TokenKind::IndentLBrace);
    table[b'}' as usize] = |_, _, t, _, _| t.push(TokenKind::IndentRBrace);
    table[b'[' as usize] = |_, _, t, _, _| t.push(TokenKind::IndentLBracket);
    table[b']' as usize] = |_, _, t, _, _| t.push(TokenKind::IndentRBracket);
    table[b',' as usize] = |_, _, t, _, _| t.push(TokenKind::PuncComma);
    table[b'.' as usize] = |_, _, t, _, _| t.push(TokenKind::PuncDot);
    table[b';' as usize] = |_, _, t, _, _| t.push(TokenKind::PuncSemi);

    // table[b':' as usize] = Lexer::lex_colon_family;
    // table[b'+' as usize] = Lexer::lex_plus_family;
    // table[b'-' as usize] = Lexer::lex_minus_family;
    // table[b'!' as usize] = Lexer::lex_bang_family;
    // table[b'*' as usize] = Lexer::lex_star_family;
    // table[b'/' as usize] = Lexer::lex_slash_family;
    // table[b'%' as usize] = Lexer::lex_modulo_family;
    // table[b'^' as usize] = Lexer::lex_xor_family;
    // table[b'=' as usize] = Lexer::lex_eq_family;
    // table[b'<' as usize] = Lexer::lex_lt_family;
    // table[b'>' as usize] = Lexer::lex_gt_family;
    // table[b'|' as usize] = Lexer::lex_or_family;
    // table[b'&' as usize] = Lexer::lex_and_family;

    table[b'0' as usize] = |l, _, t, e, i| unsafe { l.zero_lit(t, e, i) };

    let mut i = b'1';
    while i <= b'9' {
        table[i as usize] =
            |l, _, t, e, i_vec| unsafe { l.num_lit(t, e, i_vec) };
        i += 1;
    }

    let mut i = b'a';
    while i <= b'z' {
        table[i as usize] =
            |l, _, t, e, i_vec| unsafe { l.alnum_lit(t, e, i_vec) };
        table[(i - 32) as usize] =
            |l, _, t, e, i_vec| unsafe { l.alnum_lit(t, e, i_vec) };
        i += 1;
    }
    table[b'_' as usize] = |l, _, t, e, i| unsafe { l.alnum_lit(t, e, i) };

    table[b'"' as usize] = |l, _, t, e, i| unsafe { l.string_lit(t, e, i) };
    table[b'\'' as usize] = |l, _, t, e, i| unsafe { l.char_lit(t, e, i) };

    table
};

impl<'a, 'src> Lexer<'a, 'src> {
    #[inline]
    pub fn lex_unknown(
        _lexer: &mut Lexer<'_, '_>, _c: u8, _tokens: &mut Vec<TokenKind>,
        _errors: &mut Vec<LexerError>, _idents: &mut Vec<&str>,
    ) {
    }
}
