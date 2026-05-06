use std::fs;

use printerator::PrinterateDebug;
use printerator::PrinterateDisplay;

use crate::Options;
use crate::compiler::lexer::Lexer;
use crate::compiler::lexer::lex;
use crate::middle::source::VexationsSource;

pub mod lexer;

pub fn compile(
    Options {
        in_files,
        out_file,
        ..
    }: Options,
) {
    if in_files.is_empty() {
        eprintln!("No input files provided");
        return;
    }

    for in_file in in_files {
        let mut bytes = fs::read(&in_file).unwrap();
        bytes.extend_from_slice(&[0; 3]);
        let source = VexationsSource::try_from_bytes(&bytes).unwrap();

        let (tokens, idents, errors) = lex(source.clone());

        if !errors.is_empty() {
            eprintln!("errors during lexing:");
            for error in errors {
                eprintln!("in file {}\n\t{}", in_file.display(), error);
            }
            continue;
        }

        println!("lex finished successfully");
        println!(
            "Tokens: {}",
            tokens
                .get(0..(tokens.len().min(10)))
                .unwrap_or_default()
                .iter()
                .map(|t| t.source_repr())
                .printer()
        );
        println!(
            "Idents: {}",
            idents
                .get(0..(idents.len().min(10)))
                .unwrap_or_default()
                .iter()
                .printer()
        );
    }
    todo!()
}
