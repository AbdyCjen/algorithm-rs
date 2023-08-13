/**
 * [1802] Maximum Value at a Given Index in a Bounded Array
 *
 * You are given three positive integers: n, index, and maxSum. You want to construct an array nums (0-indexed) that satisfies the following conditions:
 *
 *     nums.length == n
 *     nums[i] is a positive integer where 0 <= i < n.
 *     abs(nums[i] - nums[i+1]) <= 1 where 0 <= i < n-1.
 *     The sum of all the elements of nums does not exceed maxSum.
 *     nums[index] is maximized.
 *
 * Return nums[index] of the constructed array.
 * Note that abs(x) equals x if x >= 0, and -x otherwise.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 4, index = 2,  maxSum = 6
 * Output: 2
 * Explanation: nums = [1,2,<u>2</u>,1] is one array that satisfies all the conditions.
 * There are no arrays that satisfy all the conditions and have nums[2] == 3, so 2 is the maximum nums[2].
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 6, index = 1,  maxSum = 10
 * Output: 3
 *
 *  
 * Constraints:
 *
 *     1 <= n <= maxSum <= 10^9
 *     0 <= index < n
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_value(n: i32, idx: i32, max_sum: i32) -> i32 {
		fn sum(f: i32, l: i32) -> i32 {
			if l < f {
				(f + f - l + 1).saturating_mul(l) / 2
			} else {
				(f + 1).saturating_mul(f) / 2 + l - f
			}
		}
		let (mut l, mut r) = (1, max_sum);
		while l <= r {
			let m = (l + r) / 2;
			if sum(m - 1, idx).saturating_add(sum(m, n - idx)) > max_sum {
				r = m - 1;
			} else {
				l = m + 1;
			}
		}
		r
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1802() {
		assert_eq!(Solution::max_value(6, 1, 10), 3);
		assert_eq!(Solution::max_value(4, 2, 6), 2);
		assert_eq!(Solution::max_value(3, 0, 815094800), 271698267);
		assert_eq!(Solution::max_value(8257285, 4828516, 850015631), 29014);
	}
}
