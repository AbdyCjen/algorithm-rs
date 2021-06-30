/**
 * [1004] Max Consecutive Ones III
 *
 * Given a binary array nums and an integer k, return the maximum number of consecutive 1's in the array if you can flip at most k 0's.
 *  
 * Example 1:
 *
 * Input: nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
 * Output: 6
 * Explanation: [1,1,1,0,0,<u>1,1,1,1,1,1</u>]
 * Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.
 * Example 2:
 *
 * Input: nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], k = 3
 * Output: 10
 * Explanation: [0,0,<u>1,1,1,1,1,1,1,1,1,1</u>,0,0,0,1,1,1,1]
 * Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     nums[i] is either 0 or 1.
 *     0 <= k <= nums.length
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
		use std::collections::VecDeque;
		let mut dq = VecDeque::new();
		let mut cnts = [0; 2];
		nums.into_iter().fold(0, |max_cnt, i| {
			cnts[i as usize] += 1;
			dq.push_back(i);

			while cnts[0] > k {
				if let Some(i) = dq.pop_front() {
					cnts[i as usize] -= 1;
				}
			}
			max_cnt.max(cnts[0] + cnts[1])
		})
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1004() {
		assert_eq!(
			Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2),
			6
		);
		assert_eq!(
			Solution::longest_ones(
				vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
				3
			),
			10
		);
		assert_eq!(
			Solution::longest_ones(vec![1, 1, 0, 0, 1, 1, 1, 0, 1, 1], 4),
			10
		);
	}
}
