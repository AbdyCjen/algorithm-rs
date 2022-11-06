/**
 * [334] Increasing Triplet Subsequence
 *
 * Given an integer array nums, return true if there exists a triple of indices (i, j, k) such that i < j < k and nums[i] < nums[j] < nums[k]. If no such indices exists, return false.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2,3,4,5]
 * Output: true
 * Explanation: Any triplet where i < j < k is valid.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [5,4,3,2,1]
 * Output: false
 * Explanation: No triplet exists.
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [2,1,5,0,4,6]
 * Output: true
 * Explanation: The triplet (3, 4, 5) is valid because nums[3] == 0 < nums[4] == 4 < nums[5] == 6.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 5 * 10^5
 *     -2^31 <= nums[i] <= 2^31 - 1
 *
 *  
 * Follow up: Could you implement a solution that runs in O(n) time complexity and O(1) space complexity?
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn increasing_triplet(nums: Vec<i32>) -> bool {
		let mut left = i32::MAX;
		let mut mid = i32::MAX;
		for i in nums {
			if i > mid {
				return true;
			}

			if i > left && i < mid {
				mid = i;
			}
			left = left.min(i);
		}
		false
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_334() {
		assert!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]));
		assert!(!Solution::increasing_triplet(vec![5, 4, 3, 2, 1]));
	}
}
