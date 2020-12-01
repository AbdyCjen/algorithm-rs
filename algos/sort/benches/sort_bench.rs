use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use rand::prelude::*;
use sort::*;
use std::{collections::BinaryHeap, time::Instant};

fn bench_sort(b: &mut Bencher, sort_fn: fn(&mut [i32])) {
	#[allow(dead_code)]
	fn is_sorted<T: std::cmp::Ord, I: Iterator<Item = T> + Clone>(mut it: I) -> bool {
		it.next()
			.map(|mut prev| {
				it.all(move |mut o| {
					std::mem::swap(&mut o, &mut prev);
					o <= prev
				})
			})
			.unwrap_or(true)
	}

	let mut nums: Vec<i32> = (1..10000).collect();
	(1..100).for_each(|n| nums.push(n));
	b.iter_custom(move |iters| {
		let mut total_cost = Default::default();
		for _ in 0..iters {
			nums.shuffle(&mut rand::thread_rng());
			let start = Instant::now();
			sort_fn(&mut nums);
			total_cost += start.elapsed();
			// no need to check sorted in benches
			//assert!(is_sorted(nums.iter()));
		}
		total_cost
	})
}

fn bench_std_heap(b: &mut Bencher) {
	let mut nums: Vec<i32> = (1..10000).collect();
	(1..100).for_each(|n| nums.push(n));
	b.iter_custom(move |iters| {
		let mut total_cost = Default::default();
		for _ in 0..iters {
			let mut nums = nums.clone();
			nums.shuffle(&mut rand::thread_rng());
			let start = Instant::now();
			let heap = BinaryHeap::from(nums);
			nums = heap.into_sorted_vec();
			total_cost += start.elapsed();
			black_box(&nums);
		}
		total_cost
	})
}

#[rustfmt::skip]
fn bench_all(c: &mut Criterion) {
	c.bench_function("std::sort"          , |b| bench_sort(b , <[i32]>::sort));
	c.bench_function("std::sort_unstable" , |b| bench_sort(b , <[i32]>::sort_unstable) );
	c.bench_function("std::binary_heap"   , bench_std_heap);
	c.bench_function("merge_sort"         , |b| bench_sort(b , merge_sort::<i32>));
	c.bench_function("bottomup_sort"      , |b| bench_sort(b , bottomup_sort::<i32>));
	c.bench_function("quick_sort"         , |b| bench_sort(b , quick_sort::<i32>));
	c.bench_function("heap_sort"          , |b| bench_sort(b , heap_sort::<i32>));
	c.bench_function("select_sort"        , |b| bench_sort(b , select_sort::<i32>));
	c.bench_function("insert_sort"        , |b| bench_sort(b , insert_sort::<i32>));
}

criterion_group!(benches, bench_all);

criterion_main!(benches);
