/**
 * [1201] Ugly Number III
 *
 * An ugly number is a positive integer that is divisible by a, b, or c.
 * Given four integers n, a, b, and c, return the n^th ugly number.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 3, a = 2, b = 3, c = 5
 * Output: 4
 * Explanation: The ugly numbers are 2, 3, 4, 5, 6, 8, 9, 10... The 3^rd is 4.
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 4, a = 2, b = 3, c = 4
 * Output: 6
 * Explanation: The ugly numbers are 2, 3, 4, 6, 8, 9, 10, 12... The 4^th is 6.
 *
 * <strong class="example">Example 3:
 *
 * Input: n = 5, a = 2, b = 11, c = 13
 * Output: 10
 * Explanation: The ugly numbers are 2, 4, 6, 8, 10, 11, 12, 13... The 5^th is 10.
 *
 *  
 * Constraints:
 *
 *     1 <= n, a, b, c <= 10^9
 *     1 <= a * b * c <= 10^18
 *     It is guaranteed that the result will be in range [1, 2 * 10^9].
 *
 */
pub struct Solution {}

impl Solution {
	pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
		let min = a.min(b).min(c);
		let (mut l, mut r) = (n, n.saturating_mul(min));
		while l <= r {
			let m = (r - l) / 2 + l;
			if Self::count(m, (a, b, c)) >= n {
				r = m - 1;
			} else {
				l = m + 1;
			}
		}
		l
	}
	fn gcd(a: i32, b: i32) -> i32 {
		match b {
			0 => a,
			_ => Self::gcd(b, a % b),
		}
	}
	fn lcm(a: i32, b: i32) -> i32 { (a / Self::gcd(a, b)).saturating_mul(b) }

	fn count(k: i32, (a, b, c): (i32, i32, i32)) -> i32 {
		(k / a + k / b + k / c) - (k / Self::lcm(a, b) + k / Self::lcm(b, c) + k / Self::lcm(c, a))
			+ (k / (Self::lcm(Self::lcm(a, b), c)))
	}
}
