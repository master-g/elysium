use criterion::{black_box, criterion_group, Criterion};
use elysium_game::okey_mahjong::*;

pub fn benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group("okey_is_run");

	group.bench_function("Bench true, 0 joker", |b| {
		b.iter(|| {
			okey_is_run(black_box(&[
				Tile::Red01,
				Tile::Red02,
				Tile::Red03,
				Tile::Red04,
				Tile::Red05,
				Tile::Red06,
				Tile::Red07,
				Tile::Red08,
				Tile::Red09,
				Tile::Red10,
				Tile::Red11,
				Tile::Red12,
				Tile::Red13,
			]));
		});
	});

	group.bench_function("Bench false, 0 joker", |b| {
		b.iter(|| {
			okey_is_run(black_box(&[
				Tile::Yellow01,
				Tile::Yellow02,
				Tile::Yellow03,
				Tile::Yellow04,
				Tile::Yellow05,
				Tile::Yellow06,
				Tile::Yellow07,
				Tile::Yellow08,
				Tile::Yellow09,
				Tile::Yellow10,
				Tile::Yellow11,
				Tile::Yellow13,
			]));
		});
	});

	group.bench_function("Bench true, 1 joker", |b| {
		b.iter(|| {
			okey_is_run(black_box(&[
				Tile::Yellow01,
				Tile::Yellow02,
				Tile::Yellow03,
				Tile::Yellow04,
				Tile::Yellow05,
				Tile::Yellow06,
				Tile::Yellow07,
				Tile::Yellow08,
				Tile::Yellow09,
				Tile::Yellow10,
				Tile::Yellow12,
				Tile::Joker,
			]));
		});
	});

	group.bench_function("Bench true, 2 jokers", |b| {
		b.iter(|| {
			okey_is_run(black_box(&[
				Tile::Yellow01,
				Tile::Yellow02,
				Tile::Yellow03,
				Tile::Yellow05,
				Tile::Yellow06,
				Tile::Yellow07,
				Tile::Yellow08,
				Tile::Yellow10,
				Tile::Yellow11,
				Tile::Joker,
				Tile::Joker,
			]));
		});
	});

	group.bench_function("Bench false, 2 joker", |b| {
		b.iter(|| {
			okey_is_run(black_box(&[
				Tile::Yellow01,
				Tile::Yellow02,
				Tile::Yellow04,
				Tile::Yellow05,
				Tile::Yellow06,
				Tile::Yellow09,
				Tile::Yellow10,
				Tile::Joker,
				Tile::Joker,
			]));
		});
	});

	group.bench_function("Bench true, 0 jokers", |b| {
		b.iter(|| {
			okey_is_run(black_box(&[
				Tile::Yellow01,
				Tile::Yellow03,
				Tile::Yellow04,
				Tile::Yellow05,
				Tile::Yellow06,
				Tile::Yellow07,
				Tile::Yellow08,
				Tile::Yellow09,
				Tile::Yellow10,
				Tile::Yellow11,
				Tile::Yellow12,
				Tile::Yellow13,
			]));
		});
	});

	group.finish();
}

criterion_group!(benches, benchmark);
