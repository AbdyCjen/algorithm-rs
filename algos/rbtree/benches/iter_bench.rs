use bst_util::bst_benches;
use criterion::{criterion_group, criterion_main};
use rbtree::*;

//criterion_group!(benches, bench_iter, bench_std_iter, bench_insert, bench_std_insert);
criterion_group!(
	benches,
	bst_benches::bench_iter::<RbTree<i32>>,
	bst_benches::bench_std_iter,
	bst_benches::bench_insert::<RbTree<i32>>,
	bst_benches::bench_std_insert
);
criterion_main!(benches);
