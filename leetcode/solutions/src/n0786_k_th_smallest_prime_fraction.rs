/**
 * [786] K-th Smallest Prime Fraction
 *
 * You are given a sorted integer array arr containing 1 and prime numbers, where all the integers of arr are unique. You are also given an integer k.
 * For every i and j where 0 <= i < j < arr.length, we consider the fraction arr[i] / arr[j].
 * Return the k^th smallest fraction considered. Return your answer as an array of integers of size 2, where answer[0] == arr[i] and answer[1] == arr[j].
 *  
 * Example 1:
 *
 * Input: arr = [1,2,3,5], k = 3
 * Output: [2,5]
 * Explanation: The fractions to be considered in sorted order are:
 * 1/5, 1/3, 2/5, 1/2, 3/5, and 2/3.
 * The third fraction is 2/5.
 *
 * Example 2:
 *
 * Input: arr = [1,7], k = 1
 * Output: [1,7]
 *
 *  
 * Constraints:
 *
 *     2 <= arr.length <= 1000
 *     1 <= arr[i] <= 3 * 10^4
 *     arr[0] == 1
 *     arr[i] is a prime number for i > 0.
 *     All the numbers of arr are unique and sorted in strictly increasing order.
 *     1 <= k <= arr.length * (arr.length - 1) / 2
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
		Self::kth_smallest_prime_fraction_3(arr, k)
	}
	// best performance but most lengthy
	pub fn kth_smallest_prime_fraction_3(arr: Vec<i32>, k: i32) -> Vec<i32> {
		let arr = arr.into_iter().map(|i| i as f64).collect::<Vec<_>>();
		let n = arr.len();
		let frac = |i: usize, j: usize| arr[i] / arr[n - j - 1];
		let mut l = frac(0, 0);
		let mut r = frac(n - 1, n - 1);

		loop {
			let m = (l + r) / 2.0;
			let (mut i, mut j) = (0, n);
			let (mut cnt, mut closest, mut diff) = (0, (0, 0), f64::MAX);
			while i < n {
				while j > 0 && frac(i, j - 1) > m {
					j -= 1;
				}
				// record the closest fraction parts
				if j > 0 {
					let cur_diff = m - frac(i, j - 1);
					if cur_diff < diff {
						diff = cur_diff;
						closest = (arr[i] as _, arr[n - j] as _);
					}
				}

				cnt += j as i32;
				i += 1;
			}
			match cnt.cmp(&k) {
				std::cmp::Ordering::Less => l = m,
				std::cmp::Ordering::Greater => r = m,
				std::cmp::Ordering::Equal => break vec![closest.0, closest.1],
			}
		}
	}
	pub fn kth_smallest_prime_fraction_2(arr: Vec<i32>, k: i32) -> Vec<i32> {
		#[derive(PartialEq, Eq)]
		struct Frac<'a>((i32, i32), &'a [i32]);
		impl<'a> PartialOrd for Frac<'a> {
			fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
				Some(self.cmp(other))
			}
		}
		impl<'a> Ord for Frac<'a> {
			fn cmp(&self, other: &Self) -> std::cmp::Ordering {
				let (s, o) = (&self.0, other.0);
				(s.0 * o.1).cmp(&(o.0 * s.1))
			}
		}

		let mut bh = std::collections::BinaryHeap::new();
		use std::cmp::Reverse;
		for &n in arr.iter() {
			bh.push(Reverse(Frac((1, n), &arr[1..])));
		}
		for _ in 1..k {
			if let Some(Reverse(Frac(cur, [n, next @ ..]))) = bh.pop() {
				bh.push(Reverse(Frac((*n, cur.1), next)));
			}
		}
		let ans = bh.pop().unwrap().0 .0;
		vec![ans.0, ans.1]
	}

	pub fn kth_smallest_prime_fraction_1(arr: Vec<i32>, k: i32) -> Vec<i32> {
		use std::collections::BinaryHeap;
		#[derive(PartialEq, Eq)]
		struct Frac(i32, i32);
		impl PartialOrd for Frac {
			fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
				Some(self.cmp(other))
			}
		}
		impl Ord for Frac {
			fn cmp(&self, other: &Self) -> std::cmp::Ordering {
				(self.0 * other.1).cmp(&(other.0 * self.1))
			}
		}

		let mut bh = BinaryHeap::new();

		for (m, i) in arr.iter().zip(1..) {
			for n in arr[i..].iter() {
				bh.push(Frac(*m, *n));
				if bh.len() > k as _ {
					bh.pop();
				}
			}
		}

		let ans = bh.pop().unwrap();
		vec![ans.0, ans.1]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_786() {
		assert_eq!(
			Solution::kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3),
			vec![2, 5]
		);
		assert_eq!(
			Solution::kth_smallest_prime_fraction(vec![1, 7], 1),
			vec![1, 7]
		);
	}
}
