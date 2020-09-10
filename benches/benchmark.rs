use criterion::{criterion_group, criterion_main, Criterion};
use multicomb::{Heap, Lex};

pub fn heap_benchmark(c: &mut Criterion) {
    let mut h = Heap::new(vec![1, 3, 5, 7].as_slice()).unwrap();
    c.bench_function("heap", |b| b.iter(|| h.next()));
}

pub fn lex_benchmark(c: &mut Criterion) {
    let mut l = Lex::new(vec![1, 3, 5, 7], 4);
    c.bench_function("lex", |b| b.iter(|| l.next()));
}

criterion_group!(benches, heap_benchmark, lex_benchmark);
criterion_main!(benches);
