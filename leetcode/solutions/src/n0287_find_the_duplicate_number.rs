/**
 * [287] Find the Duplicate Number
 *
 * Given an array of integers nums containing n + 1 integers where each integer is in the range [1, n] inclusive.
 * There is only one repeated number in nums, return this repeated number.
 * You must solve the problem without modifying the array nums and uses only constant extra space.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,3,4,2,2]
 * Output: 2
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [3,1,3,4,2]
 * Output: 3
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 10^5
 *     nums.length == n + 1
 *     1 <= nums[i] <= n
 *     All the integers in nums appear only once except for precisely one integer which appears two or more times.
 *
 *  
 * Follow up:
 *
 *     How can we prove that at least one duplicate number must exist in nums?
 *     Can you solve the problem in linear runtime complexity?
 *
 */
pub struct Solution {}

impl Solution {
	pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
		let n = nums.len() - 1;
		while nums[n] != nums[nums[n] as usize - 1] {
			let nn = nums[n] as usize - 1;
			nums.swap(n, nn);
		}
		nums[n]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_287() {
		assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
		assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
	}
}
