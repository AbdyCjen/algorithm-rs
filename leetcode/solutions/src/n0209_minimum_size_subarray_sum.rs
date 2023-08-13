/**
 * [209] Minimum Size Subarray Sum
 *
 * Given an array of positive integers nums and a positive integer target, return the minimal length of a <span data-keyword="subarray-nonempty">subarray</span> whose sum is greater than or equal to target. If there is no such subarray, return 0 instead.
 *  
 * <strong class="example">Example 1:
 *
 * Input: target = 7, nums = [2,3,1,2,4,3]
 * Output: 2
 * Explanation: The subarray [4,3] has the minimal length under the problem constraint.
 *
 * <strong class="example">Example 2:
 *
 * Input: target = 4, nums = [1,4,4]
 * Output: 1
 *
 * <strong class="example">Example 3:
 *
 * Input: target = 11, nums = [1,1,1,1,1,1,1,1]
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     1 <= target <= 10^9
 *     1 <= nums.length <= 10^5
 *     1 <= nums[i] <= 10^4
 *
 *  
 * Follow up: If you have figured out the O(n) solution, try coding another solution of which the time complexity is O(n log(n)).
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
		let (mut i, mut s, mut ans) = (0_u32, 0, u32::MAX);
		for (j, &n) in (0..).zip(&nums) {
			s += n;
			while s >= target {
				ans = ans.min(j - i + 1);
				s -= nums[i as usize];
				i += 1;
			}
		}
		ans.wrapping_add(1).saturating_sub(1) as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_209() {
		assert_eq!(
			Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
			0
		);
		assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
		assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
	}
}
