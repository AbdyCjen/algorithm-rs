use bst::{BSTree, Iter};
use criterion::{black_box, Criterion};
use std::{collections::BTreeSet, time::Instant};

const TEST_RANGE1: std::ops::Range<i32> = 0..1_000_000;
const TEST_RANGE2: std::ops::Range<i32> = 0..10_000_i32;

// fuck lifetime here
pub fn bench_iter<Tree>(c: &mut Criterion)
where Tree: BSTree<Item = i32> + std::default::Default + 'static {
	let mut test_by_range = |test_range: std::ops::Range<i32>, promt: &str| {
		let mut tr: Tree = Default::default();
		for _ in test_range {
			tr.insert(rand::random::<i32>());
		}
		c.bench_function(promt, |b| {
			b.iter(|| {
				for i in Iter::from_tree(&tr) {
					black_box(i);
				}
			})
		});
	};
	test_by_range(TEST_RANGE1, "1_000_000 iter");
	test_by_range(TEST_RANGE2, "10_000 iter");
}

pub fn bench_std_iter(c: &mut Criterion) {
	let mut test_by_range = |test_range: std::ops::Range<i32>, promt: &str| {
		let mut bt: BTreeSet<_> = BTreeSet::new();
		for _ in test_range {
			bt.insert(rand::random::<i32>());
		}
		c.bench_function(promt, |b| {
			b.iter(|| {
				for i in &bt {
					black_box(i);
				}
			})
		});
	};

	test_by_range(TEST_RANGE1, "1_000_000 std iter");
	test_by_range(TEST_RANGE2, "10_000 std iter");
}

pub fn bench_insert<Tree>(c: &mut Criterion)
where Tree: BSTree<Item = i32> + std::default::Default + 'static {
	let mut test_by_range = |promt, test_range: std::ops::Range<i32>| {
		c.bench_function(promt, |b| {
			b.iter_custom(|iters| {
				let mut total_cost = Default::default();
				for _ in 0..iters {
					let mut rbt: Tree = Default::default();

					let start = Instant::now();
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

pub fn bench_std_insert(c: &mut Criterion) {
	let mut test_by_range = |promt, test_range: std::ops::Range<i32>| {
		c.bench_function(promt, |b| {
			b.iter_custom(|iters| {
				let mut total_cost = Default::default();
				for _ in 0..iters {
					let mut bt = BTreeSet::new();
					let start = Instant::now();
					for _ in test_range.clone() {
						bt.insert(rand::random::<i32>());
					}
					total_cost += start.elapsed();
				}
				total_cost
			})
		});
	};

	test_by_range("1_000_000 std insert", TEST_RANGE1);
	test_by_range("10_000 std insert", TEST_RANGE2);
}
