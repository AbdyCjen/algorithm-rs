/**
 * [2461] Maximum Sum of Distinct Subarrays With Length K
 *
 * You are given an integer array nums and an integer k. Find the maximum subarray sum of all the subarrays of nums that meet the following conditions:
 *
 *     The length of the subarray is k, and
 *     All the elements of the subarray are distinct.
 *
 * Return the maximum subarray sum of all the subarrays that meet the conditions. If no subarray meets the conditions, return 0.
 * A subarray is a contiguous non-empty sequence of elements within an array.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,5,4,2,9,9,9], k = 3
 * Output: 15
 * Explanation: The subarrays of nums with length 3 are:
 * - [1,5,4] which meets the requirements and has a sum of 10.
 * - [5,4,2] which meets the requirements and has a sum of 11.
 * - [4,2,9] which meets the requirements and has a sum of 15.
 * - [2,9,9] which does not meet the requirements because the element 9 is repeated.
 * - [9,9,9] which does not meet the requirements because the element 9 is repeated.
 * We return 15 because it is the maximum subarray sum of all the subarrays that meet the conditions
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [4,4,4], k = 3
 * Output: 0
 * Explanation: The subarrays of nums with length 3 are:
 * - [4,4,4] which does not meet the requirements because the element 4 is repeated.
 * We return 0 because no subarrays meet the conditions.
 *
 *  
 * Constraints:
 *
 *     1 <= k <= nums.length <= 10^5
 *     1 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
		let k = k as usize;
		let mut set = std::collections::BTreeSet::new();
		let mut s = 0;
		let mut ans = 0;
		let mut i = 0;
		for (j, &n) in nums.iter().enumerate() {
			s += n as i64;

			if !set.insert(n) {
				while nums[i] != n {
					set.remove(&nums[i]);
					s -= nums[i] as i64;
					i += 1;
				}
				s -= nums[i] as i64;
				i += 1;
			}
			if j - i + 1 == k {
				ans = ans.max(s);
				set.remove(&nums[i]);
				s -= nums[i] as i64;
				i += 1;
			}
		}

		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2461() {
		assert_eq!(
			Solution::maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3),
			15
		);
		assert_eq!(Solution::maximum_subarray_sum(vec![4, 4, 4], 3), 0);
		assert_eq!(
			Solution::maximum_subarray_sum(vec![9, 9, 9, 1, 2, 3], 3),
			12
		);
	}
}
