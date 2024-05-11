use criterion::criterion_main;

mod benchmarks;

criterion_main! {
	benchmarks::okey::test_pairs::benches,
	benchmarks::okey::test_set::benches,
	benchmarks::okey::test_run::benches,
	benchmarks::okey::test_win::benches,
}
