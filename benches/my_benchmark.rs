use criterion::{black_box, criterion_group, criterion_main, Criterion};
use heca_lib::prelude::*;
use heca_lib::HebrewYear;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("get_holiday", |b| {
        b.iter(|| {
            let y = HebrewYear::new(black_box(5779)).unwrap();
            let mut out = Vec::new();
            y.get_holidays_extend(
                Location::Chul,
                &[
                    TorahReadingType::YomTov,
                ],
                &mut out,
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
