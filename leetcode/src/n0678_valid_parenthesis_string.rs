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

#[allow(dead_code)]
impl Solution {
	pub fn check_valid_string(s: String) -> bool { Self::unmatch_count(s).is_some() }
	fn unmatch_count(s: String) -> Option<()> {
		let (mut open_cnt, mut closed_cnt) = (vec![()], vec![()]);
		for (c, cc) in s.bytes().zip(s.bytes().rev()) {
			if c == b'(' || c == b'*' {
				open_cnt.push(());
			} else {
				open_cnt.pop();
			}
			if cc == b')' || cc == b'*' {
				closed_cnt.push(());
			} else {
				closed_cnt.pop();
			}
			open_cnt.first()?;
			closed_cnt.first()?;
		}
		Some(())
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_678() {
		assert_eq!(Solution::check_valid_string("()".to_owned()), true);
		assert_eq!(Solution::check_valid_string("(*)".to_owned()), true);
		assert_eq!(Solution::check_valid_string("(*))".to_owned()), true);
		assert_eq!(Solution::check_valid_string("********".to_owned()), true);
		assert_eq!(Solution::check_valid_string("***".to_owned()), true);
		assert_eq!(Solution::check_valid_string("*((**".to_owned()), true);
		assert_eq!(Solution::check_valid_string("*((*".to_owned()), false);
	}
}
