/**
 * [50] Pow(x, n)
 *
 * Implement <a href="http://www.cplusplus.com/reference/valarray/pow/" target="_blank">pow(x, n)</a>, which calculates x raised to the power n (i.e., x^n).
 *  
 * <strong class="example">Example 1:
 *
 * Input: x = 2.00000, n = 10
 * Output: 1024.00000
 *
 * <strong class="example">Example 2:
 *
 * Input: x = 2.10000, n = 3
 * Output: 9.26100
 *
 * <strong class="example">Example 3:
 *
 * Input: x = 2.00000, n = -2
 * Output: 0.25000
 * Explanation: 2^-2 = 1/2^2 = 1/4 = 0.25
 *
 *  
 * Constraints:
 *
 *     -100.0 < x < 100.0
 *     -2^31 <= n <= 2^31-1
 *     n is an integer.
 *     Either x is not zero or n > 0.
 *     -10^4 <= x^n <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn my_pow(mut x: f64, n: i32) -> f64 {
		let mut ans = 1.0;
		if n < 0 {
			x = 1.0 / x;
		}
		let mut n = n.unsigned_abs();
		while n > 0 {
			if n % 2 == 1 {
				ans *= x;
			}
			x *= x;
			n >>= 1;
		}
		ans
	}
}

// submission codes end
