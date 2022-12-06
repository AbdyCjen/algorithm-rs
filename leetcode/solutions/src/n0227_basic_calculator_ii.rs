/**
 * [227] Basic Calculator II
 *
 * Given a string s which represents an expression, evaluate this expression and return its value.
 * The integer division should truncate toward zero.
 * You may assume that the given expression is always valid. All intermediate results will be in the range of [-2^31, 2^31 - 1].
 * Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as eval().
 *  
 * <strong class="example">Example 1:
 * Input: s = "3+2*2"
 * Output: 7
 * <strong class="example">Example 2:
 * Input: s = " 3/2 "
 * Output: 1
 * <strong class="example">Example 3:
 * Input: s = " 3+5 / 2 "
 * Output: 5
 *  
 * Constraints:
 *
 *     1 <= s.length <= 3 * 10^5
 *     s consists of integers and operators ('+', '-', '*', '/') separated by some number of spaces.
 *     s represents a valid expression.
 *     All the integers in the expression are non-negative integers in the range [0, 2^31 - 1].
 *     The answer is guaranteed to fit in a 32-bit integer.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn calculate(s: String) -> i32 {
		let mut s = s.as_bytes();
		let (mut ans, mut cur, mut flg) = (0, 0, 1);
		while let [c, s_n @ ..] = s {
			let mut s_n = s_n;
			match c {
				b' ' => {}
				b'-' | b'+' => {
					ans += flg * cur;
					flg = if *c == b'-' { -1 } else { 1 };
					cur = 0;
				}
				b'*' | b'/' => {
					let (n, s_nn) = Self::get_i32(s_n);
					s_n = s_nn;
					if *c == b'*' {
						cur *= n;
					} else {
						cur /= n;
					}
				}
				b'0'..=b'9' => cur = cur * 10 + (*c - b'0') as i32,
				_ => unreachable!(),
			}

			s = s_n;
		}
		ans + flg * cur
	}

	fn get_i32(mut s: &[u8]) -> (i32, &[u8]) {
		while let [b' ', s_n @ ..] = s {
			s = s_n;
		}
		let mut n = 0;
		while let [c @ b'0'..=b'9', s_n @ ..] = s {
			n = n * 10 + (c - b'0') as i32;
			s = s_n;
		}
		(n, s)
	}
}

// submission codes end

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_227() {
		assert_eq!(Solution::calculate("3 - 5 / 2".to_owned()), 1);
		assert_eq!(Solution::calculate("3 + 2 * 2".to_owned()), 7);
		assert_eq!(Solution::calculate("3 / 2".to_owned()), 1);
		assert_eq!(Solution::calculate("3 + 5 / 2".to_owned()), 5);
	}
}
