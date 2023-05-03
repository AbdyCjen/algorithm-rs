/**
 * [2444] Count Subarrays With Fixed Bounds
 *
 * You are given an integer array nums and two integers minK and maxK.
 * A fixed-bound subarray of nums is a subarray that satisfies the following conditions:
 *
 *     The minimum value in the subarray is equal to minK.
 *     The maximum value in the subarray is equal to maxK.
 *
 * Return the number of fixed-bound subarrays.
 * A subarray is a contiguous part of an array.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,3,5,2,7,5], minK = 1, maxK = 5
 * Output: 2
 * Explanation: The fixed-bound subarrays are [1,3,5] and [1,3,5,2].
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,1,1,1], minK = 1, maxK = 1
 * Output: 10
 * Explanation: Every subarray of nums is a fixed-bound subarray. There are 10 possible subarrays.
 *
 *  
 * Constraints:
 *
 *     2 <= nums.length <= 10^5
 *     1 <= nums[i], minK, maxK <= 10^6
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
		let (mut min, mut max, mut bad) = (-1, -1, -1);
		nums.into_iter()
			.zip(0..)
			.map(|(n, i)| {
				if n == min_k {
					min = i;
				}
				if n == max_k {
					max = i;
				}
				if !(min_k..=max_k).contains(&n) {
					bad = i;
				}
				(min.min(max) - bad).max(0)
			})
			.sum()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2444() {
		assert_eq!(Solution::count_subarrays(vec![1, 1, 1, 1], 1, 1), 10);
		assert_eq!(Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5), 2);
	}
}
