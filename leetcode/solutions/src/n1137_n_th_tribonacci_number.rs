/**
 * [1137] N-th Tribonacci Number
 *
 * The Tribonacci sequence Tn is defined as follows:
 *
 * T0 = 0, T1 = 1, T2 = 1, and Tn+3 = Tn + Tn+1 + Tn+2 for n >= 0.
 *
 * Given n, return the value of Tn.
 *
 *  
 * <strong class="example">Example 1:
 *
 *
 * Input: n = 4
 * Output: 4
 * Explanation:
 * T_3 = 0 + 1 + 1 = 2
 * T_4 = 1 + 1 + 2 = 4
 *
 *
 * <strong class="example">Example 2:
 *
 *
 * Input: n = 25
 * Output: 1389537
 *
 *
 *  
 * Constraints:
 *
 *
 *     0 <= n <= 37
 *     The answer is guaranteed to fit within a 32-bit integer, ie. answer <= 2^31 - 1.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn tribonacci(n: i32) -> i32 { (0..n).fold((0, 1, 0), |(a, b, c), _| (a + b + c, a, b)).0 }
	pub fn tribonacci_01(n: i32) -> i32 {
		if n <= 1 {
			return n;
		}
		let mut s = [0, 1, 0];
		for j in 2..=n {
			s[(j % 3) as usize] = s[0] + s[1] + s[2];
		}
		s[(n % 3) as usize]
	}
}

// submission codes end
