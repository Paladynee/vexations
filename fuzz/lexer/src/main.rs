#[macro_use]
extern crate afl;
extern crate vexations_compiler;

use std::cell::RefCell;
use std::mem;

use vexations_compiler::compiler::lexer::Lexer;
use vexations_compiler::compiler::lexer::error::LexerError;
use vexations_compiler::frontend::source::VexationsSource;
use vexations_compiler::frontend::token::TokenKind;

thread_local! {
    pub static TEST_DATA: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    pub static TOKENS: RefCell<Vec<TokenKind>> = const { RefCell::new(Vec::new()) };
    pub static SPANS: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
    pub static IDENTS: RefCell<Vec<&'static str>> = const { RefCell::new(Vec::new()) };
    pub static ERRORS: RefCell<Vec<LexerError>> = const { RefCell::new(Vec::new()) };
}

fn main() {
    fuzz!(|data: &[u8]| {
        // beautiful
        TEST_DATA.with(|test_data| {
            TOKENS.with(|tokens| {
                SPANS.with(|spans| {
                    IDENTS.with(|idents| {
                        ERRORS.with(|errors| {
                            process(
                                data,
                                &mut *test_data.borrow_mut(),
                                &mut *tokens.borrow_mut(),
                                &mut *spans.borrow_mut(),
                                &mut *idents.borrow_mut(),
                                &mut *errors.borrow_mut(),
                            );
                        });
                    });
                });
            });
        });
    });
}

fn process(
    data: &[u8],
    test_data: &mut Vec<u8>,
    tokens: &mut Vec<TokenKind>,
    spans: &mut Vec<usize>,
    idents: &mut Vec<&'static str>, // This is 'static
    errors: &mut Vec<LexerError>,
) {
    let mut a = mem::take(test_data);
    let mut b = mem::take(tokens);
    let mut c = mem::take(spans);
    let mut d = mem::take(idents);
    let mut e = mem::take(errors);

    a.clear();
    a.extend_from_slice(data);
    a.extend_from_slice(&[0; 3]);

    if let Some(source) = VexationsSource::try_from_bytes(&a) {
        // SAFETY: read the safety comment below
        let mut lexer =
            unsafe { Lexer::new_reuse_static_allocations(source, b, c, d, e) };

        lexer.lex_all();

        let (toks, spans, mut idents, errs) = lexer.finalize();

        idents.clear();
        b = toks;
        c = spans;
        // SAFETY: this is sound because we don't read nor write to it, this
        // casts just the lifetime and the layout is guaranteed to be
        // the same because lifetimes do not cause monomorphization.
        d = unsafe {
            std::mem::transmute::<Vec<&'_ str>, Vec<&'static str>>(idents)
        };
        e = errs;
    }

    mem::swap(test_data, &mut a);
    mem::swap(tokens, &mut b);
    mem::swap(spans, &mut c);
    mem::swap(idents, &mut d);
    mem::swap(errors, &mut e);
}
