use criterion::{black_box, criterion_group, criterion_main, Criterion};
use elysium_game::okey_mahjong::*;

pub fn criterion_benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group("okey_is_seven_pairs");

	group.bench_function("Bench true, 0 joker", |b| {
		b.iter(|| {
			okey_is_seven_pairs(black_box(&[
				Tile::Black06,
				Tile::Yellow03,
				Tile::Black06,
				Tile::Yellow03,
				Tile::Black07,
				Tile::Red02,
				Tile::Blue01,
				Tile::Blue01,
				Tile::Yellow01,
				Tile::Black07,
				Tile::Yellow05,
				Tile::Red02,
				Tile::Yellow01,
				Tile::Yellow05,
			]));
		});
	});

	group.bench_function("Bench false, 0 joker", |b| {
		b.iter(|| {
			okey_is_seven_pairs(black_box(&[
				Tile::Black13,
				Tile::Yellow03,
				Tile::Black06,
				Tile::Yellow03,
				Tile::Black07,
				Tile::Red02,
				Tile::Blue01,
				Tile::Blue01,
				Tile::Yellow01,
				Tile::Black07,
				Tile::Yellow05,
				Tile::Red02,
				Tile::Yellow01,
				Tile::Yellow05,
			]));
		});
	});

	group.bench_function("Bench true, 1 joker", |b| {
		b.iter(|| {
			okey_is_seven_pairs(black_box(&[
				Tile::Yellow01,
				Tile::Yellow01,
				Tile::Yellow02,
				Tile::Yellow02,
				Tile::Yellow03,
				Tile::Yellow03,
				Tile::Yellow04,
				Tile::Yellow04,
				Tile::Yellow05,
				Tile::Yellow05,
				Tile::Black06,
				Tile::Black06,
				Tile::Black07,
				Tile::Joker,
			]));
		});
	});

	group.bench_function("Bench true, 2 jokers", |b| {
		b.iter(|| {
			okey_is_seven_pairs(black_box(&[
				Tile::Yellow01,
				Tile::Yellow01,
				Tile::Red02,
				Tile::Red02,
				Tile::Yellow03,
				Tile::Yellow03,
				Tile::Blue04,
				Tile::Blue04,
				Tile::Yellow05,
				Tile::Yellow05,
				Tile::Joker,
				Tile::Black06,
				Tile::Black07,
				Tile::Joker,
			]));
		});
	});

	group.bench_function("Bench false, 1 joker", |b| {
		b.iter(|| {
			okey_is_seven_pairs(black_box(&[
				Tile::Yellow01,
				Tile::Yellow01,
				Tile::Red02,
				Tile::Red03,
				Tile::Yellow03,
				Tile::Yellow03,
				Tile::Blue07,
				Tile::Blue13,
				Tile::Yellow05,
				Tile::Yellow05,
				Tile::Black06,
				Tile::Black06,
				Tile::Black07,
				Tile::Joker,
			]));
		});
	});

	group.bench_function("Bench false, 2 jokers", |b| {
		b.iter(|| {
			okey_is_seven_pairs(black_box(&[
				Tile::Yellow01,
				Tile::Yellow01,
				Tile::Red02,
				Tile::Red11,
				Tile::Yellow03,
				Tile::Yellow03,
				Tile::Blue13,
				Tile::Blue04,
				Tile::Yellow05,
				Tile::Yellow12,
				Tile::Joker,
				Tile::Black06,
				Tile::Black07,
				Tile::Joker,
			]));
		});
	});

	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
