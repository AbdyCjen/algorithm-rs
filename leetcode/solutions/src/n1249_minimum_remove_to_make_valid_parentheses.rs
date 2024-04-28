/**
 * [1249] Minimum Remove to Make Valid Parentheses
 *
 * Given a string <font face="monospace">s</font> of '(' , ')' and lowercase English characters.
 * Your task is to remove the minimum number of parentheses ( '(' or ')', in any positions ) so that the resulting parentheses string is valid and return any valid string.
 * Formally, a parentheses string is valid if and only if:
 *
 *     It is the empty string, contains only lowercase characters, or
 *     It can be written as AB (A concatenated with B), where A and B are valid strings, or
 *     It can be written as (A), where A is a valid string.
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "lee(t(c)o)de)"
 * Output: "lee(t(c)o)de"
 * Explanation: "lee(t(co)de)" , "lee(t(c)ode)" would also be accepted.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "a)b(c)d"
 * Output: "ab(c)d"
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "))(("
 * Output: ""
 * Explanation: An empty string is also valid.
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^5
 *     s[i] is either'(' , ')', or lowercase English letter.
 * */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn min_remove_to_make_valid(s: String) -> String {
		let mut st = vec![];
		let mut buf = "".to_owned();
		for c in s.bytes() {
			match c {
				b'(' => {
					buf.push('(');
					st.push(buf.len());
				}
				b')' if st.pop().is_some() => buf.push(')'),
				b')' => {}
				c => buf.push(c as char),
			}
		}
		st.insert(0, 0);
		st.push(buf.len() + 1);
		st.windows(2).map(|w| &buf[w[0]..w[1] - 1]).collect()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1249() {
		assert_eq!(
			Solution::min_remove_to_make_valid("())()(((".to_owned()),
			"()()"
		);
	}
}
