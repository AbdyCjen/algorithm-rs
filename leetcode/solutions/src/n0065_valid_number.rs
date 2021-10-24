/**
 * [65] Valid Number
 *
 * A valid number can be split up into these components (in order):
 * <ol>
 *     A decimal number or an integer.
 *     (Optional) An 'e' or 'E', followed by an integer.
 * </ol>
 * A decimal number can be split up into these components (in order):
 * <ol>
 *     (Optional) A sign character (either '+' or '-').
 *     One of the following formats:
 *     <ol>
 *         At least one digit, followed by a dot '.'.
 *         At least one digit, followed by a dot '.', followed by at least one digit.
 *         A dot '.', followed by at least one digit.
 *     </ol>
 *
 * </ol>
 * An integer can be split up into these components (in order):
 * <ol>
 *     (Optional) A sign character (either '+' or '-').
 *     At least one digit.
 * </ol>
 * For example, all the following are valid numbers: ["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"], while the following are not valid numbers: ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"].
 * Given a string s, return true if s is a valid number.
 *  
 * Example 1:
 *
 * Input: s = "0"
 * Output: true
 *
 * Example 2:
 *
 * Input: s = "e"
 * Output: false
 *
 * Example 3:
 *
 * Input: s = "."
 * Output: false
 *
 * Example 4:
 *
 * Input: s = ".1"
 * Output: true
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 20
 *     s consists of only English letters (both uppercase and lowercase), digits (0-9), plus '+', minus '-', or dot '.'.
 *
 */
pub struct Solution {}

// submission codes start here

use std::iter::Iterator;
#[allow(dead_code)]
impl Solution {
	pub fn is_number(s: String) -> bool {
		let mut it = s.bytes().peekable();

		it.next_if(|&c| c == b'-' || c == b'+');

		if it.next_if(u8::is_ascii_digit).is_some() {
			while it.next_if(u8::is_ascii_digit).is_some() {}
			if it.next_if_eq(&b'.').is_some() {
				while it.next_if(u8::is_ascii_digit).is_some() {}
			}
		} else if it.next_if_eq(&b'.').is_some() {
			if it.next_if(u8::is_ascii_digit).is_none() {
				return false;
			}
			while it.next_if(u8::is_ascii_digit).is_some() {}
		} else {
			return false;
		}

		if it.next_if(|&c| c == b'e' || c == b'E').is_some() {
			it.next_if(|&c| c == b'-' || c == b'+');
			if it.next_if(u8::is_ascii_digit).is_some() {
				while it.next_if(u8::is_ascii_digit).is_some() {}
			} else {
				return false;
			}
		}

		it.next().is_none()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_65() {
		let valid_nums = vec_string![
			"2",
			"0089",
			"-0.1",
			"+3.14",
			"4.",
			"-.9",
			"2e10",
			"-90E3",
			"3e+7",
			"+6e-1",
			"53.5e93",
			"-123.456e789"
		];
		let invalid_nums = vec_string!["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"];
		for s in valid_nums {
			assert!(Solution::is_number(s));
		}
		for s in invalid_nums {
			assert!(!Solution::is_number(s));
		}
	}
}
