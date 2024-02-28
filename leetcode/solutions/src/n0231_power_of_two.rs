/**
 * [231] Power of Two
 *
 * Given an integer n, return true if it is a power of two. Otherwise, return false.
 * An integer n is a power of two, if there exists an integer x such that n == 2^x.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 1
 * Output: true
 * Explanation: 2^0 = 1
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 16
 * Output: true
 * Explanation: 2^4 = 16
 *
 * <strong class="example">Example 3:
 *
 * Input: n = 3
 * Output: false
 *
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

impl Solution {
	pub fn is_power_of_two(n: i32) -> bool { n > 0 && n.count_ones() == 1 }
}

// submission codes end
