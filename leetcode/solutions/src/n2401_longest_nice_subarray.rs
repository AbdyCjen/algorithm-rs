/**
 * [2401] Longest Nice Subarray
 *
 * You are given an array nums consisting of positive integers.
 * We call a subarray of nums nice if the bitwise AND of every pair of elements that are in different positions in the subarray is equal to 0.
 * Return the length of the longest nice subarray.
 * A subarray is a contiguous part of an array.
 * Note that subarrays of length 1 are always considered nice.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,3,8,48,10]
 * Output: 3
 * Explanation: The longest nice subarray is [3,8,48]. This subarray satisfies the conditions:
 * - 3 AND 8 = 0.
 * - 3 AND 48 = 0.
 * - 8 AND 48 = 0.
 * It can be proven that no longer nice subarray can be obtained, so we return 3.
 * <strong class="example">Example 2:
 *
 * Input: nums = [3,1,5,11,13]
 * Output: 1
 * Explanation: The length of the longest nice subarray is 1. Any subarray of length 1 can be chosen.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     1 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
		let mut it = nums.iter();
		let mut ans = 0;
		let mut set = 0;
		let mut len = 0;
		for &n in nums.iter() {
			len += 1;
			while n & set > 0 {
				len -= 1;
				set ^= it.next().unwrap();
			}
			set |= n;
			ans = ans.max(len);
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2401() {
		assert_eq!(Solution::longest_nice_subarray(vec![3, 8, 48]), 3);
		assert_eq!(Solution::longest_nice_subarray(vec![3, 1, 5, 11, 13]), 1);
	}
}
