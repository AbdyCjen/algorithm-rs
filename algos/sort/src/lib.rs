use std::cmp::Ordering;

fn merge_into<T>(nums: &mut [T], nums_lower: &[T])
	where T: std::cmp::Ord + Copy
{
	let (mut i, mut j) = (nums_lower.len(), 0);
	for k in 0..nums.len() {
		match (nums.get(i), nums_lower.get(j)) {
			(_, None) => {}
			(Some(n), Some(p)) => {
				if n < p {
					nums[k] = *n;
					i += 1;
				} else {
					nums[k] = *p;
					j += 1;
				}
			}
			(None, Some(p)) => {
				nums[k] = *p;
				j += 1;
			}
		}
	}
}

pub fn merge_sort<T>(nums: &mut [T])
	where T: std::cmp::Ord + Copy
{
	if nums.len() > 1 {
		let (lower, upper) = nums.split_at_mut(nums.len() / 2);
		merge_sort(lower);
		merge_sort(upper);
		let lower: Vec<_> = lower.to_vec();
		merge_into(nums, &lower);
	}
}

pub fn qsort<T>(iv: &mut [T])
	where T: std::cmp::Ord + Copy
{
	if iv.len() <= 1 {
		return;
	}
	let o = iv[0];
	let (mut i, mut j, mut k) = (0, 0, iv.len() - 1);
	while j <= k {
		match iv[j].cmp(&o) {
			Ordering::Less => {
				iv.swap(j, i);
				i += 1;
				j += 1;
			}
			Ordering::Greater => {
				iv.swap(j, k);
				k -= 1;
			}
			Ordering::Equal => j += 1,
		}
	}
	qsort(&mut iv[..i]);
	qsort(&mut iv[k + 1..]);
	return;
}

#[cfg(test)]
mod test {
	use rand::prelude::*;
	#[test]
	fn it_works() {
		check_sort(super::qsort::<i32>);
		println!("quick sort test ok!");
		check_sort(super::merge_sort::<i32>);
		println!("merge sort test ok!");
	}
	fn check_sort(sort_fn: fn(&mut [i32])) {
		let mut nums: Vec<i32> = (1..10000).collect();
		nums.shuffle(&mut rand::thread_rng());
		//nums.iter().for_each(|n| print!("{},", n)); println!("");
		sort_fn(&mut nums);
		//nums.iter().for_each(|n| print!("{},", n)); println!("");
		assert!(
		        nums.iter()
		            .fold((true, std::i32::MIN), |(sorted, prev), n| {
			            (sorted && prev < *n, *n)
		            })
		            .0
		);
	}
}
