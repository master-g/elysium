use criterion::{black_box, criterion_group, Criterion};

pub fn benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group("bench_group_name");

	group.bench_function("Bench true, 0 joker", |b| {
		b.iter(|| {});
	});
}

criterion_group!(benches, benchmark);
