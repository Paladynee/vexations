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

  let times = 1000000000 / src.len();
  eprintln!(
    "running the lexer on out put of size {}, running {} times",
    src.len(),
    times
  );
  let start = std::time::Instant::now();
  let mut tokens = vec![];
  let mut errors = vec![];
  let mut idents = vec![];
  for _ in 0..times {
    tokens.clear();
    errors.clear();
    idents.clear();
    lexer::lex(&source, &mut tokens, &mut errors, &mut idents);
  }
  let end = start.elapsed();
  eprintln!("took {:?} to lex the same program {} times", end, times);
  let bytes_per_sec = (src.len() as f64 * times as f64) / end.as_secs_f64();
  eprintln!("that's {:.2} bytes/sec", bytes_per_sec);

  println!(
    "tokens len: {}, tokens: {:#?}",
    tokens.len(),
    tokens.get(0..(10.min(tokens.len()))).unwrap_or_default()
  );
  println!(
    "idents len: {}, idents: {:#?}",
    idents.len(),
    idents.get(0..(10.min(idents.len()))).unwrap_or_default()
  );
  println!(
    "errors len: {}, errors: {:#?}",
    errors.len(),
    errors.get(0..(10.min(errors.len()))).unwrap_or_default()
  );
}
