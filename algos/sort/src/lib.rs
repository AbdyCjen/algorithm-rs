#![feature(test)]
use std::cmp::Ordering;

fn merge_into<T>(nums: &mut [T], nums_lower: Vec<T>)
	where T: std::cmp::Ord
{
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

pub fn merge_sort<T>(nums: &mut [T])
	where T: std::cmp::Ord + Clone
{
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

pub fn qsort<T>(iv: &mut [T])
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
	qsort(&mut iv[..i]);
	qsort(&mut iv[k + 1..]);
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

		check_sort(qsort::<i32>);
		println!("quick sort test ok!");

		check_sort(merge_sort::<i32>);
		println!("merge sort test ok!");

		check_sort(select_sort::<i32>);
		println!("select sort test ok!");

		check_sort(insert_sort::<i32>);
		println!("insert sort test ok!");
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
		println!("OK");
	}

	#[cfg(nightly)]
	mod bench {
		extern crate test;
		use super::*;
		use test::Bencher;
		#[bench]
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
			b.iter(|| check_sort(qsort::<i32>))
		}

		#[bench]
		fn bench_select_sort(b: &mut Bencher) {
			b.iter(|| check_sort(select_sort::<i32>))
		}

		#[bench]
		fn bench_insert_sort(b: &mut Bencher) {
			b.iter(|| check_sort(insert_sort::<i32>))
		}
	}
}
