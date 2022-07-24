use bst::BsTree;
use criterion::{black_box, Criterion};
use std::{collections::BTreeSet, time::Instant};

pub const TEST_RANGE1: std::ops::Range<i32> = 0..1_000_000;
pub const TEST_RANGE2: std::ops::Range<i32> = 0..10_000_i32;

pub fn bench_iter_generator<Tree>(
) -> impl for<'r, 's> FnMut(&'r mut Criterion, std::ops::Range<i32>, &'s str)
where
	Tree: std::iter::FromIterator<i32>,
	for<'a> &'a Tree: std::iter::IntoIterator<Item = &'a i32>,
{
	|c: &mut Criterion, test_range: std::ops::Range<i32>, promt: &str| {
		c.bench_function(promt, |b| {
			let tr: Tree = test_range.clone().map(|_| rand::random::<i32>()).collect();
			b.iter(|| {
				for i in &tr {
					black_box(i);
				}
			})
		});
	}
}

pub fn bench_std_iter(c: &mut Criterion) {
	let mut bench_fn = bench_iter_generator::<BTreeSet<i32>>();
	bench_fn(c, TEST_RANGE1, "1_000_000 std iter");
	bench_fn(c, TEST_RANGE2, "10_000 std iter");
}

pub fn bench_insert<Tree>(c: &mut Criterion)
where
	Tree: BsTree<Item = i32> + std::default::Default + 'static,
{
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
