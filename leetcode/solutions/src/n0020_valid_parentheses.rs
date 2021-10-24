/**
 * [20] Valid Parentheses
 *
 * Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
 *
 * An input string is valid if:
 *
 * <ol>
 * Open brackets must be closed by the same type of brackets.
 * Open brackets must be closed in the correct order.
 * </ol>
 *
 * Note that an empty string is also considered valid.
 *
 * Example 1:
 *
 *
 * Input: "()"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: "()[]{}"
 * Output: true
 *
 *
 * Example 3:
 *
 *
 * Input: "(]"
 * Output: false
 *
 *
 * Example 4:
 *
 *
 * Input: "([)]"
 * Output: false
 *
 *
 * Example 5:
 *
 *
 * Input: "{[]}"
 * Output: true
 *
 *
 */
#[allow(dead_code)]
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn is_valid(s: String) -> bool {
		let mut st = Vec::new();
		s.bytes().all(|c| match c {
			b'(' | b'[' | b'{' => {
				st.push(c);
				true
			}
			b')' => st.pop() == Some(b'('),
			b']' => st.pop() == Some(b'['),
			b'}' => st.pop() == Some(b'{'),
			_ => false,
		}) && st.is_empty()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_20() {
		assert!(Solution::is_valid("()".to_owned()));
		assert!(!Solution::is_valid("(()".to_owned()));
		assert!(!Solution::is_valid("(]".to_owned()));
		assert!(Solution::is_valid("()[]{}".to_owned()));
		assert!(!Solution::is_valid("(]".to_owned()));
		assert!(!Solution::is_valid("([)]".to_owned()));
		assert!(Solution::is_valid("{[]}".to_owned()));
	}
}
