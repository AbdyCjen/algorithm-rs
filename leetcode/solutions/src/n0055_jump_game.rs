/**
 * [55] Jump Game
 *
 * Given an array of non-negative integers, you are initially positioned at the first index of the array.
 * Each element in the array represents your maximum jump length at that position.
 * Determine if you are able to reach the last index.
 *  
 * Example 1:
 *
 * Input: nums = [2,3,1,1,4]
 * Output: true
 * Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
 *
 * Example 2:
 *
 * Input: nums = [3,2,1,0,4]
 * Output: false
 * Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.
 *
 *  
 * Constraints:
 *
 * 1 <= nums.length <= 3 * 10^4
 * 0 <= nums[i][j] <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn can_jump(nums: Vec<i32>) -> bool {
		let (mut cur, mut end, l) = (0, 0, nums.len());
		while cur <= end && cur < l {
			end = std::cmp::max(cur + nums[cur] as usize, end);
			cur += 1;
		}
		cur == l
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_55() {
		assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
		assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
	}
}
