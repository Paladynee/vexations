use std::mem;

use criterion::Criterion;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use vexations_compiler::compiler::lexer::Lexer;
use vexations_compiler::compiler::lexer::error::LexerError;
use vexations_compiler::frontend::source::VexationsSource;
use vexations_compiler::frontend::token::TokenKind;
use vexations_generator::lexer_test_generator::LexerTestGenerator;

const TOTAL_WORK_BYTES: usize = 10 * 1024 * 1024;

// const AMOUNT_TOKENS_TEST: usize = 10000000;
// let (bytes, expected) = {
//     let mut generator = LexerTestGenerator::new(
//         AMOUNT_TOKENS_TEST,
//         Some(TEST_SEED + 1),
//     );
//     let mut out_source = Vec::with_capacity(AMOUNT_TOKENS_TEST * 4);
//     let mut expected_tokens = Vec::with_capacity(AMOUNT_TOKENS_TEST);
//     while let Some((ws, kind, span)) = generator.next_span() {
//         if let Some(whitespace) = ws {
//             out_source.extend_from_slice(whitespace.as_bytes());
//         }
//         out_source.extend_from_slice(span.as_bytes());
//         expected_tokens.push(kind);
//     }
//     out_source.extend_from_slice(&[0; 3]);
//     (out_source, expected_tokens)
// };
// let src = VexationsSource::try_from_bytes(&bytes).unwrap();
// let (toks, spans, idents, errs) = lexer::lex(src.clone()).finalize();
// validate_gen_output(src, &toks, &spans, &idents, errs, &expected);

const BENCH_SOURCE_SEED: u64 = 100;

fn generate_source(n_tokens: usize) -> Vec<u8> {
    let mut generator =
        LexerTestGenerator::new(n_tokens, Some(BENCH_SOURCE_SEED + 1));
    let mut out_source = Vec::with_capacity(n_tokens * 4);
    while let Some((ws, _kind, span)) = generator.next_span() {
        if let Some(whitespace) = ws {
            out_source.extend_from_slice(whitespace.as_bytes());
        }
        out_source.extend_from_slice(span.as_bytes());
    }
    out_source.extend_from_slice(&[0; 3]);
    out_source
}

fn bench_short_source(c: &mut Criterion) {
    let short_source = generate_source(200);
    let short_source_len = short_source.len();
    let iterations = TOTAL_WORK_BYTES / short_source_len;
    let source = VexationsSource::try_from_bytes(&short_source).unwrap();

    let mut group = c.benchmark_group("lexer_l1_cache");
    group.sample_size(10);
    group.throughput(criterion::Throughput::Bytes((short_source_len * iterations) as u64));

    group.bench_function("short_source_many_iterations", |b| {
        let mut tokens = Vec::new();
        let mut spans = Vec::new();
        let mut idents = Vec::new();
        let mut errors = Vec::new();

        b.iter(|| {
            for _ in 0..iterations {
                let v1 = mem::take(&mut tokens);
                let v2 = mem::take(&mut spans);
                let v3 = mem::take(&mut idents);
                let v4 = mem::take(&mut errors);
                let mut lexer = Lexer::new_reuse_allocations(
                    source.clone(),
                    v1,
                    v2,
                    v3,
                    v4,
                );

                lexer.lex_all();
                let (mut v1, mut v2, mut v3, mut v4) = lexer.finalize();
                mem::swap(&mut tokens, &mut v1);
                mem::swap(&mut spans, &mut v2);
                mem::swap(&mut idents, &mut v3);
                mem::swap(&mut errors, &mut v4);
            }
        });
    });

    group.finish();
}

fn bench_long_source(c: &mut Criterion) {
    let long_source = generate_source(500000);
    let long_source_len = long_source.len();
    let iterations = (TOTAL_WORK_BYTES / long_source_len).max(1);
    let source = VexationsSource::try_from_bytes(&long_source).unwrap();

    let mut group = c.benchmark_group("lexer_main_ram");
    group.sample_size(10);
    group.throughput(criterion::Throughput::Bytes((long_source_len * iterations) as u64));

    group.bench_function("long_source_fewer_iterations", |b| {
        let mut tokens = Vec::new();
        let mut spans = Vec::new();
        let mut idents = Vec::new();
        let mut errors = Vec::new();

        b.iter(|| {
            for _ in 0..iterations {
                let v1 = mem::take(&mut tokens);
                let v2 = mem::take(&mut spans);
                let v3 = mem::take(&mut idents);
                let v4 = mem::take(&mut errors);
                let mut lexer = Lexer::new_reuse_allocations(
                    source.clone(),
                    v1,
                    v2,
                    v3,
                    v4,
                );

                lexer.lex_all();
                let (mut v1, mut v2, mut v3, mut v4) = lexer.finalize();
                mem::swap(&mut tokens, &mut v1);
                mem::swap(&mut spans, &mut v2);
                mem::swap(&mut idents, &mut v3);
                mem::swap(&mut errors, &mut v4);
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_short_source, bench_long_source);
criterion_main!(benches);
