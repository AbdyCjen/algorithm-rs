/**
 * [263] Ugly Number
 *
 * An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.
 * Given an integer n, return true if n is an ugly number.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 6
 * Output: true
 * Explanation: 6 = 2 &times; 3
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 1
 * Output: true
 * Explanation: 1 has no prime factors, therefore all of its prime factors are limited to 2, 3, and 5.
 *
 * <strong class="example">Example 3:
 *
 * Input: n = 14
 * Output: false
 * Explanation: 14 is not ugly since it includes the prime factor 7.
 *
 *  
 * Constraints:
 *
 *     -2^31 <= n <= 2^31 - 1
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn is_ugly(mut n: i32) -> bool {
		if n <= 0 {
			return false;
		}
		for i in [2, 3, 5] {
			while n % i == 0 {
				n /= i;
			}
		}
		n == 1
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_263() {
		assert!(Solution::is_ugly(10));
		assert!(!Solution::is_ugly(14));
		assert!(!Solution::is_ugly(0));
		assert!(!Solution::is_ugly(-2));
	}
}
