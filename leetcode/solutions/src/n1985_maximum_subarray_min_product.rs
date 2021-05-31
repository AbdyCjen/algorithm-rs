/**
 * [1985] Maximum Subarray Min-Product
 *
 * The min-product of an array is equal to the minimum value in the array multiplied by the array's sum.
 *
 *     For example, the array [3,2,5] (minimum value is 2) has a min-product of 2 * (3+2+5) = 2 * 10 = 20.
 *
 * Given an array of integers nums, return the maximum min-product of any non-empty subarray of nums. Since the answer may be large, return it modulo 10^9 + 7.
 * Note that the min-product should be maximized before performing the modulo operation. Testcases are generated such that the maximum min-product without modulo will fit in a 64-bit signed integer.
 * A subarray is a contiguous part of an array.
 *  
 * Example 1:
 *
 * Input: nums = [1,<u>2,3,2</u>]
 * Output: 14
 * Explanation: The maximum min-product is achieved with the subarray [2,3,2] (minimum value is 2).
 * 2 * (2+3+2) = 2 * 7 = 14.
 *
 * Example 2:
 *
 * Input: nums = [2,<u>3,3</u>,1,2]
 * Output: 18
 * Explanation: The maximum min-product is achieved with the subarray [3,3] (minimum value is 3).
 * 3 * (3+3) = 3 * 6 = 18.
 *
 * Example 3:
 *
 * Input: nums = [3,1,<u>5,6,4</u>,2]
 * Output: 60
 * Explanation: The maximum min-product is achieved with the subarray [5,6,4] (minimum value is 4).
 * 4 * (5+6+4) = 4 * 15 = 60.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     1 <= nums[i] <= 10^7
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
		use std::iter;
		let sum: Box<[i64]> = iter::once(0)
			.chain(nums.iter().scan(0, |s, &y| {
				*s += y as i64;
				Some(*s)
			}))
			.collect();
		let mut b: Vec<_> = nums.into_iter().enumerate().collect();
		b.sort_unstable_by_key(|v| std::cmp::Reverse(v.1));

		let mut c: Vec<_> = vec![None; b.len()];

		(b.into_iter().fold(0, |acc, (i, v)| {
			let l: usize = c.get(i.saturating_sub(1)).copied().flatten().unwrap_or(i);
			let r = c.get(i + 1).copied().flatten().unwrap_or(i);
			c[l] = Some(r);
			c[r] = Some(l);

			acc.max((sum[r + 1] - sum[l]) * v as i64)
		}) % (1e9 + 7.0) as i64) as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1985() {
		assert_eq!(Solution::max_sum_min_product(vec![1, 2, 3, 2]), 14);
		assert_eq!(Solution::max_sum_min_product(vec![2, 3, 3, 1, 2]), 18);
		assert_eq!(Solution::max_sum_min_product(vec![3, 1, 5, 6, 4, 2]), 60);
	}
}
