/**
 * [2439] Minimize Maximum of Array
 *
 * You are given a 0-indexed array nums comprising of n non-negative integers.
 * In one operation, you must:
 *
 *     Choose an integer i such that 1 <= i < n and nums[i] > 0.
 *     Decrease nums[i] by 1.
 *     Increase nums[i - 1] by 1.
 *
 * Return the minimum possible value of the maximum integer of nums after performing any number of operations.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [3,7,1,6]
 * Output: 5
 * Explanation:
 * One set of optimal operations is as follows:
 * 1. Choose i = 1, and nums becomes [4,6,1,6].
 * 2. Choose i = 3, and nums becomes [4,6,2,5].
 * 3. Choose i = 1, and nums becomes [5,5,2,5].
 * The maximum integer of nums is 5. It can be shown that the maximum number cannot be less than 5.
 * Therefore, we return 5.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [10,1]
 * Output: 10
 * Explanation:
 * It is optimal to leave nums as is, and since 10 is the maximum value, we return 10.
 *
 *  
 * Constraints:
 *
 *     n == nums.length
 *     2 <= n <= 10^5
 *     0 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

impl Solution {
	pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
		let (mut ans, mut sum) = (0, 0);
		for (n, i) in nums.into_iter().zip(1..) {
			sum += n as u64;
			ans = ans.max((sum as f64 / i as f64).ceil() as i32);
		}
		ans
	}
	pub fn minimize_array_value_1(nums: Vec<i32>) -> i32 {
		let mut l = *nums.iter().min().unwrap();
		let mut r = *nums.iter().max().unwrap();

		while l < r {
			let m = (l + r) / 2;
			let mut rest = 0_u64;
			for &i in nums.iter().rev() {
				use std::cmp::Ordering;
				match i.cmp(&m) {
					Ordering::Less => rest = rest.saturating_sub((m - i) as u64),
					Ordering::Greater => rest += (i - m) as u64,
					Ordering::Equal => {}
				}
			}

			if rest > 0 {
				l = m + 1;
			} else {
				r = m;
			}
		}

		l
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2439() {
		assert_eq!(Solution::minimize_array_value(vec![6, 9, 3, 8, 14]), 8);
		assert_eq!(Solution::minimize_array_value(vec![3, 7, 1, 6]), 5);
		assert_eq!(Solution::minimize_array_value(vec![10, 1]), 10);
	}
}
