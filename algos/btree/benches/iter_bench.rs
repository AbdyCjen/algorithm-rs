use bst_util::bst_benches;
use btree::*;
use criterion::{criterion_group, criterion_main, Criterion};
const TEST_RANGE1: std::ops::Range<i32> = 0..1_000_000;
const TEST_RANGE2: std::ops::Range<i32> = 0..10_000_i32;

pub fn bench_iter(c: &mut Criterion) {
	let mut test_by_range = |test_range: std::ops::Range<i32>, promt: &str| {
		//XXX: maybe not random ?
		let tr: BTree<_> = test_range.map(|_| rand::random::<i32>()).collect();
		c.bench_function(promt, |b| {
			b.iter(|| {
				for i in &tr {
					criterion::black_box(i);
				}
			})
		});
	};
	test_by_range(TEST_RANGE1, "1_000_000 btree iter");
	test_by_range(TEST_RANGE2, "10_000 btree iter");
}

pub fn bench_insert(c: &mut Criterion) {
	let mut test_by_range = |promt, test_range: std::ops::Range<i32>| {
		c.bench_function(promt, |b| {
			b.iter_custom(|iters| {
				let mut total_cost = Default::default();
				for _ in 0..iters {
					let mut rbt: BTree<_> = Default::default();

					let start = std::time::Instant::now();
					for _ in test_range.clone() {
						rbt.insert(rand::random::<i32>());
					}
					total_cost += start.elapsed();
				}
				total_cost
			})
		});
	};

	test_by_range("1_000_000 insert", TEST_RANGE1);
	test_by_range("10_000 insert", TEST_RANGE2);
}
criterion_group!(
	benches,
	bench_iter,
	bst_benches::bench_std_iter,
	bench_insert,
	bst_benches::bench_std_insert
);
criterion_main!(benches);
