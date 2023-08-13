/**
 * [1425] Constrained Subsequence Sum
 *
 * Given an integer array nums and an integer k, return the maximum sum of a non-empty subsequence of that array such that for every two consecutive integers in the subsequence, nums[i] and nums[j], where i < j, the condition j - i <= k is satisfied.
 * A subsequence of an array is obtained by deleting some number of elements (can be zero) from the array, leaving the remaining elements in their original order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [10,2,-10,5,20], k = 2
 * Output: 37
 * Explanation: The subsequence is [10, 2, 5, 20].
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [-1,-2,-3], k = 1
 * Output: -1
 * Explanation: The subsequence must be non-empty, so we choose the largest number.
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [10,-2,-10,-5,20], k = 2
 * Output: 23
 * Explanation: The subsequence is [10, -2, -5, 20].
 *
 *  
 * Constraints:
 *
 *     1 <= k <= nums.length <= 10^5
 *     -10^4 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
		let mut dp = vec![];
		let mut win = std::collections::BTreeMap::new();
		for (i, &n) in (0..).zip(&nums) {
			if i > k {
				let e = win.get_mut(&dp[(i - k - 1) as usize]).unwrap();
				*e -= 1;
				if *e == 0 {
					win.remove(&dp[(i - k - 1) as usize]);
				}
			}
			let cur = win.keys().rev().next().copied().unwrap_or(0).max(0) + n;
			*win.entry(cur).or_insert(0) += 1;
			dp.push(cur);
		}
		dp.into_iter().max().unwrap()
	}
}

// submission codes end
