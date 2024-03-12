use criterion::{black_box, criterion_group, criterion_main, Criterion};
use skiplist::*;
use std::collections::LinkedList;

const TEST_RANGE1: std::ops::Range<i32> = 0..1_000_000;
const TEST_RANGE2: std::ops::Range<i32> = 0..10_000_i32;

fn bench_iter(c: &mut Criterion) {
    let mut test_by_range = |test_range: std::ops::Range<i32>, promt: &str| {
        let mut list = Skiplist::default();
        for _ in test_range {
            list.insert(rand::random::<i32>());
        }
        c.bench_function(promt, |b| {
            b.iter(|| {
                black_box((&list).into_iter().count());
            })
        });
    };
    test_by_range(TEST_RANGE1, "1_000_000 iter");
    test_by_range(TEST_RANGE2, "10_000 iter");
}

fn bench_list_iter(c: &mut Criterion) {
    let mut test_by_range = |test_range: std::ops::Range<i32>, promt: &str| {
        let mut list = LinkedList::new();
        black_box(&list);
        for _ in test_range {
            list.push_back(rand::random::<i32>());
            list.push_back(rand::random::<i32>());
        }
        c.bench_function(promt, |b| {
            b.iter(|| {
                black_box(list.iter().max());
            })
        });
    };

    test_by_range(TEST_RANGE1, "1_000_000 list_iter");
    test_by_range(TEST_RANGE2, "10_000 list_iter");
}

criterion_group!(
    name = skiplist_iter_benches;
    config = Criterion::default().sample_size(10);
    targets =
        bench_list_iter,
        bench_iter,
);
criterion_main!(skiplist_iter_benches);
