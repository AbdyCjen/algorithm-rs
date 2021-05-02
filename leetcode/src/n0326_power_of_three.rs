/**
 * [326] Power of Three
 *
 * Given an integer n, return true if it is a power of three. Otherwise, return false.
 * An integer n is a power of three, if there exists an integer x such that n == 3^x.
 *  
 * Example 1:
 * Input: n = 27
 * Output: true
 * Example 2:
 * Input: n = 0
 * Output: false
 * Example 3:
 * Input: n = 9
 * Output: true
 * Example 4:
 * Input: n = 45
 * Output: false
 *  
 * Constraints:
 *
 *     -2^31 <= n <= 2^31 - 1
 *
 *  
 * Follow up: Could you solve it without loops/recursion?
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn is_power_of_three(n: i32) -> bool {
		const MAX_POWER_OF_THREE: i32 = 1162261467;
		n > 0 && MAX_POWER_OF_THREE % n == 0
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_326() {
		assert_eq!(Solution::is_power_of_three(3), true);
		assert_eq!(Solution::is_power_of_three(27), true);
		assert_eq!(Solution::is_power_of_three(45), false);
	}
}
