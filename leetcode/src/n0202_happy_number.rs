/**
 * [202] Happy Number
 *
 * Write an algorithm to determine if a number n is "happy".
 * A happy number is a number defined by the following process: Starting with any positive integer, replace the number by the sum of the squares of its digits, and repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1. Those numbers for which this process ends in 1 are happy numbers.
 * Return True if n is a happy number, and False if not.
 * Example:
 *
 * Input: 19
 * Output: true
 * Explanation:
 * 1^2 + 9^2 = 82
 * 8^2 + 2^2 = 68
 * 6^2 + 8^2 = 100
 * 1^2 + 0^2 + 0^2 = 1
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn is_happy(mut n: i32) -> bool {
		// i32 最大(2e10 - 1).happy_number() == 730 , 699.happy_number() = 198, 所以最多两百个循环
		for _ in 0..200 {
			let mut sum = 0;
			while n != 0 {
				sum += (n % 10) * (n % 10);
				n /= 10;
			}
			if sum == 1 {
				return true;
			}
			n = sum;
		}
		false
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_202() {
		assert_eq!(Solution::is_happy(19), true);
	}
}
