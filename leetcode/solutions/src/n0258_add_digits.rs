/**
 * [258] Add Digits
 *
 * Given an integer num, repeatedly add all its digits until the result has only one digit, and return it.
 *  
 * Example 1:
 *
 * Input: num = 38
 * Output: 2
 * Explanation: The process is
 * 38 --> 3 + 8 --> 11
 * 11 --> 1 + 1 --> 2
 * Since 2 has only one digit, return it.
 *
 * Example 2:
 *
 * Input: num = 0
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     0 <= num <= 2^31 - 1
 *
 *  
 * Follow up: Could you do it without any loop/recursion in O(1) runtime?
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn add_digits(mut num: i32) -> i32 {
		while num >= 10 {
			let mut n = 0;
			while num > 0 {
				n += num % 10;
				num /= 10;
			}
			num = n;
		}
		num
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_258() {
		assert_eq!(Solution::add_digits(38), 2);
		assert_eq!(Solution::add_digits(0), 0);
		assert_eq!(Solution::add_digits(10), 1);
	}
}
