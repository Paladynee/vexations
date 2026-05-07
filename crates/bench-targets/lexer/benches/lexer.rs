use std::mem;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use vexations_compiler::compiler::lexer::Lexer;
use vexations_compiler::frontend::source::VexationsSource;
use vexations_generator::lexer_test_generator::LexerTestGenerator;

const BENCH_SOURCE_SEED: u64 = 1300;

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
    let source = VexationsSource::try_from_bytes(&short_source).unwrap();

    let mut group = c.benchmark_group("lexer_l1_cache");
    group.sample_size(10);
    group.throughput(criterion::Throughput::Bytes(short_source_len as u64));

    group.bench_function("short_source_many_iterations", |b| {
        let mut tokens = Vec::new();
        let mut spans = Vec::new();
        let mut idents = Vec::new();
        let mut errors = Vec::new();

        b.iter(|| {
            let v1 = mem::take(&mut tokens);
            let v2 = mem::take(&mut spans);
            let v3 = mem::take(&mut idents);
            let v4 = mem::take(&mut errors);
            let mut lexer =
                Lexer::new_reuse_allocations(source.clone(), v1, v2, v3, v4);

            lexer.lex_all();
            let (mut v1, mut v2, mut v3, mut v4) = lexer.finalize();
            mem::swap(&mut tokens, &mut v1);
            mem::swap(&mut spans, &mut v2);
            mem::swap(&mut idents, &mut v3);
            mem::swap(&mut errors, &mut v4);
        });
    });

    group.finish();
}

fn bench_long_source(c: &mut Criterion) {
    let long_source = generate_source(50000);
    let long_source_len = long_source.len();
    let source = VexationsSource::try_from_bytes(&long_source).unwrap();

    let mut group = c.benchmark_group("lexer_main_ram");
    group.sample_size(10);
    group.throughput(criterion::Throughput::Bytes(long_source_len as u64));

    group.bench_function("long_source_fewer_iterations", |b| {
        let mut tokens = Vec::new();
        let mut spans = Vec::new();
        let mut idents = Vec::new();
        let mut errors = Vec::new();

        b.iter(|| {
            let v1 = mem::take(&mut tokens);
            let v2 = mem::take(&mut spans);
            let v3 = mem::take(&mut idents);
            let v4 = mem::take(&mut errors);
            let mut lexer =
                Lexer::new_reuse_allocations(source.clone(), v1, v2, v3, v4);

            lexer.lex_all();
            let (mut v1, mut v2, mut v3, mut v4) = lexer.finalize();
            mem::swap(&mut tokens, &mut v1);
            mem::swap(&mut spans, &mut v2);
            mem::swap(&mut idents, &mut v3);
            mem::swap(&mut errors, &mut v4);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_short_source, bench_long_source);
criterion_main!(benches);
