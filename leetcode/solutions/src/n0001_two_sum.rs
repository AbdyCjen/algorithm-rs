/**
 * [1] Two Sum
 *
 * Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 * You can return the answer in any order.
 *  
 * Example 1:
 *
 * Input: nums = [2,7,11,15], target = 9
 * Output: [0,1]
 * Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
 *
 * Example 2:
 *
 * Input: nums = [3,2,4], target = 6
 * Output: [1,2]
 *
 * Example 3:
 *
 * Input: nums = [3,3], target = 6
 * Output: [0,1]
 *
 *  
 * Constraints:
 *
 *     2 <= nums.length <= 10^4
 *     -10^9 <= nums[i] <= 10^9
 *     -10^9 <= target <= 10^9
 *     Only one valid answer exists.
 *
 *  
 * Follow-up: Can you come up with an algorithm that is less than O(n^2) time complexity?
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
		let mut nums: Vec<_> = nums.into_iter().zip(0_i32..).collect();
		nums.sort_unstable();
		let (mut i, mut j) = (0, nums.len() - 1);
		while i < j {
			match (nums[i].0 + nums[j].0).cmp(&target) {
				std::cmp::Ordering::Less => i += 1,
				std::cmp::Ordering::Greater => j -= 1,
				std::cmp::Ordering::Equal => return vec![nums[i].1, nums[j].1],
			}
		}
		unreachable!()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
		assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
		assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
	}
}
