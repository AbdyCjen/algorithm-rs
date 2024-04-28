/**
 * [1544] Make The String Great
 *
 * Given a string s of lower and upper case English letters.
 * A good string is a string which doesn't have two adjacent characters s[i] and s[i + 1] where:
 *
 *     0 <= i <= s.length - 2
 *     s[i] is a lower-case letter and s[i + 1] is the same letter but in upper-case or vice-versa.
 *
 * To make the string good, you can choose two adjacent characters that make the string bad and remove them. You can keep doing this until the string becomes good.
 * Return the string after making it good. The answer is guaranteed to be unique under the given constraints.
 * Notice that an empty string is also good.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "leEeetcode"
 * Output: "leetcode"
 * Explanation: In the first step, either you choose i = 1 or i = 2, both will result "leEeetcode" to be reduced to "leetcode".
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "abBAcC"
 * Output: ""
 * Explanation: We have many possible scenarios, and all lead to the same answer. For example:
 * "abBAcC" --> "aAcC" --> "cC" --> ""
 * "abBAcC" --> "abBA" --> "aA" --> ""
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "s"
 * Output: "s"
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 100
 *     s contains only lower and upper case English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn make_good(s: String) -> String {
		let mut ans = "".to_owned();
		for c in s.bytes() {
			if ans.bytes().last().map(|n| n.abs_diff(c)) == Some(32) {
				ans.pop();
			} else {
				ans.push(c as char);
			}
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1544() {
		assert_eq!(
			Solution::make_good("leEeetcode".to_owned()),
			"leetcode".to_owned()
		);
		assert_eq!(Solution::make_good("abBAcC".to_owned()), "".to_owned());
		assert_eq!(Solution::make_good("s".to_owned()), "s".to_owned());
		assert_eq!(
			Solution::make_good("hHcOzoC".to_owned()),
			"cOzoC".to_owned()
		);
	}
}
