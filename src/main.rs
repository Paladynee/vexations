#![allow(clippy::missing_safety_doc)] // no they're not, i'm just using a different convention
#![feature(likely_unlikely)]

pub mod lexer;
pub mod source;
pub mod token;

use std::env;
use std::fs;

use crate::source::VexationsSource;

fn main() {
    let args = env::args_os().collect::<Vec<_>>();
    // very primitive module loader here:
    let mut src = fs::read(&args[1]).unwrap();
    // source extender with 3 zero bytes for performant lexer,
    // should be handled in module loader
    src.extend_from_slice(&[0, 0, 0]);
    let Some(source) = VexationsSource::try_from_bytes(src.as_slice()) else {
        eprintln!("file {} is not ascii source!", args[1].display());
        return;
    };

    let mut tokens = vec![];
    let mut errors = vec![];
    let mut idents = vec![];
    lexer::lex(&source, &mut tokens, &mut errors, &mut idents);

    println!("tokens: {:#?}", tokens);
    println!("idents: {:#?}", idents);
    println!("errors: {:#?}", errors);
}
