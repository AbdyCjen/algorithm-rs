use bst_util::bst_benches;
use criterion::{criterion_group, criterion_main};
use rbtree::*;

fn bench_iter_rbtree(c: &mut criterion::Criterion) {
	let mut bench_fn = bst_benches::bench_iter_generator::<RBTree<i32>>();
	bench_fn(c, bst_benches::TEST_RANGE1, "1_000_000 rbtree iter");
	bench_fn(c, bst_benches::TEST_RANGE2, "10_000 rbtree iter");
}

criterion_group!(
	benches,
<<<<<<< Updated upstream
	bst_benches::bench_iter::<RbTree<i32>>,
||||||| constructed merge base
	bst_benches::bench_iter::<RBTree<i32>>,
=======
	bench_iter_rbtree,
>>>>>>> Stashed changes
	bst_benches::bench_std_iter,
	bst_benches::bench_insert::<RbTree<i32>>,
	bst_benches::bench_std_insert
);
criterion_main!(benches);
