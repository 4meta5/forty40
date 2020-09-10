use criterion::{black_box, criterion_group, criterion_main, Criterion};
use multicomb;

// pub fn heap_benchmark(c: &mut Criterion) {
//     c.bench_function("heap", |b| {
//         b.iter(|| naive(black_box(vec![1, 3, 5, 7].as_slice())))
//     });
// }

// pub fn lex_benchmark(c: &mut Criterion) {
//     c.bench_function("lex", |b| b.iter(|| optimized(black_box(20))));
// }

// criterion_group!(benches, naive_benchmark, optimized_benchmark);
// criterion_main!(benches);
