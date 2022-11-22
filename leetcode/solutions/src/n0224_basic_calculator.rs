/**
 * [224] Basic Calculator
 *
 * Given a string s representing a valid expression, implement a basic calculator to evaluate it, and return the result of the evaluation.
 * Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as eval().
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "1 + 1"
 * Output: 2
 *
 * <strong class="example">Example 2:
 *
 * Input: s = " 2-1 + 2 "
 * Output: 3
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "(1+(4+5+2)-3)+(6+8)"
 * Output: 23
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 3 * 10^5
 *     s consists of digits, '+', '-', '(', ')', and ' '.
 *     s represents a valid expression.
 *     '+' is not used as a unary operation (i.e., "+1" and "+(2 + 3)" is invalid).
 *     '-' could be used as a unary operation (i.e., "-1" and "-(2 + 3)" is valid).
 *     There will be no two consecutive operators in the input.
 *     Every number and running calculation will fit in a signed 32-bit integer.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn calculate(s: String) -> i32 { Self::cal_inner(s.as_bytes()).1 }

	fn cal_inner(mut s: &[u8]) -> (&[u8], i32) {
		let mut ans = 0;
		let mut flg = 1;
		while let [c, s_n @ ..] = s {
			let mut s_n = s_n;
			match c {
				b'-' => flg *= -1,
				b'+' | b' ' => {}
				b')' => return (s_n, ans),
				b'(' => {
					let (s_nn, to_add) = Self::cal_inner(s_n);
					ans += flg * to_add;
					flg = 1;
					s_n = s_nn;
				}
				c if c.is_ascii_digit() => {
					let mut cur = 0;
					let mut i = 0;
					for c in s.iter().take_while(|c| c.is_ascii_digit()) {
						i += 1;
						cur = cur * 10 + (c - b'0') as i32;
					}
					ans += cur * flg;
					flg = 1;
					s_n = &s[i..];
				}
				_ => unreachable!(),
			}
			s = s_n;
		}
		(s, ans)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_224() {
		assert_eq!(Solution::calculate("1 + 1".to_owned()), 2);
		assert_eq!(Solution::calculate(" 2-1 + 2 ".to_owned()), 3);
		assert_eq!(Solution::calculate(" 1 - -1 + 2".to_owned()), 4);
		assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_owned()), 23);
	}
}
