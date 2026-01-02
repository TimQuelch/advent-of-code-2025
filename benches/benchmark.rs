use std::fs;

use aoc25::days::DAYS;
use criterion::{Criterion, criterion_group, criterion_main};

fn day_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("days");
    DAYS.iter().for_each(|day| {
        group.bench_function(format!("{}_part1", day.name).as_str(), |b| {
            b.iter(|| day.part1());
        });
        group.bench_function(format!("{}_part2", day.name).as_str(), |b| {
            b.iter(|| day.part2());
        });
    });
    group.finish();
}

fn all_benches(c: &mut Criterion) {
    c.bench_function("all", |b| {
        b.iter(|| {
            DAYS.iter()
                .map(|day| (day.part1(), day.part2()))
                .collect::<Vec<_>>()
        });
    });
}

criterion_group!(days, day_benches);
criterion_group! {
    name = all;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(15));
    targets = all_benches
}
criterion_main!(days, all);
