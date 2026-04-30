use core::hint::unreachable_unchecked;
use core::ptr;

use crate::lexer::Lexer;
use crate::lexer::LexerError;
use crate::lexer::LexerErrorKind;
use crate::token::TokenKind;

type LexFnPtr = for<'a, 'src_ptr, 'src> unsafe fn(
    lexer: &'a mut Lexer<'src_ptr, 'src>,
    c: u8,
    tokens: &mut Vec<TokenKind>,
    errors: &mut Vec<LexerError>,
    literals: &mut Vec<&'src str>,
);

macro_rules! lex_op_1 {
    ($Lexer:ident, $TokenVec:ident, $( $Byte:expr => $Tok:expr ),* ; _ => $Fallback:expr) => {
        unsafe {
            let ptr = $Lexer.src().as_ptr().add($Lexer.index);
            match *ptr {
                $( $Byte => {
                    $Lexer.index = $Lexer.index.unchecked_add(1);
                    $TokenVec.push_unchecked($Tok);
                } )*
                _ => {
                    $TokenVec.push_unchecked($Fallback);
                }
            }
        }
    };
}

pub trait PushUnchecked<T> {
    unsafe fn push_unchecked(&mut self, value: T);
}

impl<T> PushUnchecked<T> for Vec<T> {
    unsafe fn push_unchecked(&mut self, value: T) {
        let len = self.len();
        unsafe {
            let end = self.as_mut_ptr().add(len);
            ptr::write(end, value);
            self.set_len(len.unchecked_add(1));
        }
    }
}

macro_rules! lex_op_2 {
    ($Lexer:ident, $TokenVec:ident,
     $( 2: [$Byte1of2:expr, $Byte2of2:expr] => $Tok2:expr ),* ;
      1: $( $Byte1of1:expr => $Tok1:expr ),* ;
     _ => $Fallback:expr) => {
        unsafe {
            let ptr = $Lexer.src().as_ptr().add($Lexer.index);
            // Single 16-bit unaligned load covers up to 3-char tokens (c + 2)
            let next2 = ptr::read_unaligned(ptr as *const u16);

            // Dummy if-branch for easy macro generation of else-if chains
            if false { unreachable_unchecked() }
            $(
                // u16::from_ne_bytes guarantees correctness regardless of target Endianness
                else if next2 == u16::from_ne_bytes([$Byte1of2, $Byte2of2]) {
                    $Lexer.index = $Lexer.index.unchecked_add(2);
                    $TokenVec.push_unchecked($Tok2);
                }
            )*
            else {
                // Extract the first byte dynamically out of the native u16
                let next1 = next2.to_ne_bytes()[0];
                match next1 {
                    $( $Byte1of1 => {
                        $Lexer.index = $Lexer.index.unchecked_add(1);
                        $TokenVec.push_unchecked($Tok1);
                    } )*
                    _ => {
                        $TokenVec.push_unchecked($Fallback);
                    }
                }
            }
        }
    };
}

// todo @Perf: find a way to push_unchecked so we avoid the allocation in the
// icache by reserving space beforehand.
#[rustfmt::skip]
pub static PER_CHAR_FN_TABLE: [LexFnPtr; 256] = {
    let mut table = [Lexer::lex_unknown as LexFnPtr; 256];

    table[b'(' as usize] = |_, _, t, _, _|
        unsafe { t.push_unchecked(TokenKind::IndentLParen) };
    table[b')' as usize] = |_, _, t, _, _|
        unsafe { t.push_unchecked(TokenKind::IndentRParen) };
    table[b'{' as usize] = |_, _, t, _, _|
        unsafe { t.push_unchecked(TokenKind::IndentLBrace) };
    table[b'}' as usize] = |_, _, t, _, _|
        unsafe { t.push_unchecked(TokenKind::IndentRBrace) };
    table[b'[' as usize] = |_, _, t, _, _|
        unsafe { t.push_unchecked(TokenKind::IndentLBracket) };
    table[b']' as usize] = |_, _, t, _, _|
        unsafe { t.push_unchecked(TokenKind::IndentRBracket) };
    table[b',' as usize] = |_, _, t, _, _|
        unsafe { t.push_unchecked(TokenKind::PuncComma) };
    table[b'.' as usize] = |_, _, t, _, _|
        unsafe { t.push_unchecked(TokenKind::PuncDot) };
    table[b';' as usize] = |_, _, t, _, _|
        unsafe { t.push_unchecked(TokenKind::PuncSemi) };

    // table[b':' as usize] =
    //     |l, _, t, _, _| unsafe {t.push_unchecked(if l.matches(b':') { TokenKind::PuncColonColon }
    //     else { TokenKind::PuncColon })};
    // table[b'+' as usize] =
    //     |l, _, t, _, _| unsafe {t.push_unchecked(if l.matches(b'=') { TokenKind::PuncPlusEq }
    //     else { TokenKind::PuncPlus })};
    // table[b'-' as usize] =
    //     |l, _, t, _, _| unsafe {t.push_unchecked(if l.matches(b'=') { TokenKind::PuncMinusEq }
    //     else if l.matches(b'>') {TokenKind::PuncArrowRight }
    //     else { TokenKind::PuncMinus })};
    // table[b'!' as usize] = 
    //     |l, _, t, _, _| unsafe {t.push_unchecked(if l.matches(b'=') { TokenKind::PuncBangEq }
    //     else { TokenKind::PuncBang })};
    // table[b'*' as usize] = 
    //     |l, _, t, _, _| unsafe {t.push_unchecked(if l.matches(b'=') { TokenKind::PuncStarEq }
    //     else { TokenKind::PuncStar })};
    // table[b'/' as usize] = 
    //     |l, _, t, _, _| unsafe {t.push_unchecked(if l.matches(b'=') { TokenKind::PuncSlashEq }
    //     else { TokenKind::PuncSlash })};
    // table[b'%' as usize] = 
    //     |l, _, t, _, _| unsafe {t.push_unchecked(if l.matches(b'=') { TokenKind::PuncModuloEq }
    //     else { TokenKind::PuncModulo })};
    // table[b'^' as usize] = 
    //     |l, _, t, _, _| unsafe {t.push_unchecked(if l.matches(b'=') { TokenKind::PuncXorEq }
    //     else { TokenKind::PuncXor })};
    // table[b'=' as usize] = 
    //     |l, _, t, _, _| unsafe {t.push_unchecked(if l.matches(b'=') { TokenKind::PuncEqEq }
    //     else { TokenKind::PuncEq })};
    // table[b'<' as usize] = 
    //     |l, _, t, _, _| unsafe {t.push_unchecked(if l.matches(b'=') { TokenKind::PuncLtEq }
    //     else if l.matches(b'<') {
    //         if l.matches(b'=') { TokenKind::PuncShlEq }
    //         else { TokenKind::PuncShl }
    //     } else { TokenKind::PuncLt })};
    // table[b'>' as usize] = 
    //     |l, _, t, _, _| unsafe {t.push_unchecked(if l.matches(b'=') { TokenKind::PuncGtEq }
    //     else if l.matches(b'>') {
    //         if l.matches(b'=') { TokenKind::PuncShrEq }
    //         else { TokenKind::PuncShr }
    //     } else { TokenKind::PuncGt })};
    // table[b'|' as usize] = 
    //     |l, _, t, _, _| unsafe {t.push_unchecked(if l.matches(b'=') { TokenKind::PuncOrEq }
    //     else if l.matches(b'|') { TokenKind::PuncOrOr }
    //     else { TokenKind::PuncOr })};
    // table[b'&' as usize] =
    //     |l, _, t, _, _| unsafe {t.push_unchecked(if l.matches(b'=') { TokenKind::PuncAndEq }
    //     else if l.matches(b'&') { TokenKind::PuncAndAnd }
    //     else { TokenKind::PuncAnd })};

    table[b':' as usize] = |l, _, t, _, _| lex_op_1!(l, t,
        b':' => TokenKind::PuncColonColon ; _ => TokenKind::PuncColon);
    table[b'+' as usize] = |l, _, t, _, _| lex_op_1!(l, t,
        b'=' => TokenKind::PuncPlusEq ; _ => TokenKind::PuncPlus);
    table[b'-' as usize] = |l, _, t, _, _| lex_op_1!(l, t,
        b'=' => TokenKind::PuncMinusEq,
        b'>' => TokenKind::PuncArrowRight ;
        _ => TokenKind::PuncMinus);
    table[b'!' as usize] = |l, _, t, _, _| lex_op_1!(l, t,
        b'=' => TokenKind::PuncBangEq ; _ => TokenKind::PuncBang);
    table[b'*' as usize] = |l, _, t, _, _| lex_op_1!(l, t,
        b'=' => TokenKind::PuncStarEq ; _ => TokenKind::PuncStar);
    table[b'/' as usize] = |l, _, t, _, _| lex_op_1!(l, t,
        b'=' => TokenKind::PuncSlashEq ; _ => TokenKind::PuncSlash);
    table[b'%' as usize] = |l, _, t, _, _| lex_op_1!(l, t,
        b'=' => TokenKind::PuncModuloEq ; _ => TokenKind::PuncModulo);
    table[b'^' as usize] = |l, _, t, _, _| lex_op_1!(l, t,
        b'=' => TokenKind::PuncXorEq ; _ => TokenKind::PuncXor);
    table[b'=' as usize] = |l, _, t, _, _| lex_op_1!(l, t,
        b'=' => TokenKind::PuncEqEq ; _ => TokenKind::PuncEq);
    table[b'|' as usize] = |l, _, t, _, _| lex_op_1!(l, t,
        b'=' => TokenKind::PuncOrEq,
        b'|' => TokenKind::PuncOrOr ;
        _ => TokenKind::PuncOr);
    table[b'&' as usize] = |l, _, t, _, _| lex_op_1!(l, t,
        b'=' => TokenKind::PuncAndEq,
        b'&' => TokenKind::PuncAndAnd ;
        _ => TokenKind::PuncAnd);


    table[b'<' as usize] = |l, _, t, _, _| lex_op_2!(
        l, t,
        2: [b'<', b'='] => TokenKind::PuncShlEq ;
        1: b'<' => TokenKind::PuncShl,
        b'=' => TokenKind::PuncLtEq ;
        _ => TokenKind::PuncLt
    );

    table[b'>' as usize] = |l, _, t, _, _| lex_op_2!(
        l, t,
        2: [b'>', b'='] => TokenKind::PuncShrEq ;
        1: b'>' => TokenKind::PuncShr,
        b'=' => TokenKind::PuncGtEq ;
        _ => TokenKind::PuncGt
    );


    // numeric
    table[b'0' as usize] =
        |l, _, t, e, lit| unsafe { l.zero_lit(t, e, lit) };

    let mut i = b'1';
    while i <= b'9' {
        table[i as usize] =
            |l, _, t, e, lit| unsafe { l.num_lit(t, e, lit) };
        i += 1;
    }

    // identifier
    let mut i = b'a';
    while i <= b'z' {
        table[i as usize] =
            |l, _, t, e, lit| unsafe { l.alnum_lit(t, e, lit) };
        i += 1;
    }
    let mut i = b'A';
    while i <= b'Z' {
        table[i as usize] =
            |l, _, t, e, lit| unsafe { l.alnum_lit(t, e, lit) };
        i += 1;
    }
    table[b'_' as usize] =
        |l, _, t, e, lit| unsafe { l.alnum_lit(t, e, lit) };

    // string
    table[b'"' as usize] =
        |l, _, t, e, lit| unsafe { l.string_lit(t, e, lit) };
    table[b'\'' as usize] =
        |l, _, t, e, lit| unsafe { l.char_lit(t, e, lit) };

    table
};

impl<'a, 'src> Lexer<'a, 'src> {
    #[inline]
    pub fn lex_unknown(
        lexer: &mut Lexer<'_, '_>, c: u8, _tokens: &mut Vec<TokenKind>,
        errors: &mut Vec<LexerError>, _idents: &mut Vec<&str>,
    ) {
        errors.push(LexerError {
            location: lexer.location(),
            kind: LexerErrorKind::UnknownCharacter(c),
        })
    }
}
