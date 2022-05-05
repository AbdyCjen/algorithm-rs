/**
 * [172] Factorial Trailing Zeroes
 *
 * Given an integer n, return the number of trailing zeroes in n!.
 * Note that n! = n * (n - 1) * (n - 2) * ... * 3 * 2 * 1.
 *  
 * Example 1:
 *
 * Input: n = 3
 * Output: 0
 * Explanation: 3! = 6, no trailing zero.
 *
 * Example 2:
 *
 * Input: n = 5
 * Output: 1
 * Explanation: 5! = 120, one trailing zero.
 *
 * Example 3:
 *
 * Input: n = 0
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     0 <= n <= 10^4
 *
 *  
 * Follow up: Could you write a solution that works in logarithmic time complexity?
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn trailing_zeroes(mut n: i32) -> i32 {
		let mut cnt_five = 0;
		while n / 5 > 0 {
			cnt_five += n / 5;
			n /= 5;
		}

		// 2 因数一定会少于5
		cnt_five
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_172() {
		assert_eq!(Solution::trailing_zeroes(3), 0);
		assert_eq!(Solution::trailing_zeroes(5), 1);
		assert_eq!(Solution::trailing_zeroes(0), 0);
	}
}
