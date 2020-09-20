extern crate alloc;
use alloc::vec;
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};
use malamute::{
    FisherYates,
    SatCycles,
};

pub fn f_yates(c: &mut Criterion) {
    let mut fy = FisherYates::new(vec![1, 5, 9, 18, 39, 42].as_slice());
    c.bench_function("fisher-yates", |b| b.iter(|| fy.next()));
}

pub fn s_cycles(c: &mut Criterion) {
    let mut sc = SatCycles::new(vec![1, 5, 9, 18, 39, 42].as_slice());
    c.bench_function("sattolo-cycles", |b| b.iter(|| sc.next()));
}

criterion_group!(benches, f_yates, s_cycles,);
criterion_main!(benches);
