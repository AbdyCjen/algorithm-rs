use criterion::{criterion_group, criterion_main, Criterion};
use skiplist::*;
use std::{collections::LinkedList, time::Instant};

const TEST_RANGE1: std::ops::Range<i32> = 0..1_000_000;
const TEST_RANGE2: std::ops::Range<i32> = 0..10_000_i32;

fn bench_insert(c: &mut Criterion) {
    let mut test_by_range = |promt, test_range: std::ops::Range<i32>| {
        c.bench_function(promt, |b| {
            b.iter_custom(|iters| {
                let mut total_cost = Default::default();
                for _ in 0..iters {
                    let mut list: Skiplist<_> = Default::default();

                    let start = Instant::now();
                    for _ in test_range.clone() {
                        list.insert(rand::random::<i32>());
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

fn bench_list_insert(c: &mut Criterion) {
    let mut test_by_range = |promt, test_range: std::ops::Range<i32>| {
        c.bench_function(promt, |b| {
            b.iter_custom(|iters| {
                let mut total_cost = Default::default();
                for _ in 0..iters {
                    let mut list = LinkedList::new();
                    let test_range = test_range.clone();
                    let start = Instant::now();
                    for _ in test_range {
                        list.push_back(rand::random::<i32>());
                    }
                    total_cost += start.elapsed();
                }
                total_cost
            })
        });
    };

    test_by_range("1_000_000 list insert", TEST_RANGE1);
    test_by_range("10_000 list insert", TEST_RANGE2);
}

criterion_group!(
    name = skiplist_insert_benches;
    config = Criterion::default().sample_size(10);
    targets =
        bench_insert,
        bench_list_insert,
);
criterion_main!(skiplist_insert_benches);
