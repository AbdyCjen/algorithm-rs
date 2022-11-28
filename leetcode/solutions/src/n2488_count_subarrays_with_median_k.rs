/**
 * [2488] Count Subarrays With Median K
 *
 * You are given an array nums of size n consisting of distinct integers from 1 to n and a positive integer k.
 * Return the number of non-empty subarrays in nums that have a median equal to k.
 * Note:
 *
 *     The median of an array is the middle element after sorting the array in ascending order. If the array is of even length, the median is the left middle element.
 *     
 *         For example, the median of [2,3,1,4] is 2, and the median of [8,4,3,5,1] is 4.
 *     
 *     
 *     A subarray is a contiguous part of an array.
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [3,2,1,4,5], k = 4
 * Output: 3
 * Explanation: The subarrays that have a median equal to 4 are: [4], [4,5] and [1,4,5].
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [2,3,1], k = 3
 * Output: 1
 * Explanation: [3] is the only subarray that has a median equal to 3.
 *
 *  
 * Constraints:
 *
 *     n == nums.length
 *     1 <= n <= 10^5
 *     1 <= nums[i], k <= n
 *     The integers in nums are distinct.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
		use std::cmp::Ordering;
		let ki = match nums.iter().position(|i| *i == k) {
			None => return 0,
			Some(i) => i,
		};
		let mut bal = 0_i32;
		let mut cnt = std::collections::HashMap::new();
		cnt.insert(0, 1);
		for &i in nums[..ki].iter().rev() {
			match i.cmp(&k) {
				Ordering::Less => bal -= 1,
				Ordering::Greater => bal += 1,
				_ => {}
			}
			*cnt.entry(bal).or_insert(0) += 1;
		}
		bal = 0;
		let mut ans = 0;
		for &j in &nums[ki..] {
			match j.cmp(&k) {
				Ordering::Less => bal -= 1,
				Ordering::Greater => bal += 1,
				_ => {}
			}
			ans += cnt.get(&(-bal)).unwrap_or(&0);
			ans += cnt.get(&(-bal + 1)).unwrap_or(&0);
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2488() {
		assert_eq!(
			Solution::count_subarrays(vec![10, 3, 8, 5, 6, 7, 2, 9, 4, 1], 9),
			1
		);
		assert_eq!(Solution::count_subarrays(vec![3, 2, 1, 4, 5], 4), 3);
	}
}
