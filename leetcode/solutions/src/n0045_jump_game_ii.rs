/**
 * [45] Jump Game II
 *
 * Given an array of non-negative integers nums, you are initially positioned at the first index of the array.
 * Each element in the array represents your maximum jump length at that position.
 * Your goal is to reach the last index in the minimum number of jumps.
 * You can assume that you can always reach the last index.
 *  
 * Example 1:
 *
 * Input: nums = [2,3,1,1,4]
 * Output: 2
 * Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.
 *
 * Example 2:
 *
 * Input: nums = [2,3,0,1,4]
 * Output: 2
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 1000
 *     0 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn jump(mut nums: Vec<i32>) -> i32 {
		nums.pop();

		nums.into_iter()
			.enumerate()
			.fold((0, 0, 0), |(step_cnt, cur_max, next_max), (i, stp)| {
				let next_max = next_max.max(i + stp as usize);
				if i == cur_max {
					(step_cnt + 1, next_max, next_max)
				} else {
					(step_cnt, cur_max, next_max)
				}
			})
			.0
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_45() {
		assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
		assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
	}
}
