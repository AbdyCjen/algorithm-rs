/**
 * [665] Non-decreasing Array
 *
 * Given an array nums with n integers, your task is to check if it could become non-decreasing by modifying at most one element.
 * We define an array is non-decreasing if nums[i] <= nums[i + 1] holds for every i (0-based) such that (0 <= i <= n - 2).
 *  
 * Example 1:
 *
 * Input: nums = [4,2,3]
 * Output: true
 * Explanation: You could modify the first 4 to 1 to get a non-decreasing array.
 *
 * Example 2:
 *
 * Input: nums = [4,2,1]
 * Output: false
 * Explanation: You can't get a non-decreasing array by modify at most one element.
 *
 *  
 * Constraints:
 *
 *     n == nums.length
 *     1 <= n <= 10^4
 *     -10^5 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn check_possibility(nums: Vec<i32>) -> bool {
		match nums.windows(2).position(|w| w[0] > w[1]) {
			None => true,
			Some(0) => nums[1..].windows(2).all(|w| w[0] <= w[1]),
			Some(i) if i + 2 >= nums.len() => true,
			Some(i) => {
				(nums[i - 1] <= nums[i + 1] || nums[i] <= nums[i + 2])
					&& nums[i + 1..].windows(2).all(|w| w[0] <= w[1])
			}
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_665() {
		assert!(Solution::check_possibility(vec![4, 2, 3]));
		assert!(Solution::check_possibility(vec![1, 2, 7, 4, 5]));
		assert!(!Solution::check_possibility(vec![4, 2, 1]));
		assert!(Solution::check_possibility(vec![1, 4, 1, 2]));
		assert!(Solution::check_possibility(vec![1, 2, -1]));
	}
}
