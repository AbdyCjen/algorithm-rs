/**
 * [1298] Reverse Substrings Between Each Pair of Parentheses
 *
 * You are given a string s that consists of lower case English letters and brackets.
 * Reverse the strings in each pair of matching parentheses, starting from the innermost one.
 * Your result should not contain any brackets.
 *  
 * Example 1:
 *
 * Input: s = "(abcd)"
 * Output: "dcba"
 *
 * Example 2:
 *
 * Input: s = "(u(love)i)"
 * Output: "iloveu"
 * Explanation: The substring "love" is reversed first, then the whole string is reversed.
 *
 * Example 3:
 *
 * Input: s = "(ed(et(oc))el)"
 * Output: "leetcode"
 * Explanation: First, we reverse the substring "oc", then "etco", and finally, the whole string.
 *
 * Example 4:
 *
 * Input: s = "a(bcdefghijkl(mno)p)q"
 * Output: "apmnolkjihgfedcbq"
 *
 *  
 * Constraints:
 *
 *     0 <= s.length <= 2000
 *     s only contains lower case English characters and parentheses.
 *     It's guaranteed that all parentheses are balanced.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn reverse_parentheses(s: String) -> String {
		let mut stk = Vec::new();
		let mut ans = Vec::new();
		for c in s.bytes() {
			match c {
				b'(' => stk.push(ans.len()),
				b')' => ans[stk.pop().unwrap()..].reverse(),
				c => ans.push(c),
			}
		}

		String::from_utf8(ans).unwrap()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1190() {
		assert_eq!(
			Solution::reverse_parentheses("(u(love)i)".to_string()),
			"iloveu".to_string()
		);
		assert_eq!(
			Solution::reverse_parentheses("(abcd)".to_string()),
			"dcba".to_string()
		);
		assert_eq!(
			Solution::reverse_parentheses("(ed(et(oc))el)".to_string()),
			"leetcode".to_string()
		);
		assert_eq!(
			Solution::reverse_parentheses("a(bcdefghijkl(mno)p)q".to_string()),
			"apmnolkjihgfedcbq".to_string()
		);
	}
}
