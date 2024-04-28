/**
 * [678] Valid Parenthesis String
 *
 *
 * Given a string containing only three types of characters: '(', ')' and '*', write a function to check whether this string is valid. We define the validity of a string by these rules:
 * <ol>
 * Any left parenthesis '(' must have a corresponding right parenthesis ')'.
 * Any right parenthesis ')' must have a corresponding left parenthesis '('.
 * Left parenthesis '(' must go before the corresponding right parenthesis ')'.
 * '*' could be treated as a single right parenthesis ')' or a single left parenthesis '(' or an empty string.
 * An empty string is also valid.
 * </ol>
 *
 *
 * Example 1:<br />
 *
 * Input: "()"
 * Output: True
 *
 *
 *
 * Example 2:<br />
 *
 * Input: "(*)"
 * Output: True
 *
 *
 *
 * Example 3:<br />
 *
 * Input: "(*))"
 * Output: True
 *
 *
 *
 * Note:<br>
 * <ol>
 * The string size will be in the range [1, 100].
 * </ol>
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	//TODO: illustration
	pub fn check_valid_string(s: String) -> bool {
		let (mut open, mut close) = (1, 1);
		s.bytes().all(|c| {
			match c {
				b'(' | b'*' => open += 1,
				_ => open -= 1,
			}
			open > 0
		}) && s.bytes().rev().all(|c| {
			match c {
				b')' | b'*' => close += 1,
				_ => close -= 1,
			}
			close > 0
		})
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_678() {
		assert!(Solution::check_valid_string("()".to_owned()));
		assert!(Solution::check_valid_string("(*)".to_owned()));
		assert!(Solution::check_valid_string("(*))".to_owned()));
		assert!(Solution::check_valid_string("********".to_owned()));
		assert!(Solution::check_valid_string("***".to_owned()));
		assert!(Solution::check_valid_string("*((**".to_owned()));
		assert!(!Solution::check_valid_string("*((*".to_owned()));
	}
}
