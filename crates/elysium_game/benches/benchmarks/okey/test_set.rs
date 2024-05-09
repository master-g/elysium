use criterion::{black_box, criterion_group, Criterion};
use elysium_game::okey_mahjong::*;

pub fn benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group("okey_is_set");

	group.bench_function("Bench false, duplicate colors", |b| {
		b.iter(|| {
			okey_is_set(black_box(&[Tile::Yellow01, Tile::Black01, Tile::Red01, Tile::Red01]));
		});
	});

	group.bench_function("Bench true, one joker", |b| {
		b.iter(|| {
			okey_is_set(black_box(&[Tile::Yellow01, Tile::Black01, Tile::Red01, Tile::Joker]));
		});
	});

	group.bench_function("Bench false, one joker", |b| {
		b.iter(|| {
			okey_is_set(black_box(&[Tile::Yellow01, Tile::Black01, Tile::Red02, Tile::Joker]));
		});
	});
}

criterion_group!(benches, benchmark);
