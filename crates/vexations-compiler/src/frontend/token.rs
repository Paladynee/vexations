#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    KwLet,
    KwFn,
    KwReturn,
    KwExtern,
    KwConst,
    KwMut,
    KwAnymut,
    KwCompiletime,
    KwRuntime,
    KwStatic,
    KwType,
    KwCast,
    KwIf,
    KwElse,
    KwFor,
    KwWhile,
    KwLoop,
    KwContinue,
    KwBreak,

    KwAdtStruct,
    KwAdtEnum,
    KwAdtUnion,

    LitInteger,
    LitFloat,
    LitStr,
    LitChar,
    LitBool,
    LitUninit,
    LitIdentifier,

    PuncDot,
    PuncComma,
    PuncSemi,
    PuncColon,
    PuncColonColon,
    PuncArrowRight,

    PuncEq,
    PuncEqEq,
    PuncBang,
    PuncBangEq,
    PuncLt,
    PuncLtEq,
    PuncGt,
    PuncGtEq,

    PuncPlus,
    PuncMinus,
    PuncStar,
    PuncSlash,
    PuncModulo,

    PuncAnd,
    PuncAndAnd,
    PuncOr,
    PuncOrOr,
    PuncXor,

    PuncShl,
    PuncShr,

    PuncPlusEq,
    PuncMinusEq,
    PuncStarEq,
    PuncSlashEq,
    PuncModuloEq,
    PuncAndEq,
    PuncOrEq,
    PuncXorEq,
    PuncShlEq,
    PuncShrEq,

    IndentLParen,
    IndentRParen,
    IndentLBrace,
    IndentRBrace,
    IndentLBracket,
    IndentRBracket,

    MetaDummy,
}

impl TokenKind {
    pub const ALL: &[TokenKind] = &[
        TokenKind::KwLet,
        TokenKind::KwFn,
        TokenKind::KwReturn,
        TokenKind::KwExtern,
        TokenKind::KwConst,
        TokenKind::KwMut,
        TokenKind::KwAnymut,
        TokenKind::KwCompiletime,
        TokenKind::KwRuntime,
        TokenKind::KwStatic,
        TokenKind::KwType,
        TokenKind::KwCast,
        TokenKind::KwIf,
        TokenKind::KwElse,
        TokenKind::KwFor,
        TokenKind::KwWhile,
        TokenKind::KwLoop,
        TokenKind::KwContinue,
        TokenKind::KwBreak,
        TokenKind::KwAdtStruct,
        TokenKind::KwAdtEnum,
        TokenKind::KwAdtUnion,
        TokenKind::LitInteger,
        TokenKind::LitFloat,
        TokenKind::LitStr,
        TokenKind::LitChar,
        TokenKind::LitBool,
        TokenKind::LitUninit,
        TokenKind::LitIdentifier,
        TokenKind::PuncDot,
        TokenKind::PuncComma,
        TokenKind::PuncSemi,
        TokenKind::PuncColon,
        TokenKind::PuncColonColon,
        TokenKind::PuncArrowRight,
        TokenKind::PuncEq,
        TokenKind::PuncEqEq,
        TokenKind::PuncBang,
        TokenKind::PuncBangEq,
        TokenKind::PuncLt,
        TokenKind::PuncLtEq,
        TokenKind::PuncGt,
        TokenKind::PuncGtEq,
        TokenKind::PuncPlus,
        TokenKind::PuncMinus,
        TokenKind::PuncStar,
        TokenKind::PuncSlash,
        TokenKind::PuncModulo,
        TokenKind::PuncAnd,
        TokenKind::PuncAndAnd,
        TokenKind::PuncOr,
        TokenKind::PuncOrOr,
        TokenKind::PuncXor,
        TokenKind::PuncShl,
        TokenKind::PuncShr,
        TokenKind::PuncPlusEq,
        TokenKind::PuncMinusEq,
        TokenKind::PuncStarEq,
        TokenKind::PuncSlashEq,
        TokenKind::PuncModuloEq,
        TokenKind::PuncAndEq,
        TokenKind::PuncOrEq,
        TokenKind::PuncXorEq,
        TokenKind::PuncShlEq,
        TokenKind::PuncShrEq,
        TokenKind::IndentLParen,
        TokenKind::IndentRParen,
        TokenKind::IndentLBrace,
        TokenKind::IndentRBrace,
        TokenKind::IndentLBracket,
        TokenKind::IndentRBracket,
        TokenKind::MetaDummy,
    ];

    #[rustfmt::skip]
    #[inline]
    pub const fn is_identifier_extractable(self) -> bool {
        matches!(
            self,
            // WARNING: validate all callsites before modifying. unsafe code relies on this for exhaustiveness
            TokenKind::LitInteger |
            TokenKind::LitFloat |
            TokenKind::LitStr |
            TokenKind::LitChar |
            TokenKind::LitBool |
            TokenKind::LitUninit |
            TokenKind::LitIdentifier
        )
    }

    #[rustfmt::skip]
    #[inline]
    pub const fn is_literal_item(self) -> bool {
        matches!(
            self,
            // WARNING: validate all callsites before modifying. unsafe code relies on this for exhaustiveness
            TokenKind::LitInteger |
            TokenKind::LitFloat |
            TokenKind::LitStr |
            TokenKind::LitChar |
            TokenKind::LitBool |
            TokenKind::LitUninit
        )
    }

    #[rustfmt::skip]
    #[inline]
    pub const fn is_keyword(self) -> bool {
        matches!(
            self,
            TokenKind::KwLet |
            TokenKind::KwFn |
            TokenKind::KwReturn |
            TokenKind::KwExtern |
            TokenKind::KwConst |
            TokenKind::KwMut |
            TokenKind::KwAnymut |
            TokenKind::KwCompiletime |
            TokenKind::KwRuntime |
            TokenKind::KwStatic |
            TokenKind::KwType |
            TokenKind::KwCast |
            TokenKind::KwIf |
            TokenKind::KwElse |
            TokenKind::KwFor |
            TokenKind::KwWhile |
            TokenKind::KwLoop |
            TokenKind::KwContinue |
            TokenKind::KwBreak |
            TokenKind::KwAdtStruct |
            TokenKind::KwAdtEnum |
            TokenKind::KwAdtUnion
        )
    }

    #[inline]
    pub const fn can_safely_follow(prev: TokenKind, next: TokenKind) -> bool {
        let prev_is_wordlike =
            prev.is_keyword() || prev.is_identifier_extractable();
        let next_is_wordlike =
            next.is_keyword() || next.is_identifier_extractable();

        if prev_is_wordlike && next_is_wordlike {
            // wordlike tokens cannot be adjacent without whitespace because
            // they would merge into a single identifier during lexing
            return false;
        }

        match (prev, next) {
            // 2 character token combinations
            (TokenKind::PuncEq, TokenKind::PuncEq) => false, // ==
            (TokenKind::PuncBang, TokenKind::PuncEq) => false, // !=
            (TokenKind::PuncLt, TokenKind::PuncEq) => false, // <=
            (TokenKind::PuncGt, TokenKind::PuncEq) => false, // >=
            (TokenKind::PuncLt, TokenKind::PuncLt) => false, // <<
            (TokenKind::PuncGt, TokenKind::PuncGt) => false, // >>
            (TokenKind::PuncAnd, TokenKind::PuncAnd) => false, // &&
            (TokenKind::PuncOr, TokenKind::PuncOr) => false, // ||
            (TokenKind::PuncPlus, TokenKind::PuncEq) => false, // +=
            (TokenKind::PuncMinus, TokenKind::PuncEq) => false, // -=
            (TokenKind::PuncMinus, TokenKind::PuncGt) => false, // ->
            (TokenKind::PuncStar, TokenKind::PuncEq) => false, // *=
            (TokenKind::PuncSlash, TokenKind::PuncEq) => false, // /=
            (TokenKind::PuncSlash, TokenKind::PuncSlash) => false, // //
            (TokenKind::PuncSlash, TokenKind::PuncStar) => false, // /*
            (TokenKind::PuncModulo, TokenKind::PuncEq) => false, // %=
            (TokenKind::PuncAnd, TokenKind::PuncEq) => false, // &=
            (TokenKind::PuncOr, TokenKind::PuncEq) => false, // |=
            (TokenKind::PuncXor, TokenKind::PuncEq) => false, // ^=
            (TokenKind::PuncColon, TokenKind::PuncColon) => false, // ::
            // 3 character token combinations
            (TokenKind::PuncShl, TokenKind::PuncEq) => false, // <<=
            (TokenKind::PuncShr, TokenKind::PuncEq) => false, // >>=
            (TokenKind::PuncLt, TokenKind::PuncLtEq) => false, // <, <= vs <<, =
            (TokenKind::PuncGt, TokenKind::PuncGtEq) => false, // >, >= vs >>, =
            (TokenKind::PuncEq, TokenKind::PuncEqEq) => false, // =, == vs ==, =
            (TokenKind::PuncBang, TokenKind::PuncEqEq) => false, // !, == vs !=, =
            (TokenKind::PuncSlash, TokenKind::PuncStarEq) => false, // /, *= vs /*, =
            (TokenKind::PuncSlash, TokenKind::PuncSlashEq) => false, // /, /= vs //, =
            (TokenKind::PuncMinus, TokenKind::PuncGtEq) => false, // -, >= vs ->, =
            // 4 character token combinations
            (TokenKind::PuncShr, TokenKind::PuncEqEq) => false, // >>, == vs >>=, = vs >>, =, = 
            (TokenKind::PuncShl, TokenKind::PuncEqEq) => false, // <<, == vs <<=, = vs <<, =, = 
            // multi character combinations
            (TokenKind::LitInteger, TokenKind::PuncDot) => false, /* erroneous float */
            // hopefully all other pairs are safe
            _ => true,
        }
    }

    #[inline]
    pub const fn source_repr(self) -> &'static str {
        match self {
            TokenKind::KwLet => "let",
            TokenKind::KwFn => "fn",
            TokenKind::KwReturn => "return",
            TokenKind::KwExtern => "extern",
            TokenKind::KwConst => "const",
            TokenKind::KwMut => "mut",
            TokenKind::KwAnymut => "anymut",
            TokenKind::KwCompiletime => "compiletime",
            TokenKind::KwRuntime => "runtime",
            TokenKind::KwStatic => "static",
            TokenKind::KwType => "type",
            TokenKind::KwCast => "cast",
            TokenKind::KwIf => "if",
            TokenKind::KwElse => "else",
            TokenKind::KwFor => "for",
            TokenKind::KwWhile => "while",
            TokenKind::KwLoop => "loop",
            TokenKind::KwContinue => "continue",
            TokenKind::KwBreak => "break",
            TokenKind::KwAdtStruct => "struct",
            TokenKind::KwAdtEnum => "enum",
            TokenKind::KwAdtUnion => "union",
            TokenKind::LitInteger => "{integer}",
            TokenKind::LitFloat => "{float}",
            TokenKind::LitStr => "{string}",
            TokenKind::LitChar => "{char}",
            TokenKind::LitBool => "{bool}",
            TokenKind::LitUninit => "uninit",
            TokenKind::LitIdentifier => "{identifier}",
            TokenKind::PuncDot => ".",
            TokenKind::PuncComma => ",",
            TokenKind::PuncSemi => ";",
            TokenKind::PuncColon => ":",
            TokenKind::PuncColonColon => "::",
            TokenKind::PuncArrowRight => "->",
            TokenKind::PuncEq => "=",
            TokenKind::PuncEqEq => "==",
            TokenKind::PuncBang => "!",
            TokenKind::PuncBangEq => "!=",
            TokenKind::PuncLt => "<",
            TokenKind::PuncLtEq => "<=",
            TokenKind::PuncGt => ">",
            TokenKind::PuncGtEq => ">=",
            TokenKind::PuncPlus => "+",
            TokenKind::PuncMinus => "-",
            TokenKind::PuncStar => "*",
            TokenKind::PuncSlash => "/",
            TokenKind::PuncModulo => "%",
            TokenKind::PuncAnd => "&",
            TokenKind::PuncAndAnd => "&&",
            TokenKind::PuncOr => "|",
            TokenKind::PuncOrOr => "||",
            TokenKind::PuncXor => "^",
            TokenKind::PuncShl => "<<",
            TokenKind::PuncShr => ">>",
            TokenKind::PuncPlusEq => "+=",
            TokenKind::PuncMinusEq => "-=",
            TokenKind::PuncStarEq => "*=",
            TokenKind::PuncSlashEq => "/=",
            TokenKind::PuncModuloEq => "%=",
            TokenKind::PuncAndEq => "&=",
            TokenKind::PuncOrEq => "|=",
            TokenKind::PuncXorEq => "^=",
            TokenKind::PuncShlEq => "<<=",
            TokenKind::PuncShrEq => ">>=",
            TokenKind::IndentLParen => "(",
            TokenKind::IndentRParen => ")",
            TokenKind::IndentLBrace => "{",
            TokenKind::IndentRBrace => "}",
            TokenKind::IndentLBracket => "[",
            TokenKind::IndentRBracket => "]",
            TokenKind::MetaDummy => "<dummy token>",
        }
    }
}
