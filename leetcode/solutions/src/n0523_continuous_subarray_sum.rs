/**
 * [523] Continuous Subarray Sum
 *
 * Given an integer array nums and an integer k, return true if nums has a continuous subarray of size at least two whose elements sum up to a multiple of k, or false otherwise.
 * An integer x is a multiple of k if there exists an integer n such that x = n * k. 0 is always a multiple of k.
 *  
 * Example 1:
 *
 * Input: nums = [23,<u>2,4</u>,6,7], k = 6
 * Output: true
 * Explanation: [2, 4] is a continuous subarray of size 2 whose elements sum up to 6.
 *
 * Example 2:
 *
 * Input: nums = [<u>23,2,6,4,7</u>], k = 6
 * Output: true
 * Explanation: [23, 2, 6, 4, 7] is an continuous subarray of size 5 whose elements sum up to 42.
 * 42 is a multiple of 6 because 42 = 7 * 6 and 7 is an integer.
 *
 * Example 3:
 *
 * Input: nums = [23,2,6,4,7], k = 13
 * Output: false
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     0 <= nums[i] <= 10^9
 *     0 <= sum(nums[i]) <= 2^31 - 1
 *     1 <= k <= 2^31 - 1
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
		use std::collections::HashSet;
		let mut mods = HashSet::with_capacity(nums.len().min(k as usize));

		let (mut prv, mut sum) = (0, nums[0] % k);
		for v in nums.into_iter().skip(1) {
			mods.insert(prv);
			prv = sum;
			sum = (sum + v) % k;
			if mods.contains(&sum) {
				return true;
			}
		}
		false
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_523() {
		assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
		assert!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
		assert!(!Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 13));
		assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 6], 7));
		assert!(!Solution::check_subarray_sum(vec![0], 1));
		assert!(Solution::check_subarray_sum(vec![23, 0, 0], 6));
	}
}
