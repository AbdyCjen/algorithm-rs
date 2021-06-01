#![allow(clippy::all)]
/**
 * [1776] Minimum Operations to Reduce X to Zero
 *
 * You are given an integer array nums and an integer x. In one operation, you can either remove the leftmost or the rightmost element from the array nums and subtract its value from x. Note that this modifies the array for future operations.
 *
 * Return the minimum number of operations to reduce x to exactly 0 if it's possible, otherwise, return -1.
 *
 *  
 * Example 1:
 *
 *
 * Input: nums = [1,1,4,2,3], x = 5
 * Output: 2
 * Explanation: The optimal solution is to remove the last two elements to reduce x to zero.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [5,6,7,8,9], x = 4
 * Output: -1
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [3,2,20,1,1,3], x = 10
 * Output: 5
 * Explanation: The optimal solution is to remove the last three elements and the first two elements (5 operations in total) to reduce x to zero.
 *
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^4
 * 	1 <= x <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

// return the maximum window size that sums up to (nums.sum() - x)
#[allow(dead_code)]
impl Solution {
	pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
		let total: i32 = nums.iter().sum();
		if total == x {
			return nums.len() as i32;
		} else if total < x {
			return -1;
		}
		let target: i32 = total - x;

		let (mut win_sum, mut max_win) = (0, 0);
		let mut l = 0;
		for (r, &v) in nums.iter().enumerate() {
			win_sum += v;
			while win_sum > target {
				win_sum -= nums[l];
				l += 1;
			}
			if win_sum == target {
				max_win = max_win.max(r - l + 1);
			}
		}

		if max_win > 0 {
			(nums.len() - max_win) as i32
		} else {
			-1
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1776() {
		assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
		assert_eq!(Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
		assert_eq!(Solution::min_operations(vec![5, 6, 7, 8, 9], 4), -1);
		assert_eq!(Solution::min_operations(vec![1, 1, 3, 2, 5], 5), 1);
		assert_eq!(Solution::min_operations(vec![500, 1, 4, 2, 3], 500), 1);
		assert_eq!(Solution::min_operations(vec![1, 4, 2, 3, 500], 500), 1);
		assert_eq!(Solution::min_operations(vec![1, 1], 3), -1);
	}
}
