#![feature(test)]
use std::cmp::Ordering;

pub fn merge_sort<T>(nums: &mut [T])
	where T: std::cmp::Ord + Clone
{
	fn merge_into<T: std::cmp::Ord>(nums: &mut [T], nums_lower: Vec<T>) {
		let mut i = nums_lower.len();
		let mut nums_lower = nums_lower.into_iter().peekable();
		for k in 0..nums.len() {
			match (nums.get(i), nums_lower.peek()) {
				(_, None) => {}
				(Some(n), Some(p)) if n < p => {
					nums.swap(i, k);
					i += 1;
				}
				(_, Some(_)) => nums[k] = nums_lower.next().unwrap(),
			}
		}
	}
	if nums.len() > 1 {
		let (lower, upper) = nums.split_at_mut(nums.len() / 2);
		merge_sort(lower);
		merge_sort(upper);
		let lower: Vec<_> = lower.to_vec();
		merge_into(nums, lower);
	}
}

pub fn insert_sort<T>(nums: &mut [T])
	where T: std::cmp::Ord
{
	for i in 1..nums.len() {
		let (l, r) = nums[..i + 1].split_at_mut(i);
		let mut prev = &mut r[0];
		for b in l.into_iter().rev() {
			if b < prev {
				break;
			}
			std::mem::swap(prev, b);
			prev = b;
		}
	}
}

pub fn select_sort<T>(nums: &mut [T])
	where T: std::cmp::Ord
{
	if nums.len() <= 1 {
		return;
	}
	for i in 0..(nums.len() - 1) {
		let (l, r) = nums[i..].split_at_mut(1);
		r.into_iter()
			.min()
			.map(|min| {
				if *min < l[0] {
					std::mem::swap(&mut l[0], min)
				}
			});
	}
}

pub fn quick_sort<T>(iv: &mut [T])
	where T: std::cmp::Ord
{
	if iv.len() <= 1 {
		return;
	}
	let (mut i, mut j, mut k) = (0, 0, iv.len() - 1);
	{
		let (o, iv) = iv.split_at_mut(1);
		while j < k {
			match iv[j].cmp(&o[0]) {
				Ordering::Less => {
					iv.swap(j, i);
					i += 1;
					j += 1;
				}
				Ordering::Greater => {
					iv.swap(j, k - 1);
					k -= 1;
				}
				Ordering::Equal => j += 1,
			}
		}
	}
	iv.swap(0, i);
	quick_sort(&mut iv[..i]);
	quick_sort(&mut iv[k + 1..]);
}

pub fn heap_sort<T>(nums: &mut [T])
	where T: std::cmp::Ord
{
	fn shift_down<T: std::cmp::Ord>(tree: &mut [T], mut i: usize) {
		let child_idxs = |i: usize| (i * 2 + 1, i * 2 + 2);
		while i < tree.len() {
			let (l, r) = child_idxs(i);
			let larger_child = match (tree.get(l), tree.get(r)) {
				(None, _) => return,
				(Some(lv), Some(rv)) if lv < rv => r,
				(Some(_), _) => l,
			};
			if tree.get(i) < tree.get(larger_child) {
				tree.swap(i, larger_child);
				i = larger_child;
			} else {
				return;
			}
		}
	}

	for i in (0..nums.len()).rev() {
		shift_down(nums, i);
	}

	for i in (1..nums.len()).rev() {
		nums.swap(0, i);
		shift_down(&mut nums[0..i], 0);
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use rand::prelude::*;
	#[test]
	fn it_works() {
		check_sort(<[i32]>::sort);
		println!("std sort test ok!");

		check_sort(<[i32]>::sort_unstable);
		println!("std sort_unstable test ok!");

		check_sort(quick_sort::<i32>);
		println!("quick sort test ok!");

		check_sort(merge_sort::<i32>);
		println!("merge sort test ok!");

		check_sort(select_sort::<i32>);
		println!("select sort test ok!");

		check_sort(insert_sort::<i32>);
		println!("insert sort test ok!");

		check_sort(heap_sort::<i32>);
		println!("heap sort test ok!");
	}

	fn check_sort(sort_fn: fn(&mut [i32])) {
		let mut nums: Vec<i32> = (1..10000).collect();
		(1..100).for_each(|n| nums.push(n));
		nums.shuffle(&mut rand::thread_rng());
		fn is_sorted<T: std::cmp::Ord, I: Iterator<Item = T>>(mut it: I) -> bool {
			it.next()
			  .map(|mut prev| {
				  it.all(move |mut o| {
					    std::mem::swap(&mut o, &mut prev);
					    o <= prev
				    })
			  })
			  .unwrap_or(true)
		}
		//nums.iter().for_each(|n| print!("{},", n)); println!("");
		sort_fn(&mut nums);
		//nums.iter().for_each(|n| print!("{},", n)); println!("");
		assert!(is_sorted(nums.iter()));
		let mut nums = vec![0; 100];
		nums[0] = 1;
		sort_fn(&mut nums);
		assert!(is_sorted(nums.iter()));
	}

	//#[cfg(nightly)]
	mod bench {
		extern crate test;
		use super::*;
		use test::Bencher;
		#[bench]
		// TODO: 再写一个bench_sort的函数, check_sort有生成待排序和检查排序结果的成本;
		fn bench_std_sort(b: &mut Bencher) {
			b.iter(|| check_sort(<[i32]>::sort))
		}

		#[bench]
		fn bench_std_sort_unstable(b: &mut Bencher) {
			b.iter(|| check_sort(<[i32]>::sort_unstable))
		}

		#[bench]
		fn bench_merge_sort(b: &mut Bencher) {
			b.iter(|| check_sort(merge_sort::<i32>))
		}

		#[bench]
		fn bench_quick_sort(b: &mut Bencher) {
			b.iter(|| check_sort(quick_sort::<i32>))
		}

		#[bench]
		fn bench_select_sort(b: &mut Bencher) {
			b.iter(|| check_sort(select_sort::<i32>))
		}

		#[bench]
		fn bench_insert_sort(b: &mut Bencher) {
			b.iter(|| check_sort(insert_sort::<i32>))
		}

		#[bench]
		fn bench_heap_sort(b: &mut Bencher) {
			b.iter(|| check_sort(heap_sort::<i32>))
		}
	}
}
