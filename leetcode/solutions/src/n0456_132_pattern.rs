/**
 * [456] 132 Pattern
 *
 * Given an array of n integers nums, a 132 pattern is a subsequence of three integers nums[i], nums[j] and nums[k] such that i < j < k and nums[i] < nums[k] < nums[j].
 * Return true if there is a 132 pattern in nums, otherwise, return false.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,4]
 * Output: false
 * Explanation: There is no 132 pattern in the sequence.
 *
 * Example 2:
 *
 * Input: nums = [3,1,4,2]
 * Output: true
 * Explanation: There is a 132 pattern in the sequence: [1, 4, 2].
 *
 * Example 3:
 *
 * Input: nums = [-1,3,2,0]
 * Output: true
 * Explanation: There are three 132 patterns in the sequence: [-1, 3, 2], [-1, 3, 0] and [-1, 2, 0].
 *
 *  
 * Constraints:
 *
 *     n == nums.length
 *     1 <= n <= 2 * 10^5
 *     -10^9 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn find132pattern(nums: Vec<i32>) -> bool {
		let mut s2 = i32::MIN;

		let mut mono_stk = vec![];
		for x in nums.into_iter().rev() {
			if x < s2 {
				return true;
			}
			use std::cmp::Reverse;
			while mono_stk.last().map(Reverse) > Some(Reverse(&x)) {
				s2 = mono_stk.pop().unwrap();
			}
			mono_stk.push(x);
		}
		false
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_456() {
		assert!(!Solution::find132pattern(vec![1, 2, 3, 4]));
		assert!(Solution::find132pattern(vec![3, 1, 4, 2]));
		assert!(Solution::find132pattern(vec![-1, 3, 2, 0]));
		assert!(!Solution::find132pattern(vec![-2, 1, -2]));
	}
}
