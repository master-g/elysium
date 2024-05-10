use criterion::{black_box, criterion_group, Criterion};
use elysium_game::okey_mahjong::*;

pub fn benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group("okey_check_win");

	// assert!(okey_check_win(&okey_tiles_from_str("b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 j")));
	// assert!(okey_check_win(&okey_tiles_from_str("b1 b1 r2 r2 y5 y5 b6 b6 b7 b7 b8 b8 k9 k9")));
	// assert!(okey_check_win(&okey_tiles_from_str("b1 b1 b2 b2 k3 k3 k4 k4 r5 r5 y6 y6 k7 j")));
	// assert!(okey_check_win(&okey_tiles_from_str("b1 b1 b2 b2 k3 k3 k4 k4 r5 r5 y6 y6 j j")));
	// assert!(okey_check_win(&okey_tiles_from_str("r1 y1 b1 k1 k2 k3 r1 r2 r3 r4 r5 r6 r7 r8")));

	let vec1 = okey_tiles_from_str("b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 j");
	group.bench_function("Bench win, 13 run with 1 joker", |b| {
		b.iter(|| {
			okey_check_win(black_box(&vec1));
		});
	});

	let vec2 = okey_tiles_from_str("b1 b1 r2 r2 y5 y5 b6 b6 b7 b7 b8 b8 k9 k9");
	group.bench_function("Bench win, 7 pairs", |b| {
		b.iter(|| {
			okey_check_win(black_box(&vec2));
		});
	});

	let vec3 = okey_tiles_from_str("b1 b1 b2 b2 k3 k3 k4 k4 r5 r5 y6 y6 k7 j");
	group.bench_function("Bench win, 7 paris(1 joker)", |b| {
		b.iter(|| {
			okey_check_win(black_box(&vec3));
		});
	});

	let vec4 = okey_tiles_from_str("b1 b1 b2 b2 k3 k3 k4 k4 r5 r5 y6 y6 j j");
	group.bench_function("Bench win, 7 paris(2 jokers)", |b| {
		b.iter(|| {
			okey_check_win(black_box(&vec4));
		});
	});

	let vec5 = okey_tiles_from_str("r1 y1 b1 k1 k2 k3 r1 r2 r3 r4 r5 r6 r7 r8");
	group.bench_function("Bench win, 1 set, 2 runs", |b| {
		b.iter(|| {
			okey_check_win(black_box(&vec5));
		});
	});

	let vec6 = okey_tiles_from_str("r1 y2 b3 k4 k2 k3 r1 r2 r3 r4 r5 r6 r7 j");
	group.bench_function("Bench negative", |b| {
		b.iter(|| {
			okey_check_win(black_box(&vec6));
		});
	});
	let vec7 = okey_tiles_from_str("r1 r1 r5 r3 r2 b9 b8 b2 b3 y7 y5 y13 k8 k4");
	group.bench_function("Bench negative 2", |b| {
		b.iter(|| {
			okey_check_win(black_box(&vec7));
		});
	});
}

criterion_group!(benches, benchmark);
