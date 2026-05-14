use std::fs;
use std::path::PathBuf;

use printerator::PrinterateDisplay;

use crate::compiler::lexer::lex;
use crate::compiler::parser::Parser;
use crate::frontend::source::VexationsSource;

pub mod lexer;
pub mod parser;

pub fn compile(in_files: Vec<PathBuf>, out_file: PathBuf) {
    if in_files.is_empty() {
        eprintln!("No input files provided");
        return;
    }

    for in_file in in_files {
        let mut bytes = fs::read(&in_file).unwrap();
        bytes.extend_from_slice(&[0; 3]);
        let source = VexationsSource::try_from_bytes(&bytes).unwrap();

        let mut lexer = lex(source.clone());

        if !lexer.errors_view().is_empty() {
            eprintln!("errors during lexing:");
            for error in lexer.take_errors() {
                eprintln!(
                    "in file {}\n\t{}",
                    in_file.display(),
                    error.display(source.clone())
                );
            }
        }

        let mut parser = Parser::new(
            source.clone(),
            lexer.take_tokens(),
            lexer.take_spans(),
            lexer.take_idents(),
        );
    }

    println!("finished compiling");
    todo!()
}
