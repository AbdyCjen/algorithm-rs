use immutable_avl::*;
use bst_util::bst_benches;
use criterion::{criterion_group, criterion_main};

fn bench_iter_avl(c: &mut criterion::Criterion) {
	let mut bench_fn = bst_benches::bench_iter_generator::<AvlTree<i32>>();
	bench_fn(c, bst_benches::TEST_RANGE2, "10_000 avl iter");
	bench_fn(c, bst_benches::TEST_RANGE1, "1_000_000 avl iter");
}

criterion_group!(
	benches,
	bench_iter_avl,
	bst_benches::bench_std_iter,
	//bst_benches::bench_insert::<AvlTree<i32>>,
	bst_benches::bench_std_insert
);
criterion_main!(benches);
