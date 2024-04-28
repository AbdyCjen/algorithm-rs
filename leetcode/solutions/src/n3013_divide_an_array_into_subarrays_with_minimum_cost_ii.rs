/**
 * [3013] Divide an Array Into Subarrays With Minimum Cost II
 *
 * You are given a 0-indexed array of integers nums of length n, and two positive integers k and dist.
 * The cost of an array is the value of its first element. For example, the cost of [1,2,3] is 1 while the cost of [3,4,1] is 3.
 * You need to divide nums into k disjoint contiguous <span data-keyword="subarray-nonempty">subarrays</span>, such that the difference between the starting index of the second subarray and the starting index of the kth subarray should be less than or equal to dist. In other words, if you divide nums into the subarrays nums[0..(i1 - 1)], nums[i1..(i2 - 1)], ..., nums[ik-1..(n - 1)], then ik-1 - i1 <= dist.
 * Return the minimum possible sum of the cost of these subarrays.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,3,2,6,4,2], k = 3, dist = 3
 * Output: 5
 * Explanation: The best possible way to divide nums into 3 subarrays is: [1,3], [2,6,4], and [2]. This choice is valid because ik-1 - i1 is 5 - 2 = 3 which is equal to dist. The total cost is nums[0] + nums[2] + nums[5] which is 1 + 2 + 2 = 5.
 * It can be shown that there is no possible way to divide nums into 3 subarrays at a cost lower than 5.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [10,1,2,2,2,1], k = 4, dist = 3
 * Output: 15
 * Explanation: The best possible way to divide nums into 4 subarrays is: [10], [1], [2], and [2,2,1]. This choice is valid because ik-1 - i1 is 3 - 1 = 2 which is less than dist. The total cost is nums[0] + nums[1] + nums[2] + nums[3] which is 10 + 1 + 2 + 2 = 15.
 * The division [10], [1], [2,2,2], and [1] is not valid, because the difference between ik-1 and i1 is 5 - 1 = 4, which is greater than dist.
 * It can be shown that there is no possible way to divide nums into 4 subarrays at a cost lower than 15.
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [10,8,18,9], k = 3, dist = 1
 * Output: 36
 * Explanation: The best possible way to divide nums into 4 subarrays is: [10], [8], and [18,9]. This choice is valid because ik-1 - i1 is 2 - 1 = 1 which is equal to dist.The total cost is nums[0] + nums[1] + nums[2] which is 10 + 8 + 18 = 36.
 * The division [10], [8,18], and [9] is not valid, because the difference between ik-1 and i1 is 3 - 1 = 2, which is greater than dist.
 * It can be shown that there is no possible way to divide nums into 3 subarrays at a cost lower than 36.
 *
 *  
 * Constraints:
 *
 *     3 <= n <= 10^5
 *     1 <= nums[i] <= 10^9
 *     3 <= k <= n
 *     k - 2 <= dist <= n - 2
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::*;
impl Solution {
	pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
		let mut win = BinaryHeap::<(i32, i32)>::new();
		let mut avail = win.clone();
		let mut ans = i64::MAX;
		let mut s = 0_i64;
		let mut picked = vec![false; nums.len()];

		for (j, &n) in (1..).zip(&nums[1..]) {
			let i = j - dist - 1;
			if j > dist && picked[i as usize] {
				s -= nums[i as usize] as i64;
				let (nn, jj) = Self::get_max(&mut avail, i);
				avail.pop();
				s -= nn as i64;
				picked[jj as usize] = true;
				win.push((-nn, jj));
			}
			if win.len() as i32 >= k - 2 {
				ans = ans.min(s + n as i64);
			}

			if win.len() < k as usize - 2 {
				win.push((n, j));
				picked[j as usize] = true;
				s += n as i64;
			} else if n < Self::get_max(&mut win, i).0 {
				let (mm, jj) = win.pop().unwrap();
				picked[jj as usize] = false;
				avail.push((-mm, jj));
				s += (n - mm) as i64;
				win.push((n, j));
				picked[j as usize] = true;
			} else {
				avail.push((-n, j));
			}
		}
		ans + nums[0] as i64
	}
	fn get_max(bh: &mut BinaryHeap<(i32, i32)>, i: i32) -> (i32, i32) {
		while bh.peek().unwrap().1 <= i {
			bh.pop();
		}
		*bh.peek().unwrap()
	}
	pub fn minimum_cost1(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
		let mut cnts = BTreeMap::<i32, i32>::new();
		let mut greater = cnts.clone();
		let mut i = 1;
		let mut s = 0_i64;
		let mut len = 0;
		let mut ans = i64::MAX;
		for (j, &n) in (1..).zip(&nums[1..]) {
			if j - i > dist {
				let prv = nums[i as usize];
				if greater.contains_key(&prv) {
					Self::remove(&mut greater, prv);
				} else {
					s -= prv as i64;
					Self::remove(&mut cnts, prv);
					let mm = Self::pop_min(&mut greater);
					s += mm as i64;
					*cnts.entry(mm).or_insert(0) += 1;
				}
				i += 1;
			}
			if len == k - 2 {
				ans = ans.min((n + nums[0]) as i64 + s);
			}

			if len < k - 2 {
				*cnts.entry(n).or_insert(0) += 1;
				s += n as i64;
				len += 1;
			} else if n < Self::max(&cnts) {
				let mm = Self::pop_max(&mut cnts);
				s = (n - mm) as i64;
				*cnts.entry(n).or_insert(0) += 1;
				*greater.entry(mm).or_insert(0) += 1;
			} else {
				*greater.entry(n).or_insert(0) += 1;
			}
		}
		ans
	}

	fn max(cnts: &BTreeMap<i32, i32>) -> i32 { *cnts.keys().next_back().unwrap() }

	fn pop_max(cnts: &mut BTreeMap<i32, i32>) -> i32 {
		let k = *cnts.keys().next_back().unwrap();
		Self::remove(cnts, k);
		k
	}

	fn pop_min(cnts: &mut BTreeMap<i32, i32>) -> i32 {
		let k = *cnts.keys().next().unwrap();
		Self::remove(cnts, k);
		k
	}

	fn remove(cnts: &mut BTreeMap<i32, i32>, k: i32) {
		*cnts.entry(k).or_insert(0) -= 1;
		if cnts[&k] == 0 {
			cnts.remove(&k);
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_3013() {
		assert_eq!(Solution::minimum_cost(vec![1, 3, 2, 6, 4, 2], 3, 3), 5);
		assert_eq!(Solution::minimum_cost(vec![10, 8, 18, 9], 3, 1), 36);
	}
}
