use avl::*;
use bst_util::bst_benches;
use btree::*;
use criterion::{criterion_group, criterion_main, Criterion};
use rbtree::*;
use skiplist::*;
use treap::*;

fn bench_iter_avl(c: &mut criterion::Criterion) {
	let mut bench_fn = bst_benches::bench_iter_generator::<AvlTree<i32>>();
	bench_fn(c, bst_benches::TEST_RANGE2, "10k avl iter");
	bench_fn(c, bst_benches::TEST_RANGE1, "1m avl iter");
}
fn bench_iter_btree(c: &mut criterion::Criterion) {
	let mut bench_fn = bst_benches::bench_iter_generator::<BTree<i32>>();
	bench_fn(c, bst_benches::TEST_RANGE2, "10k btree iter");
	bench_fn(c, bst_benches::TEST_RANGE1, "1m btree iter");
}

fn bench_iter_treap(c: &mut criterion::Criterion) {
	let mut bench_fn = bst_benches::bench_iter_generator::<Treap<i32>>();
	bench_fn(c, bst_benches::TEST_RANGE2, "10k treap iter");
	bench_fn(c, bst_benches::TEST_RANGE1, "1m treap iter");
}

fn bench_iter_rbtree(c: &mut criterion::Criterion) {
	let mut bench_fn = bst_benches::bench_iter_generator::<RbTree<i32>>();
	bench_fn(c, bst_benches::TEST_RANGE2, "10k rbtree iter");
	bench_fn(c, bst_benches::TEST_RANGE1, "1m rbtree iter");
}
fn bench_iter_skiplist(c: &mut criterion::Criterion) {
	let mut bench_fn = bst_benches::bench_iter_generator::<SkipList<i32>>();
	bench_fn(c, bst_benches::TEST_RANGE2, "10k skiplist iter");
	bench_fn(c, bst_benches::TEST_RANGE1, "1m skiplist iter");
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

	test_by_range("1m insert", bst_benches::TEST_RANGE1);
	test_by_range("10k insert", bst_benches::TEST_RANGE2);
}

criterion_group!(
	benches,
	bench_iter_rbtree,
	bench_iter_avl,
	bench_iter_btree,
	bench_iter_treap,
	bench_iter_skiplist,
	bst_benches::bench_std_iter,
	bench_insert,
	bst_benches::bench_std_insert
);
criterion_main!(benches);
