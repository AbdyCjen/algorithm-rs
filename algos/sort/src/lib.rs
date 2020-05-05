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
		(1..100).for_each(|n| nums.push(n));
		nums.shuffle(&mut rand::thread_rng());
		//nums.iter().for_each(|n| print!("{},", n)); println!("");
		sort_fn(&mut nums);
		//nums.iter().for_each(|n| print!("{},", n)); println!("");
		assert!(
		        nums.iter()
		            .fold((true, &std::i32::MIN), |(sorted, prev), n| {
			            (sorted && prev <= n, n)
		            })
		            .0
		);
		let mut nums = vec![0; 100];
		nums[0] = 1;
		sort_fn(&mut nums);
		assert!(
		        nums.iter()
		            .fold((true, &std::i32::MIN), |(sorted, prev), n| {
			            (sorted && prev <= n, n)
		            })
		            .0
		);
		println!("OK");
	}
}
